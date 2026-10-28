use everything::knowledge::Knowledge;
use num_enum::TryFromPrimitive;
use zerocopy::{FromBytes, Immutable, KnownLayout, little_endian::U32};

pub mod v1;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    #[error("invalid magic bytes")]
    InvalidMagicBytes,
    #[error("invalid version")]
    InvalidVersion,
    #[error("content is too short for the header")]
    ContentIsTooShortForHeader,
    #[error("content is too short for version 1 of the file format")]
    ContentIsTooShortForV1,
    #[error("content is too short for parsing an atom")]
    ContentIsTooShortForAtom,
    #[error("content is too short for parsing an object")]
    ContentIsTooShortForObject,
    #[error("invalid object type, found {found:02X}")]
    InvalidObjectType { found: u8 },
}

pub const MAGIC_BYTES: [u8; 28] = *b"EVERYTHING KNOWLEDGE BASE   ";

#[derive(Copy, Clone, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum Version {
    V1 = 1,
}

#[derive(FromBytes, KnownLayout, Immutable)]
#[repr(C)]
struct MappedKnowledgeBaseFile {
    pub header: MappedHeader,
    pub rest: [u8],
}

#[derive(FromBytes, KnownLayout, Immutable)]
#[repr(C)]
struct MappedHeader {
    pub magic_bytes: [u8; 28],
    pub version: U32,
}

impl MappedHeader {
    pub fn parse(&self) -> Result<Header, ParseError> {
        if self.magic_bytes != MAGIC_BYTES {
            return Err(ParseError::InvalidMagicBytes);
        }

        let version =
            Version::try_from(self.version.get()).map_err(|_| ParseError::InvalidVersion)?;

        Ok(Header { version })
    }
}

#[derive(Copy, Clone, Debug)]
struct Header {
    pub version: Version,
}

pub fn parse_knowledge(bytes: &[u8]) -> Result<Knowledge, ParseError> {
    let file = MappedKnowledgeBaseFile::ref_from_bytes(bytes)
        .map_err(|_| ParseError::ContentIsTooShortForHeader)?;

    let header = file.header.parse()?;

    match header.version {
        Version::V1 => v1::parse_knowledge(&file.rest),
    }
}

#[cfg(target_pointer_width = "64")]
pub fn u64_to_usize(u: u64) -> usize {
    #[allow(clippy::cast_possible_truncation)]
    return u as usize;
}
