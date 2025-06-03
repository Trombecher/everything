//! This module handles built-in objects and ids.

use std::num::NonZeroU64;

pub type ObjectId = NonZeroU64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomObjectId(NonZeroU64);

impl CustomObjectId {
    pub const FIRST: Self = unsafe { Self::new_unchecked(NonZeroU64::new(1024).unwrap()) };
    
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(id: NonZeroU64) -> Self {
        Self(id)
    }
}

impl TryFrom<u64> for CustomObjectId {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value < Self::FIRST.0.get() {
            Err(())
        } else {
            // SAFETY: this is safe because the smallest custom object id is not 0.
            Ok(unsafe { Self::new_unchecked(NonZeroU64::new_unchecked(value)) })
        }
    }
}

impl Into<NonZeroU64> for CustomObjectId {
    fn into(self) -> NonZeroU64 {
        self.0
    }
}

/// This module contains all built-in tags.
pub mod tags {
    use std::num::NonZeroU64;

    pub const LAST_READ: NonZeroU64 = NonZeroU64::new(32).unwrap();
    pub const LAST_WRITE: NonZeroU64 = NonZeroU64::new(33).unwrap();
    pub const CREATED: NonZeroU64 = NonZeroU64::new(34).unwrap();
    pub const SHA_256: NonZeroU64 = NonZeroU64::new(35).unwrap();
    pub const OWNER: NonZeroU64 = NonZeroU64::new(36).unwrap();
    // pub const LINK: NonZeroU64 = NonZeroU64::new(37).unwrap();
    pub const SOURCE: NonZeroU64 = NonZeroU64::new(38).unwrap();
    pub const TEMPORARY: NonZeroU64 = NonZeroU64::new(39).unwrap();
    // pub const AUDIO: NonZeroU64 = NonZeroU64::new(40).unwrap();
    pub const AUTHOR: NonZeroU64 = NonZeroU64::new(41).unwrap();
    pub const SIZE: NonZeroU64 = NonZeroU64::new(42).unwrap();
    pub const TAG: NonZeroU64 = NonZeroU64::new(43).unwrap();
    pub const FILE_COUNT: NonZeroU64 = NonZeroU64::new(44).unwrap();
    pub const DIRECTORY_COUNT: NonZeroU64 = NonZeroU64::new(45).unwrap();
    pub const TOTAL_FILE_COUNT: NonZeroU64 = NonZeroU64::new(46).unwrap();
    pub const TOTAL_DIRECTORY_COUNT: NonZeroU64 = NonZeroU64::new(47).unwrap();
    pub const TRASHED: NonZeroU64 = NonZeroU64::new(48).unwrap();
    pub const FILE: NonZeroU64 = NonZeroU64::new(49).unwrap();
    pub const DIRECTORY: NonZeroU64 = NonZeroU64::new(50).unwrap();
    // pub const FAVOURITE: NonZeroU64 = NonZeroU64::new(51).unwrap();
    pub const IMAGE: NonZeroU64 = NonZeroU64::new(52).unwrap();
    pub const IMAGE_WIDTH: NonZeroU64 = NonZeroU64::new(53).unwrap();
    pub const IMAGE_HEIGHT: NonZeroU64 = NonZeroU64::new(54).unwrap();
    pub const IMAGE_BIT_DEPTH: NonZeroU64 = NonZeroU64::new(55).unwrap();
    pub const IMAGE_CAMERA_MAKER: NonZeroU64 = NonZeroU64::new(56).unwrap();
    pub const IMAGE_CAMERA_MODEL: NonZeroU64 = NonZeroU64::new(57).unwrap();
    pub const IMAGE_F_STOP: NonZeroU64 = NonZeroU64::new(58).unwrap();
    pub const IMAGE_EXPOSURE: NonZeroU64 = NonZeroU64::new(59).unwrap();
    pub const IMAGE_ISO: NonZeroU64 = NonZeroU64::new(60).unwrap();
    pub const IMAGE_FOCAL_LENGTH: NonZeroU64 = NonZeroU64::new(61).unwrap();
    pub const TITLE: NonZeroU64 = NonZeroU64::new(62).unwrap();
    pub const PATH: NonZeroU64 = NonZeroU64::new(63).unwrap();
    pub const WORD_COUNT: NonZeroU64 = NonZeroU64::new(64).unwrap();
    // pub const TEXT: NonZeroU64 = NonZeroU64::new(65).unwrap();
    pub const FILE_EXTENSION: NonZeroU64 = NonZeroU64::new(66).unwrap();
    pub const FILE_EXTENSION_INNER: NonZeroU64 = NonZeroU64::new(67).unwrap();
    pub const NAME: NonZeroU64 = NonZeroU64::new(68).unwrap();
    pub const PARENT: NonZeroU64 = NonZeroU64::new(69).unwrap();
    pub const READ_ACCESS: NonZeroU64 = NonZeroU64::new(70).unwrap();
    pub const WRITE_ACCESS: NonZeroU64 = NonZeroU64::new(71).unwrap();
    pub const USER: NonZeroU64 = NonZeroU64::new(72).unwrap();
    pub const GROUP: NonZeroU64 = NonZeroU64::new(73).unwrap();
    pub const TAG_SCHEMA: NonZeroU64 = NonZeroU64::new(74).unwrap();
    pub const TAG_PARENT: NonZeroU64 = NonZeroU64::new(75).unwrap();
    pub const DESCRIPTION: NonZeroU64 = NonZeroU64::new(76).unwrap();
    pub const LANGUAGE: NonZeroU64 = NonZeroU64::new(77).unwrap();
    pub const TAG_CONSTRAINT: NonZeroU64 = NonZeroU64::new(79).unwrap();
    pub const TAG_INFERRED: NonZeroU64 = NonZeroU64::new(80).unwrap();
    pub const TAG_INHERITABLE: NonZeroU64 = NonZeroU64::new(81).unwrap();
    pub const TAG_UNIQUE_ID: NonZeroU64 = NonZeroU64::new(82).unwrap();
    pub const REFERENCE_BASED: NonZeroU64 = NonZeroU64::new(83).unwrap();
    pub const TAG_UNIQUE_VALUE: NonZeroU64 = NonZeroU64::new(84).unwrap();
    pub const REFERENCES: NonZeroU64 = NonZeroU64::new(85).unwrap();
}