use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};
use everything::{
    knowledge::{Knowledge, Statement},
    model::{Atom, CharacterSet, Object, Set},
};
use num_enum::TryFromPrimitive;
use zerocopy::{FromBytes, Immutable, KnownLayout, TryFromBytes, little_endian::U64};

use crate::format::{ParseError, u64_to_usize};

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

pub(super) fn parse_knowledge(bytes: &[u8]) -> Result<Knowledge, ParseError> {
    let file: &MappedKnowledgeBaseContent = MappedKnowledgeBaseContent::try_ref_from_bytes(bytes)
        .map_err(|_| ParseError::ContentIsTooShortForV1)?;

    let base_length_in_bytes = file.base_length_in_bytes.get();

    let (base, change_log) = file
        .data
        .split_at_checked(u64_to_usize(base_length_in_bytes))
        .ok_or(ParseError::ContentIsTooShortForV1)?;

    let mut knowledge = Knowledge::empty();

    collect_base(base, &mut knowledge)?;
    apply_changes(change_log, &mut knowledge)?;

    Ok(knowledge)
}

fn parse_atom(cursor: &mut Cursor<&[u8]>) -> Result<Atom, ParseError> {
    let atom_id = cursor
        .read_u128::<LittleEndian>()
        .map_err(|_| ParseError::ContentIsTooShortForAtom)?;

    Ok(Atom(atom_id))
}

fn parse_object(cursor: &mut Cursor<&[u8]>) -> Result<Object, ParseError> {
    let raw_object_type = cursor
        .read_u8()
        .map_err(|_| ParseError::ContentIsTooShortForObject)?;

    let object_type =
        ObjectType::try_from(raw_object_type).map_err(|_| ParseError::InvalidObjectType {
            found: raw_object_type,
        })?;

    match object_type {
        ObjectType::Atom => Ok(Object::Atom(parse_atom(cursor)?)),
        ObjectType::Integer => todo!(),
        ObjectType::Construction => todo!(),
        ObjectType::Character1 => {
            let byte = cursor.read_u8().map_err(|_| todo!())?;
            let character = char::try_from(u32::from(byte)).map_err(|_| todo!())?;

            Ok(Object::Set(Set::Character(CharacterSet {
                representing: character,
            })))
        }
        ObjectType::Character2 => {
            let read_bytes: [u8; 2] = cursor.read_array().map_err(|_| todo!())?;
            let character =
                char::try_from(u32::from_le_bytes([read_bytes[0], read_bytes[1], 0, 0]))
                    .map_err(|_| todo!())?;

            Ok(Object::Set(Set::Character(CharacterSet {
                representing: character,
            })))
        }
        ObjectType::Character3 => {
            let read_bytes: [u8; 3] = cursor.read_array().map_err(|_| todo!())?;
            let character = char::try_from(u32::from_le_bytes([
                read_bytes[0],
                read_bytes[1],
                read_bytes[2],
                0,
            ]))
            .map_err(|_| todo!())?;

            Ok(Object::Set(Set::Character(CharacterSet {
                representing: character,
            })))
        }
        ObjectType::Character4 => {
            let read_bytes: [u8; 4] = cursor.read_array().map_err(|_| todo!())?;
            let character = char::try_from(u32::from_le_bytes(read_bytes)).map_err(|_| todo!())?;

            Ok(Object::Set(Set::Character(CharacterSet {
                representing: character,
            })))
        }
        ObjectType::Text => todo!(),
        ObjectType::Binary => todo!(),
        ObjectType::Set => todo!(),
        ObjectType::Byte => todo!(),
    }
}

fn parse_statement(cursor: &mut Cursor<&[u8]>) -> Result<Statement, ParseError> {
    let atom = parse_atom(cursor)?;
    let is_object = parse_object(cursor)?;

    Ok(Statement { atom, is_object })
}

fn collect_base(base: &[u8], knowledge: &mut Knowledge) -> Result<(), ParseError> {
    let mut cursor = Cursor::new(base);

    while cursor.position() < cursor.get_ref().len() as u64 {
        let atom = parse_atom(&mut cursor)?;
    }

    todo!()
}

fn apply_changes(change_log: &[u8], knowledge: &mut Knowledge) -> Result<(), ParseError> {
    todo!()
}
