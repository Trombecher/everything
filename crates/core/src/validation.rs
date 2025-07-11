//! Validation id primitives.

use std::mem::forget;
use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

pub type ValidationId = NonZeroU64;

/// A storage location for locking and validating other resources.
///
/// Implemented using one atomic `u64`.
pub struct ValidationIdStore(AtomicU64);

impl ValidationIdStore {
    #[inline]
    pub fn init(&mut self, vid: ValidationId) {
        self.0.store(vid.get(), Ordering::SeqCst);
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const u64 {
        self.0.as_ptr()
    }

    #[inline]
    pub fn status(&self, check_vid: ValidationId) -> ValidationIdStatus {
        let mut current = self.0.swap(0, Ordering::SeqCst);

        if current == 0 {
            // This means another thread is currently validating.
            // We wait, it should not take long. This
            // scenario is rare.

            loop {
                current = self.0.load(Ordering::SeqCst);

                if current != 0 {
                    break;
                }
            }
        }

        if current == check_vid.get() {
            // Swap the validation id back in.
            self.0.swap(current, Ordering::SeqCst);

            ValidationIdStatus::Validated
        } else {
            // The caller needs to validate that resource.
            ValidationIdStatus::NotValidated(ValidationIdLockGuard {
                store: self,
                vid: ValidationId::new(current).unwrap(),
            })
        }
    }
}

pub enum ValidationIdStatus<'a> {
    /// This means the resource is already validated.
    Validated,
    /// This means the caller needs to validate the resource.
    NotValidated(ValidationIdLockGuard<'a>),
}

pub struct ValidationIdLockGuard<'a> {
    store: &'a ValidationIdStore,
    vid: ValidationId,
}

impl<'a> ValidationIdLockGuard<'a> {
    /// You should call this function if you encountered an error
    /// while validating the resource. This saves one atomic
    /// store operation.
    pub fn discard(self) {
        forget(self)
    }
}

impl Drop for ValidationIdLockGuard<'_> {
    fn drop(&mut self) {
        self.store.0.store(self.vid.get(), Ordering::SeqCst);
    }
}
