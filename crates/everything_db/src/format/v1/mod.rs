mod read;
#[cfg(test)]
mod tests;
mod write;

use everything::knowledge::Knowledge;
use num_enum::TryFromPrimitive;
use zerocopy::{FromBytes, Immutable, KnownLayout, little_endian::U64};

use crate::format::{FileFormatApiContract, ParseError};

// Wire discriminants. Reading and writing must agree on these, so they
// are defined once here and referenced by both the enums and the writer.

const STATEMENT_CHANGE_ADD: u8 = 1;
const STATEMENT_CHANGE_REMOVE: u8 = 2;

const OBJECT_TYPE_ATOM: u8 = 1;
const OBJECT_TYPE_ZERO: u8 = 2;
const OBJECT_TYPE_CONSTRUCTION: u8 = 3;
const OBJECT_TYPE_CHARACTER_1: u8 = 4;
const OBJECT_TYPE_CHARACTER_2: u8 = 5;
const OBJECT_TYPE_CHARACTER_3: u8 = 6;
const OBJECT_TYPE_CHARACTER_4: u8 = 7;
const OBJECT_TYPE_TEXT: u8 = 8;
const OBJECT_TYPE_BINARY: u8 = 9;
const OBJECT_TYPE_SET: u8 = 10;
const OBJECT_TYPE_BYTE: u8 = 11;
const OBJECT_TYPE_POSITIVE_INTEGER: u8 = 12;
const OBJECT_TYPE_NEGATIVE_INTEGER: u8 = 13;

#[derive(Copy, Clone, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u8)]
enum StatementChangeKind {
    Add = STATEMENT_CHANGE_ADD,
    Remove = STATEMENT_CHANGE_REMOVE,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u8)]
enum ObjectType {
    Atom = OBJECT_TYPE_ATOM,
    Zero = OBJECT_TYPE_ZERO,
    PositiveInteger = OBJECT_TYPE_POSITIVE_INTEGER,
    NegativeInteger = OBJECT_TYPE_NEGATIVE_INTEGER,
    Construction = OBJECT_TYPE_CONSTRUCTION,
    Character1 = OBJECT_TYPE_CHARACTER_1,
    Character2 = OBJECT_TYPE_CHARACTER_2,
    Character3 = OBJECT_TYPE_CHARACTER_3,
    Character4 = OBJECT_TYPE_CHARACTER_4,
    Text = OBJECT_TYPE_TEXT,
    Binary = OBJECT_TYPE_BINARY,
    Set = OBJECT_TYPE_SET,
    Byte = OBJECT_TYPE_BYTE,
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
        write::write_knowledge(knowledge, out);
    }

    fn change_knowledge_incremental<'a>(
        changes: impl Iterator<Item = crate::StatementChange<'a>>,
        out: &mut Vec<u8>,
    ) {
        // Write changes into out.
        write::write_changes(changes, out);
    }
}
