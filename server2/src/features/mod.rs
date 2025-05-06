mod tests;

use crate::ff;
use num_enum::TryFromPrimitive;
use std::fmt::{Debug, Formatter};
use std::mem::transmute;
use std::ops::*;

/*#[derive(Copy, Clone, PartialEq)]
struct Flags(u32);

bitflags! {
    impl Flags: u32 {
        const CREATION = ff::features::CREATION;
        const BIN = ff::features::BIN;
        const NAMING = ff::features::NAMING;
        const INT = ff::features::INT;
        const FS = ff::features::FS;
        const FILE_TYPES = ff::features::FILE_TYPES;
        const NODE_COUNT = ff::features::NODE_COUNT;
        const IMAGES = ff::features::IMAGES;
        const FAVOURITES = ff::features::FAVOURITES;
        const TEMPORARY_OBJECTS = ff::features::TEMPORARY_OBJECTS;
        const USERS = ff::features::USERS;
        const REFERENCES = ff::features::REFERENCES;
    }
}*/

#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Features(pub u32);

impl Features {
    /// A bit mask that highlights the bits that are used by the current implementation.
    const USED_BITS: u32 = {
        let mut used: u32 = 0;
        let mut i = 0;

        while i < Feature::ALL.len() {
            used |= Feature::ALL[i].into_mask();
            i += 1;
        }

        used
    };

    #[inline]
    #[must_use]
    pub const fn none() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn has(self, feature: Feature) -> bool {
        match feature {
            Feature::Naming => self.0 & Feature::NAMING_IMPLICATION_MASK != 0,
            Feature::FileSystem => self.0 & Feature::FILE_SYSTEM_IMPLICATION_MASK != 0,
            feature => self.0 & feature.into_mask() != 0,
        }
    }

    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        let mut bits = self.0 & Self::USED_BITS;

        if bits & Feature::FILE_SYSTEM_IMPLICATION_MASK != 0 {
            bits |= Feature::FileSystem.into_mask();
        }

        if bits & Feature::NAMING_IMPLICATION_MASK != 0 {
            bits |= Feature::Naming.into_mask();
        }

        Self(bits)
    }
}

impl IntoIterator for Features {
    type Item = Feature;
    type IntoIter = FeaturesIter;

    fn into_iter(self) -> Self::IntoIter {
        FeaturesIter(self.normalize().0)
    }
}

impl Default for Features {
    fn default() -> Self {
        Self::none()
    }
}

impl Add<Feature> for Features {
    type Output = Self;

    fn add(self, feature: Feature) -> Self::Output {
        Self(self.0 | feature.into_mask())
    }
}

impl AddAssign<Feature> for Features {
    fn add_assign(&mut self, feature: Feature) {
        self.0 |= feature.into_mask();
    }
}

impl Sub<Feature> for Features {
    type Output = Self;

    fn sub(self, feature: Feature) -> Self::Output {
        Self(self.0 & !feature.into_mask())
    }
}

impl SubAssign<Feature> for Features {
    fn sub_assign(&mut self, feature: Feature) {
        self.0 &= !feature.into_mask();
    }
}

impl From<&u32> for &Features {
    fn from(value: &u32) -> Self {
        unsafe { transmute(value) }
    }
}

impl From<&mut u32> for &mut Features {
    fn from(value: &mut u32) -> Self {
        unsafe { transmute(value) }
    }
}

impl Debug for Features {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut set = f.debug_set();

        for feature in self.into_iter() {
            set.entry(&feature);
        }

        set.finish()
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct FeaturesIter(
    u32, // This field is guaranteed to be normalized!
);

impl Iterator for FeaturesIter {
    type Item = Feature;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 != 0 {
            // Get the index of the low one bit.
            let next = self.0.trailing_zeros();

            // Reset the bit in the mask.
            self.0 &= !1_u32.checked_shl(next).unwrap_or(0);

            unsafe {
                // SAFETY: field is normalized
                Some(Feature::try_from(next as u8).unwrap_unchecked())
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Feature {
    Creation = ff::features::CREATION,
    Bin = ff::features::BIN,
    Naming = ff::features::NAMING,
    Internationalization = ff::features::INTERNATIONALIZATION,
    FileSystem = ff::features::FILE_SYSTEM,
    FileTypes = ff::features::FILE_TYPES,
    NodeCount = ff::features::NODE_COUNT,
    Images = ff::features::IMAGES,
    Favourites = ff::features::FAVOURITES,
    TemporaryObjects = ff::features::TEMPORARY_OBJECTS,
    Users = ff::features::USERS,
    References = ff::features::REFERENCES,
}

impl Feature {
    #[inline]
    #[must_use]
    const fn into_mask(self) -> u32 {
        1 << (self as u32)
    }

    const ALL: [Feature; 10] = [
        Feature::Creation,
        Feature::Bin,
        Feature::Naming,
        Feature::Internationalization,
        Feature::FileSystem,
        Feature::FileTypes,
        Feature::NodeCount,
        Feature::Images,
        Feature::Favourites,
        Feature::TemporaryObjects,
    ];

    const fn mask_of(features: &'static [Feature]) -> u32 {
        let mut mask = 0;
        let mut index: usize = 0;
        while index < features.len() {
            mask |= features[index].into_mask();
            index += 1;
        }
        mask
    }

    const NAMING_IMPLICATION_MASK: u32 = Self::mask_of(
        [
            Feature::Naming,
            Feature::Internationalization,
            Feature::FileSystem,
            Feature::FileTypes,
            Feature::NodeCount,
            Feature::Images,
        ]
        .as_slice(),
    );

    const FILE_SYSTEM_IMPLICATION_MASK: u32 = Self::mask_of(
        [
            Feature::FileSystem,
            Feature::FileTypes,
            Feature::NodeCount,
            Feature::Images,
        ]
        .as_slice(),
    );
}
