//! WASAPI loopback capture (Windows).
//!
//! * Shared-mode loopback on a *render* endpoint: captures what the user hears,
//!   never a microphone.
//! * Requests IEEE float at the endpoint's own mix rate/channels with
//!   AUTOCONVERTPCM, so integer mix formats are converted by Windows and all
//!   resampling to 16 kHz is done by the shared, tested resampler.
//! * Loopback delivers no packets while nothing is playing; the adapter fills
//!   that time with silence so the segmenter can close a phrase when a video is
//!   paused mid-sentence.
//! * Follows the default output: if it changes, or the device is invalidated
//!   (unplugged, format change), the stream is re-opened and counted.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crossbeam_like::{Receiver, RecvTimeoutError, SyncSender, TrySendError, sync_channel};
use kikoe_core::audio::{AudioFormat, AudioPacket, CaptureError, CaptureStats, CaptureStream, SourceInfo, SourceKind};
use kikoe_core::clock::PipelineClock;
use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Media::Audio::{
    AUDCLNT_BUFFERFLAGS_DATA_DISCONTINUITY, AUDCLNT_BUFFERFLAGS_SILENT, AUDCLNT_E_DEVICE_INVALIDATED,
    AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM, AUDCLNT_STREAMFLAGS_EVENTCALLBACK,
    AUDCLNT_STREAMFLAGS_LOOPBACK, AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY, DEVICE_STATE_ACTIVE, IAudioCaptureClient,
    IAudioClient, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator, WAVEFORMATEX, eConsole, eRender,
};
use windows::Win32::Media::Multimedia::WAVE_FORMAT_IEEE_FLOAT;
use windows::Win32::System::Com::StructuredStorage::{PropVariantClear, PropVariantToStringAlloc};
use windows::Win32::System::Com::{
    CLSCTX_ALL, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, STGM_READ,
};
use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObject};

mod crossbeam_like {
    pub use std::sync::mpsc::{Receiver, RecvTimeoutError, SyncSender, TrySendError, sync_channel};
}

/// ~2.5 s of 10 ms packets; beyond this the adapter drops (and counts) rather than block.
const CHANNEL_PACKETS: usize = 256;
const SILENCE_AFTER: Duration = Duration::from_millis(120);
const DEFAULT_CHECK_EVERY: Duration = Duration::from_secs(2);

/// COM for the current thread. If the thread already has COM in another
/// apartment (e.g. a UI thread in STA, as Tauri's), RPC_E_CHANGED_MODE is not an
/// error for our use: the MMDevice / WASAPI interfaces work in either apartment.
/// We only uninitialise what we initialised.
struct ComGuard {
    owned: bool,
}
impl ComGuard {
    fn new() -> Result<Self, CaptureError> {
        const RPC_E_CHANGED_MODE: windows::core::HRESULT = windows::core::HRESULT(0x8001_0106_u32 as i32);
        let hr = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
        if hr == RPC_E_CHANGED_MODE {
            return Ok(ComGuard { owned: false });
        }
        if hr.is_err() {
            return Err(CaptureError::Device(format!("COM initialisation failed: {hr:?}")));
        }
        Ok(ComGuard { owned: true })
    }
}
impl Drop for ComGuard {
    fn drop(&mut self) {
        if self.owned {
            unsafe { CoUninitialize() };
        }
    }
}

fn dev_err(ctx: &str, e: windows::core::Error) -> CaptureError {
    CaptureError::Device(format!("{ctx}: {e}"))
}

fn enumerator() -> Result<IMMDeviceEnumerator, CaptureError> {
    unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) }.map_err(|e| dev_err("audio device enumerator", e))
}

fn device_id(dev: &IMMDevice) -> Result<String, CaptureError> {
    unsafe {
        let p = dev.GetId().map_err(|e| dev_err("device id", e))?;
        let s = p.to_string().unwrap_or_default();
        CoTaskMemFree(Some(p.0 as *const _));
        Ok(s)
    }
}

fn friendly_name(dev: &IMMDevice) -> String {
    unsafe {
        let Ok(store) = dev.OpenPropertyStore(STGM_READ) else { return "Audio output".into() };
        let Ok(mut pv) = store.GetValue(&PKEY_Device_FriendlyName) else { return "Audio output".into() };
        let name = match PropVariantToStringAlloc(&pv) {
            Ok(p) => {
                let s = p.to_string().unwrap_or_else(|_| "Audio output".into());
                CoTaskMemFree(Some(p.0 as *const _));
                s
            }
            Err(_) => "Audio output".into(),
        };
        let _ = PropVariantClear(&mut pv);
        name
    }
}

pub fn list_sources() -> Result<Vec<SourceInfo>, CaptureError> {
    let _com = ComGuard::new()?;
    let en = enumerator()?;
    let default_id = unsafe { en.GetDefaultAudioEndpoint(eRender, eConsole) }.ok().and_then(|d| device_id(&d).ok());
    let coll = unsafe { en.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE) }.map_err(|e| dev_err("enumerating outputs", e))?;
    let n = unsafe { coll.GetCount() }.map_err(|e| dev_err("counting outputs", e))?;
    let mut out = Vec::new();
    for i in 0..n {
        let Ok(dev) = (unsafe { coll.Item(i) }) else { continue };
        let Ok(id) = device_id(&dev) else { continue };
        out.push(SourceInfo {
            is_default: default_id.as_deref() == Some(id.as_str()),
            label: friendly_name(&dev),
            id,
            kind: SourceKind::SystemLoopback,
        });
    }
    out.sort_by_key(|s| !s.is_default);
    Ok(out)
}

struct Session {
    client: IAudioClient,
    capture: IAudioCaptureClient,
    event: HANDLE,
    format: AudioFormat,
    info: SourceInfo,
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            let _ = self.client.Stop();
            let _ = CloseHandle(self.event);
        }
    }
}

fn open_session(en: &IMMDeviceEnumerator, source_id: Option<&str>) -> Result<Session, CaptureError> {
    unsafe {
        let dev = match source_id {
            Some(id) => en.GetDevice(&windows::core::HSTRING::from(id)).map_err(|e| dev_err("selected output not found", e))?,
            None => en.GetDefaultAudioEndpoint(eRender, eConsole).map_err(|_| {
                CaptureError::NoSource("Windows reports no default audio output device".into())
            })?,
        };
        let id = device_id(&dev)?;
        let info = SourceInfo { label: friendly_name(&dev), id, kind: SourceKind::SystemLoopback, is_default: source_id.is_none() };
        let client: IAudioClient = dev.Activate(CLSCTX_ALL, None).map_err(|e| dev_err("activating audio client", e))?;
        let mix = client.GetMixFormat().map_err(|e| dev_err("reading output format", e))?;
        let (rate, channels) = ((*mix).nSamplesPerSec, (*mix).nChannels);
        CoTaskMemFree(Some(mix as *const _));
        let wf = WAVEFORMATEX {
            wFormatTag: WAVE_FORMAT_IEEE_FLOAT as u16,
            nChannels: channels,
            nSamplesPerSec: rate,
            nAvgBytesPerSec: rate * channels as u32 * 4,
            nBlockAlign: channels * 4,
            wBitsPerSample: 32,
            cbSize: 0,
        };
        let flags = AUDCLNT_STREAMFLAGS_LOOPBACK
            | AUDCLNT_STREAMFLAGS_EVENTCALLBACK
            | AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM
            | AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY;
        // 200 ms buffer (100 ns units)
        client
            .Initialize(AUDCLNT_SHAREMODE_SHARED, flags, 2_000_000, 0, &wf, None)
            .map_err(|e| dev_err("initialising loopback capture", e))?;
        let event = CreateEventW(None, false, false, windows::core::PCWSTR::null()).map_err(|e| dev_err("creating capture event", e))?;
        if let Err(e) = client.SetEventHandle(event) {
            let _ = CloseHandle(event);
            return Err(dev_err("registering capture event", e));
        }
        let capture: IAudioCaptureClient = client.GetService().map_err(|e| dev_err("getting capture service", e))?;
        client.Start().map_err(|e| dev_err("starting capture", e))?;
        Ok(Session { client, capture, event, format: AudioFormat { sample_rate: rate, channels }, info })
    }
}

enum Msg {
    Packet(AudioPacket),
    Error(CaptureError),
}

pub struct WasapiLoopback {
    info: SourceInfo,
    rx: Receiver<Msg>,
    stop: Arc<AtomicBool>,
    stats: Arc<Mutex<CaptureStats>>,
    thread: Option<JoinHandle<()>>,
    finished: bool,
}

impl WasapiLoopback {
    pub fn open(source_id: Option<&str>, clock: PipelineClock) -> Result<Self, CaptureError> {
        let (tx, rx) = sync_channel::<Msg>(CHANNEL_PACKETS);
        let (ready_tx, ready_rx) = sync_channel::<Result<SourceInfo, CaptureError>>(1);
        let stop = Arc::new(AtomicBool::new(false));
        let stats = Arc::new(Mutex::new(CaptureStats::default()));
        let (stop2, stats2) = (stop.clone(), stats.clone());
        let source = source_id.map(str::to_owned);
        let thread = std::thread::Builder::new()
            .name("kikoe-wasapi".into())
            .spawn(move || capture_thread(source, clock, tx, ready_tx, stop2, stats2))
            .map_err(|e| CaptureError::Device(e.to_string()))?;
        match ready_rx.recv_timeout(Duration::from_secs(5)) {
            Ok(Ok(info)) => Ok(Self { info, rx, stop, stats, thread: Some(thread), finished: false }),
            Ok(Err(e)) => {
                let _ = thread.join();
                Err(e)
            }
            Err(_) => {
                stop.store(true, Ordering::Release);
                Err(CaptureError::Device("audio device did not start within 5 s".into()))
            }
        }
    }
}

fn capture_thread(
    source: Option<String>,
    clock: PipelineClock,
    tx: SyncSender<Msg>,
    ready: SyncSender<Result<SourceInfo, CaptureError>>,
    stop: Arc<AtomicBool>,
    stats: Arc<Mutex<CaptureStats>>,
) {
    let _com = match ComGuard::new() {
        Ok(g) => g,
        Err(e) => {
            let _ = ready.send(Err(e));
            return;
        }
    };
    let en = match enumerator() {
        Ok(e) => e,
        Err(e) => {
            let _ = ready.send(Err(e));
            return;
        }
    };
    let mut session = match open_session(&en, source.as_deref()) {
        Ok(s) => s,
        Err(e) => {
            let _ = ready.send(Err(e));
            return;
        }
    };
    let _ = ready.send(Ok(session.info.clone()));
    let follow_default = source.is_none();
    let mut seq = 0u64;
    let mut last_data = Instant::now();
    let mut last_default_check = Instant::now();
    let mut pending_discontinuity = false;

    let push = |pkt: AudioPacket, stats: &Mutex<CaptureStats>| -> bool {
        let frames = pkt.frames() as u64;
        match tx.try_send(Msg::Packet(pkt)) {
            Ok(()) => {
                let mut s = stats.lock().unwrap();
                s.packets += 1;
                s.frames += frames;
                true
            }
            Err(TrySendError::Full(_)) => {
                stats.lock().unwrap().dropped_packets += 1;
                true
            }
            Err(TrySendError::Disconnected(_)) => false,
        }
    };

    'outer: while !stop.load(Ordering::Acquire) {
        // Wake on the device event, or every 20 ms to fill silence / watch for stop.
        let _ = unsafe { WaitForSingleObject(session.event, 20) };
        let mut reopen = false;
        loop {
            let size = match unsafe { session.capture.GetNextPacketSize() } {
                Ok(n) => n,
                Err(e) => {
                    reopen = e.code() == AUDCLNT_E_DEVICE_INVALIDATED;
                    if !reopen {
                        let _ = tx.send(Msg::Error(dev_err("reading capture packet", e)));
                        break 'outer;
                    }
                    break;
                }
            };
            if size == 0 {
                break;
            }
            let mut data: *mut u8 = std::ptr::null_mut();
            let mut frames = 0u32;
            let mut flags = 0u32;
            if let Err(e) = unsafe { session.capture.GetBuffer(&mut data, &mut frames, &mut flags, None, None) } {
                reopen = e.code() == AUDCLNT_E_DEVICE_INVALIDATED;
                if !reopen {
                    let _ = tx.send(Msg::Error(dev_err("reading capture buffer", e)));
                    break 'outer;
                }
                break;
            }
            let ch = session.format.channels as usize;
            let n = frames as usize * ch;
            let samples = if flags & (AUDCLNT_BUFFERFLAGS_SILENT.0 as u32) != 0 || data.is_null() {
                vec![0f32; n]
            } else {
                unsafe { std::slice::from_raw_parts(data as *const f32, n) }.to_vec()
            };
            let _ = unsafe { session.capture.ReleaseBuffer(frames) };
            let disc = pending_discontinuity || flags & (AUDCLNT_BUFFERFLAGS_DATA_DISCONTINUITY.0 as u32) != 0;
            if disc {
                stats.lock().unwrap().discontinuities += 1;
            }
            pending_discontinuity = false;
            seq += 1;
            last_data = Instant::now();
            if !push(
                AudioPacket { seq, format: session.format, samples, captured_at_ns: clock.now_ns(), discontinuity: disc },
                &stats,
            ) {
                break 'outer;
            }
        }
        // Nothing playing: keep the stream clock moving with silence.
        let idle = last_data.elapsed();
        if idle >= SILENCE_AFTER {
            let frames = (session.format.sample_rate as f64 * idle.as_secs_f64().min(0.2)) as usize;
            seq += 1;
            last_data = Instant::now();
            let samples = vec![0f32; frames * session.format.channels as usize];
            if !push(
                AudioPacket { seq, format: session.format, samples, captured_at_ns: clock.now_ns(), discontinuity: false },
                &stats,
            ) {
                break 'outer;
            }
        }
        if follow_default && !reopen && last_default_check.elapsed() >= DEFAULT_CHECK_EVERY {
            last_default_check = Instant::now();
            if let Ok(d) = unsafe { en.GetDefaultAudioEndpoint(eRender, eConsole) } {
                if device_id(&d).ok().as_deref() != Some(session.info.id.as_str()) {
                    reopen = true;
                }
            }
        }
        if reopen {
            drop(session);
            // retry until a device is available again or we are stopped
            loop {
                if stop.load(Ordering::Acquire) {
                    break 'outer;
                }
                match open_session(&en, source.as_deref()) {
                    Ok(s) => {
                        session = s;
                        stats.lock().unwrap().reopen_count += 1;
                        pending_discontinuity = true;
                        last_data = Instant::now();
                        break;
                    }
                    Err(_) => std::thread::sleep(Duration::from_millis(500)),
                }
            }
        }
    }
}

impl CaptureStream for WasapiLoopback {
    fn source(&self) -> &SourceInfo {
        &self.info
    }

    fn next_packet(&mut self, timeout: Duration) -> Result<Option<AudioPacket>, CaptureError> {
        if self.finished {
            return Ok(None);
        }
        match self.rx.recv_timeout(timeout) {
            Ok(Msg::Packet(p)) => Ok(Some(p)),
            Ok(Msg::Error(e)) => {
                self.finished = true;
                Err(e)
            }
            Err(RecvTimeoutError::Timeout) => Ok(None),
            Err(RecvTimeoutError::Disconnected) => {
                self.finished = true;
                Ok(None)
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn stats(&self) -> CaptureStats {
        *self.stats.lock().unwrap()
    }

    fn stop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        self.finished = true;
    }
}

impl Drop for WasapiLoopback {
    fn drop(&mut self) {
        self.stop();
    }
}
