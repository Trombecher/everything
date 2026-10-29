#![forbid(unsafe_code)]

use everything::knowledge::Knowledge;
use num_enum::TryFromPrimitive;
use zerocopy::{FromBytes, Immutable, KnownLayout, little_endian::U32};

use crate::{StatementChange, format::v1::FileFormatApiV1};

mod ext;
mod leb128;
mod v1;

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
    #[error("invalid character code point, found {found:02X}")]
    InvalidCharacter { found: u32 },
    #[error("invalid set value count")]
    InvalidSetValueCount,
    #[error("invalid text length")]
    InvalidTextLength,
    #[error("content is too short for parsing text")]
    ContentIsTooShortForText,
    #[error("text is not valid UTF-8")]
    InvalidText,
    #[error("text is empty")]
    EmptyText,
    #[error("content is too short for statement change")]
    ContentIsTooShortForStatementChange,
    #[error("expected statement change byte, found {found_byte:02X}")]
    InvalidStatementChange { found_byte: u8 },
}

pub const MAGIC_BYTES: [u8; 28] = [
    // U128 LE everything file format atom
    189, 225, 166, 125, 187, 246, 207, 98, 45, 183, 146, 233, 93, 133, 161, 1, b'E', b'V', b'E',
    b'R', b'Y', b'T', b'H', b'I', b'N', b'G', b'K', b'B',
];

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

impl Header {
    fn write(self, out: &mut Vec<u8>) {
        out.extend_from_slice(&MAGIC_BYTES);
        out.extend_from_slice(&(self.version as u32).to_le_bytes());
    }
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

#[cfg(target_pointer_width = "64")]
#[must_use]
pub fn u64_to_usize(u: u64) -> usize {
    #[allow(clippy::cast_possible_truncation)]
    return u as usize;
}

pub struct FileFormatApi;

impl FileFormatApiContract for FileFormatApi {
    fn read_knowledge(bytes: &[u8]) -> Result<Knowledge, ParseError> {
        let file = MappedKnowledgeBaseFile::ref_from_bytes(bytes)
            .map_err(|_| ParseError::ContentIsTooShortForHeader)?;

        let header = file.header.parse()?;

        match header.version {
            Version::V1 => FileFormatApiV1::read_knowledge(&file.rest),
        }
    }

    fn write_knowledge(knowledge: &Knowledge, out: &mut Vec<u8>) {
        // Latest
        Header {
            version: Version::V1,
        }
        .write(out);
        FileFormatApiV1::write_knowledge(knowledge, out);
    }

    fn change_knowledge_incremental<'a>(
        changes: impl Iterator<Item = StatementChange<'a>>,
        out: &mut Vec<u8>,
    ) {
        // Latest
        FileFormatApiV1::change_knowledge_incremental(changes, out);
    }
}

pub trait FileFormatApiContract {
    /// # Errors
    ///
    /// on parse erro
    fn read_knowledge(bytes: &[u8]) -> Result<Knowledge, ParseError>;

    fn write_knowledge(knowledge: &Knowledge, out: &mut Vec<u8>);

    fn change_knowledge_incremental<'a>(
        changes: impl Iterator<Item = StatementChange<'a>>,
        out: &mut Vec<u8>,
    );
}
