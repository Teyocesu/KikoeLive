//! Bounded hand-off queue between pipeline stages.
//!
//! Live subtitles must track *current* playback, so when a stage falls behind the
//! oldest pending item is evicted (and returned to the caller so the drop can be
//! reported) instead of blocking the producer or growing without limit.

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::time::Duration;

pub struct DropOldestQueue<T> {
    inner: Mutex<Inner<T>>,
    cond: Condvar,
    capacity: usize,
}

struct Inner<T> {
    items: VecDeque<T>,
    closed: bool,
    high_water: usize,
}

pub enum Pop<T> {
    Item(T),
    Timeout,
    Closed,
}

impl<T> DropOldestQueue<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            inner: Mutex::new(Inner { items: VecDeque::with_capacity(capacity), closed: false, high_water: 0 }),
            cond: Condvar::new(),
            capacity,
        }
    }

    /// Push an item. Returns the evicted oldest item when the queue was full.
    /// Pushing into a closed queue returns the item itself (nothing is kept).
    pub fn push(&self, item: T) -> Option<T> {
        let mut g = self.inner.lock().unwrap();
        if g.closed {
            return Some(item);
        }
        let evicted = if g.items.len() >= self.capacity { g.items.pop_front() } else { None };
        g.items.push_back(item);
        g.high_water = g.high_water.max(g.items.len());
        drop(g);
        self.cond.notify_one();
        evicted
    }

    pub fn pop_timeout(&self, timeout: Duration) -> Pop<T> {
        let mut g = self.inner.lock().unwrap();
        loop {
            if let Some(item) = g.items.pop_front() {
                return Pop::Item(item);
            }
            if g.closed {
                return Pop::Closed;
            }
            let (ng, res) = self.cond.wait_timeout(g, timeout).unwrap();
            g = ng;
            if res.timed_out() && g.items.is_empty() {
                return if g.closed { Pop::Closed } else { Pop::Timeout };
            }
        }
    }

    /// End of input: producers are refused, consumers drain what is left and then see `Closed`.
    pub fn finish(&self) {
        let mut g = self.inner.lock().unwrap();
        g.closed = true;
        drop(g);
        self.cond.notify_all();
    }

    /// Close the queue: pending items are discarded and returned; consumers wake up.
    pub fn close(&self) -> Vec<T> {
        let mut g = self.inner.lock().unwrap();
        g.closed = true;
        let rest: Vec<T> = g.items.drain(..).collect();
        drop(g);
        self.cond.notify_all();
        rest
    }

    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn high_water(&self) -> usize {
        self.inner.lock().unwrap().high_water
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn evicts_oldest_when_full_and_never_exceeds_capacity() {
        let q = DropOldestQueue::new(2);
        assert!(q.push(1).is_none());
        assert!(q.push(2).is_none());
        assert_eq!(q.push(3), Some(1));
        assert_eq!(q.len(), 2);
        for i in 4..1000 {
            q.push(i);
            assert!(q.len() <= 2);
        }
        assert_eq!(q.high_water(), 2);
        match q.pop_timeout(Duration::from_millis(1)) {
            Pop::Item(v) => assert_eq!(v, 998),
            _ => panic!("expected item"),
        }
    }

    #[test]
    fn close_wakes_blocked_consumer_promptly() {
        let q = Arc::new(DropOldestQueue::<u32>::new(1));
        let q2 = q.clone();
        let t = std::thread::spawn(move || {
            let start = std::time::Instant::now();
            let r = q2.pop_timeout(Duration::from_secs(10));
            (matches!(r, Pop::Closed), start.elapsed())
        });
        std::thread::sleep(Duration::from_millis(30));
        q.close();
        let (closed, waited) = t.join().unwrap();
        assert!(closed);
        assert!(waited < Duration::from_secs(1));
    }

    #[test]
    fn finish_lets_consumer_drain() {
        let q = DropOldestQueue::new(4);
        q.push(1);
        q.push(2);
        q.finish();
        assert_eq!(q.push(3), Some(3));
        assert!(matches!(q.pop_timeout(Duration::from_millis(1)), Pop::Item(1)));
        assert!(matches!(q.pop_timeout(Duration::from_millis(1)), Pop::Item(2)));
        assert!(matches!(q.pop_timeout(Duration::from_millis(1)), Pop::Closed));
    }

    #[test]
    fn push_after_close_is_rejected() {
        let q = DropOldestQueue::new(1);
        q.close();
        assert_eq!(q.push(5), Some(5));
        assert!(q.is_empty());
    }
}
