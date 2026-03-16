use std::sync::atomic::{AtomicUsize, Ordering};

pub struct DebugDepthCount(AtomicUsize);

impl DebugDepthCount {
    pub const fn new() -> Self {
        Self(AtomicUsize::new(0))
    }

    pub fn get(&self) -> usize {
        #[cfg(debug_assertions)]
        return self.0.load(Ordering::Relaxed);

        #[cfg(not(debug_assertions))]
        0
    }

    pub fn inc(&self) {
        #[cfg(debug_assertions)]
        self.0
            .update(Ordering::Relaxed, Ordering::Relaxed, |depth| {
                depth.checked_add(1).unwrap()
            });
    }

    pub fn dec(&self) {
        #[cfg(debug_assertions)]
        self.0
            .update(Ordering::Relaxed, Ordering::Relaxed, |depth| {
                depth.checked_sub(1).unwrap()
            });
    }
}
