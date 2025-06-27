//! This module handles invalid format policies.

use std::{
    mem::transmute,
    sync::atomic::{AtomicU8, Ordering},
};

use num_enum::TryFromPrimitive;

/// This struct describes how Everything DB should handle
/// cases where the content in parts of the database file is malformed.
#[derive(Copy, Clone, Debug, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum InvalidFormatPolicy {
    Error = 1,
    FixIfPossible = 2,
}

/// An atomic variant of [InvalidFormatPolicy].
pub struct AtomicInvalidFormatPolicy(AtomicU8);

impl AtomicInvalidFormatPolicy {
    pub fn get(&self) -> InvalidFormatPolicy {
        unsafe { transmute(self.0.load(Ordering::Relaxed)) }
    }

    pub fn set(&self, policy: InvalidFormatPolicy) {
        self.0.store(policy as u8, Ordering::Relaxed);
    }

    /// Validates that the internal [AtomicU8] corresponds to a variant of [InvalidFormatPolicy].
    pub fn is_valid(&self) -> bool {
        InvalidFormatPolicy::try_from(self.0.load(Ordering::SeqCst))
            .is_ok()
    }
}