use crate::{
    pages::{
        FreePage, MutablePageIdLocation, PageKind,
        storage::sync::{MutableU32LeLocation, MutableU64LeLocation},
    },
    unsafe_declare_page,
    versions::MutableDatabaseFormatVersionLocation,
};

#[repr(C, align(4096))]
pub struct MetaPage {
    /// The CRC32C of this meta page, excluding itself and
    /// including the magic bytes.
    pub crc32c: MutableU32LeLocation,

    /// Magic bytes (includes the page kind).
    pub magic_bytes: MagicBytes,

    /// The version of the disk format.
    pub version: MutableDatabaseFormatVersionLocation,

    _padding: [u8; 4],

    pub allocator_next_free_page: MutablePageIdLocation<FreePage>,
    pub allocator_pages_initialized: MutableU64LeLocation,

    /// The database revision id.
    pub revision_id: MutableU64LeLocation,

    _padding2: [u8; 4096 - 48],
}

unsafe_declare_page!(MetaPage, PageKind::Meta);

impl MetaPage {
    pub fn init(&self) {
        self.magic_bytes.init();
    }
}

#[repr(transparent)]
pub struct MagicBytes {
    bytes: [MutableU32LeLocation; 3],
}

impl MagicBytes {
    const EXPECTED: [u32; 3] = [
        u32::from_le_bytes(*b"EVER"),
        u32::from_le_bytes(*b"YTHI"),
        u32::from_le_bytes(*b"NGDB"),
    ];

    pub fn init(&self) {
        self.bytes
            .iter()
            .zip(Self::EXPECTED.iter().copied())
            .for_each(|(location, expected)| location.set(expected));
    }

    pub fn validate(&self) -> Result<(), ()> {
        self.bytes
            .iter()
            .zip(Self::EXPECTED.iter().copied())
            .all(|(location, expected)| location.get() == expected)
            .ok_or(())
    }
}
