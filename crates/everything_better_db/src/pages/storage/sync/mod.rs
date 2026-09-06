//! Synchronization primitives that can be memory-mapped.

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

macro_rules! impl_le_location {
    (
        ImmutableType = $ImmutableLeLocation:ident,
        MutableType = $MutableLeLocation:ident,
        NonAtomicPrimitive = $NonAtomic:ty,
        AtomicPrimitive = $Atomic:ty
    ) => {
        #[repr(transparent)]
        pub struct $ImmutableLeLocation($NonAtomic);

        impl $ImmutableLeLocation {
            pub fn get(&self) -> $NonAtomic {
                <$NonAtomic>::from_le(self.0)
            }
        }

        #[repr(transparent)]
        pub struct $MutableLeLocation($Atomic);

        impl $MutableLeLocation {
            pub fn get(&self) -> $NonAtomic {
                <$NonAtomic>::from_le(self.0.load(Ordering::Relaxed))
            }

            pub fn set(&self, value: $NonAtomic) {
                self.0.store(value.to_le(), Ordering::Relaxed);
            }
        }
    };
}

impl_le_location!(
    ImmutableType = ImmutableU64LeLocation,
    MutableType = MutableU64LeLocation,
    NonAtomicPrimitive = u64,
    AtomicPrimitive = AtomicU64
);

impl_le_location!(
    ImmutableType = ImmutableU32LeLocation,
    MutableType = MutableU32LeLocation,
    NonAtomicPrimitive = u32,
    AtomicPrimitive = AtomicU32
);
