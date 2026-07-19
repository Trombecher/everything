//! Synchronization primitives that can be memory-mapped.

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

macro_rules! impl_le_location {
    ($LeLocation:ident, $Primitive:ty, $Atomic:ty) => {
        #[repr(transparent)]
        #[derive(::zerocopy::FromBytes)]
        pub struct $LeLocation($Atomic);

        impl $LeLocation {
            pub fn get(&self) -> $Primitive {
                <$Primitive>::from_le(self.0.load(Ordering::Relaxed))
            }

            pub fn set(&self, value: $Primitive) {
                self.0.store(value.to_le(), Ordering::Relaxed);
            }
        }
    };
}

impl_le_location!(U64LeLocation, u64, AtomicU64);
impl_le_location!(U32LeLocation, u32, AtomicU32);
