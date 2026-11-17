use std::time::Instant;

/// Monotonic clock shared by every stage of one capture session.
/// All timestamps in events are nanoseconds since the session origin, so
/// latencies from different threads are directly comparable.
#[derive(Clone, Copy, Debug)]
pub struct PipelineClock {
    origin: Instant,
}

impl PipelineClock {
    pub fn new() -> Self {
        Self { origin: Instant::now() }
    }

    pub fn now_ns(&self) -> u64 {
        self.to_ns(Instant::now())
    }

    pub fn to_ns(&self, t: Instant) -> u64 {
        t.saturating_duration_since(self.origin).as_nanos() as u64
    }
}

impl Default for PipelineClock {
    fn default() -> Self {
        Self::new()
    }
}

pub fn ns_to_ms(ns: u64) -> f64 {
    ns as f64 / 1_000_000.0
}
