mod read;

use everything::knowledge::Knowledge;
use num_enum::TryFromPrimitive;
use zerocopy::{FromBytes, Immutable, KnownLayout, little_endian::U64};

use crate::format::{FileFormatApiContract, ParseError};

#[derive(Copy, Clone, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u8)]
enum StatementChangeKind {
    Add = 1,
    Remove = 2,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u8)]
enum ObjectType {
    Atom = 1,
    Integer = 2,
    Construction = 3,
    Character1 = 4,
    Character2 = 5,
    Character3 = 6,
    Character4 = 7,
    Text = 8,
    Binary = 9,
    Set = 10,
    Byte = 11,
}

#[derive(FromBytes, Immutable, KnownLayout)]
#[repr(C)]
struct MappedKnowledgeBaseContent {
    base_length_in_bytes: U64,
    _reserved: [u8; 24],
    data: [u8],
}

pub struct FileFormatApiV1;

impl FileFormatApiContract for FileFormatApiV1 {
    fn read_knowledge(bytes: &[u8]) -> Result<Knowledge, ParseError> {
        read::parse_knowledge(bytes)
    }

    fn write_knowledge(knowledge: &Knowledge, out: &mut Vec<u8>) {
        // Write knowledge here, all into base, no changes.
        todo!()
    }

    fn change_knowledge_incremental<'a>(
        changes: impl Iterator<Item = crate::StatementChange<'a>>,
        out: &mut Vec<u8>,
    ) {
        // Write changes into out.
        todo!()
    }
}
