//! Streaming windowed-sinc resampler (mono f32), used to bring any device rate
//! (44.1/48/96 kHz...) to the 16 kHz Whisper and Silero expect.
//!
//! Kept dependency-free on purpose: it is small, deterministic and identical on
//! both platforms. The low-pass cutoff sits just below the output Nyquist so that
//! game music / SFX above 8 kHz does not alias into the speech band.

use std::f64::consts::PI;

const PHASES: usize = 256;
/// Zero crossings of the sinc kernel on each side (in output-rate units).
const ZERO_CROSSINGS: usize = 16;

pub struct Resampler {
    in_rate: u32,
    out_rate: u32,
    step: f64,
    half_taps: usize,
    table: Vec<f32>, // PHASES+1 rows of 2*half_taps taps
    buf: Vec<f32>,
    pos: f64,
}

impl Resampler {
    pub fn new(in_rate: u32, out_rate: u32) -> Self {
        assert!(in_rate > 0 && out_rate > 0);
        let step = in_rate as f64 / out_rate as f64;
        // Fraction of the input Nyquist that we keep.
        let cutoff = if out_rate < in_rate { (out_rate as f64 / in_rate as f64) * 0.94 } else { 0.94 };
        let half_taps = ((ZERO_CROSSINGS as f64) / cutoff).ceil() as usize;
        let taps = 2 * half_taps;
        let mut table = vec![0f32; (PHASES + 1) * taps];
        for p in 0..=PHASES {
            let frac = p as f64 / PHASES as f64;
            let mut row_sum = 0.0;
            for j in 0..taps {
                // distance (in input samples) between tap j and the output instant
                let x = (j as f64 - (half_taps as f64 - 1.0)) - frac;
                let v = cutoff * sinc(cutoff * x) * blackman(x, half_taps as f64);
                table[p * taps + j] = v as f32;
                row_sum += v;
            }
            // normalise DC gain to exactly 1
            for j in 0..taps {
                table[p * taps + j] = (table[p * taps + j] as f64 / row_sum) as f32;
            }
        }
        Self {
            in_rate,
            out_rate,
            step,
            half_taps,
            table,
            // prime with zeros so the first output has full left context
            buf: vec![0.0; half_taps],
            pos: (half_taps - 1) as f64,
        }
    }

    pub fn in_rate(&self) -> u32 {
        self.in_rate
    }

    pub fn out_rate(&self) -> u32 {
        self.out_rate
    }

    /// Feed input samples, append every output sample that is now computable.
    pub fn process(&mut self, input: &[f32], out: &mut Vec<f32>) {
        if self.in_rate == self.out_rate {
            out.extend_from_slice(input);
            return;
        }
        self.buf.extend_from_slice(input);
        let taps = 2 * self.half_taps;
        loop {
            let i0 = self.pos.floor() as usize;
            if i0 + self.half_taps >= self.buf.len() {
                break;
            }
            let frac = self.pos - i0 as f64;
            let pf = frac * PHASES as f64;
            let p = pf.floor() as usize;
            let w = (pf - p as f64) as f32;
            let start = i0 + 1 - self.half_taps;
            let src = &self.buf[start..start + taps];
            let r0 = &self.table[p * taps..(p + 1) * taps];
            let r1 = &self.table[(p + 1) * taps..(p + 2) * taps];
            let mut acc0 = 0f32;
            let mut acc1 = 0f32;
            for j in 0..taps {
                acc0 += src[j] * r0[j];
                acc1 += src[j] * r1[j];
            }
            out.push(acc0 + (acc1 - acc0) * w);
            self.pos += self.step;
        }
        // discard samples that can no longer contribute
        let keep_from = (self.pos.floor() as usize + 1).saturating_sub(self.half_taps);
        if keep_from > 0 {
            self.buf.drain(..keep_from);
            self.pos -= keep_from as f64;
        }
    }

    /// Flush the tail (pads with zeros), used at end of file.
    pub fn finish(&mut self, out: &mut Vec<f32>) {
        let pad = vec![0f32; self.half_taps * 2 + 1];
        self.process(&pad, out);
    }
}

fn sinc(x: f64) -> f64 {
    if x.abs() < 1e-9 { 1.0 } else { (PI * x).sin() / (PI * x) }
}

fn blackman(x: f64, half: f64) -> f64 {
    let n = (x + half) / (2.0 * half);
    if !(0.0..=1.0).contains(&n) {
        return 0.0;
    }
    0.42 - 0.5 * (2.0 * PI * n).cos() + 0.08 * (4.0 * PI * n).cos()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(rate: u32, freq: f64, secs: f64) -> Vec<f32> {
        (0..(rate as f64 * secs) as usize)
            .map(|i| (2.0 * PI * freq * i as f64 / rate as f64).sin() as f32 * 0.5)
            .collect()
    }

    fn amplitude_at(signal: &[f32], rate: u32, freq: f64) -> f64 {
        // single-bin DFT magnitude, normalised to sine amplitude
        let (mut re, mut im) = (0.0, 0.0);
        for (i, s) in signal.iter().enumerate() {
            let ph = 2.0 * PI * freq * i as f64 / rate as f64;
            re += *s as f64 * ph.cos();
            im -= *s as f64 * ph.sin();
        }
        2.0 * (re * re + im * im).sqrt() / signal.len() as f64
    }

    fn run_chunked(r: &mut Resampler, input: &[f32], chunk: usize) -> Vec<f32> {
        let mut out = Vec::new();
        for c in input.chunks(chunk) {
            r.process(c, &mut out);
        }
        out
    }

    #[test]
    fn preserves_speech_band_tone_48k() {
        let input = tone(48_000, 1_000.0, 1.0);
        let mut r = Resampler::new(48_000, 16_000);
        let out = run_chunked(&mut r, &input, 480);
        assert!((out.len() as i64 - 16_000).abs() < 64, "len {}", out.len());
        let a = amplitude_at(&out[200..out.len() - 200], 16_000, 1_000.0);
        assert!((a - 0.5).abs() < 0.02, "amplitude {a}");
    }

    #[test]
    fn handles_44100_fractional_ratio() {
        let input = tone(44_100, 440.0, 2.0);
        let mut r = Resampler::new(44_100, 16_000);
        let out = run_chunked(&mut r, &input, 441);
        assert!((out.len() as i64 - 32_000).abs() < 64, "len {}", out.len());
        let a = amplitude_at(&out[500..out.len() - 500], 16_000, 440.0);
        assert!((a - 0.5).abs() < 0.02, "amplitude {a}");
    }

    #[test]
    fn rejects_content_above_output_nyquist() {
        // 12 kHz would alias to 4 kHz at 16 kHz output without filtering.
        let input = tone(48_000, 12_000.0, 1.0);
        let mut r = Resampler::new(48_000, 16_000);
        let out = run_chunked(&mut r, &input, 1024);
        let alias = amplitude_at(&out[200..out.len() - 200], 16_000, 4_000.0);
        assert!(alias < 0.005, "alias amplitude {alias}");
    }

    #[test]
    fn chunking_does_not_change_output() {
        let input = tone(48_000, 700.0, 0.5);
        let mut a = Resampler::new(48_000, 16_000);
        let mut b = Resampler::new(48_000, 16_000);
        let oa = run_chunked(&mut a, &input, 7);
        let ob = run_chunked(&mut b, &input, 4096);
        assert_eq!(oa.len(), ob.len());
        for (x, y) in oa.iter().zip(ob.iter()) {
            assert!((x - y).abs() < 1e-5);
        }
    }

    #[test]
    fn passthrough_when_rates_match() {
        let mut r = Resampler::new(16_000, 16_000);
        let mut out = Vec::new();
        r.process(&[0.1, 0.2, 0.3], &mut out);
        assert_eq!(out, vec![0.1, 0.2, 0.3]);
    }
}
