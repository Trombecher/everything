mod read;

use num_enum::TryFromPrimitive;
use zerocopy::{FromBytes, Immutable, KnownLayout, little_endian::U64};

#[derive(Copy, Clone, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum StatementChange {
    Add = 1,
    Remove = 2,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum ObjectType {
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

pub(super) use read::parse_knowledge;
