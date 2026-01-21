use std::sync::atomic::AtomicU32;

#[repr(transparent)]
pub struct AtomicU32Le(AtomicU32);

impl AtomicU32Le {}
