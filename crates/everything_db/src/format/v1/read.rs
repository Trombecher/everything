use everything::{
    knowledge::{Knowledge, Statement},
    model::{Atom, CharacterSet, Object, Set, SetApi},
};
use scursor::ReadCursor;
use zerocopy::TryFromBytes;

use crate::format::{
    ParseError,
    ext::ReadCursorExt,
    u64_to_usize,
    v1::{MappedKnowledgeBaseContent, ObjectType, StatementChangeKind},
};

pub fn parse_knowledge(bytes: &[u8]) -> Result<Knowledge, ParseError> {
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

fn parse_atom(cursor: &mut ReadCursor) -> Result<Atom, ParseError> {
    let atom_id = cursor
        .read_u128_le()
        .map_err(|_| ParseError::ContentIsTooShortForAtom)?;

    Ok(Atom(atom_id))
}

fn parse_object(cursor: &mut ReadCursor) -> Result<Object, ParseError> {
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
            let byte = cursor
                .read_u8()
                .map_err(|_| ParseError::ContentIsTooShortForObject)?;

            let character =
                char::try_from(u32::from(byte)).map_err(|_| ParseError::InvalidCharacter {
                    found: u32::from(byte),
                })?;

            Ok(Object::Set(Set::Character(CharacterSet {
                representing: character,
            })))
        }
        ObjectType::Character2 => {
            let read_bytes: [u8; 2] = cursor
                .read_array()
                .map_err(|_| ParseError::ContentIsTooShortForObject)?;

            let code_point = u32::from_le_bytes([read_bytes[0], read_bytes[1], 0, 0]);

            let character = char::try_from(code_point)
                .map_err(|_| ParseError::InvalidCharacter { found: code_point })?;

            Ok(Object::Set(Set::Character(CharacterSet {
                representing: character,
            })))
        }
        ObjectType::Character3 => {
            let read_bytes: [u8; 3] = cursor
                .read_array()
                .map_err(|_| ParseError::ContentIsTooShortForObject)?;

            let code_point = u32::from_le_bytes([read_bytes[0], read_bytes[1], read_bytes[2], 0]);

            let character = char::try_from(code_point)
                .map_err(|_| ParseError::InvalidCharacter { found: code_point })?;

            Ok(Object::Set(Set::Character(CharacterSet {
                representing: character,
            })))
        }
        ObjectType::Character4 => {
            let read_bytes: [u8; 4] = cursor
                .read_array()
                .map_err(|_| ParseError::ContentIsTooShortForObject)?;

            let code_point = u32::from_le_bytes(read_bytes);

            let character = char::try_from(code_point)
                .map_err(|_| ParseError::InvalidCharacter { found: code_point })?;

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

fn parse_statement(cursor: &mut ReadCursor) -> Result<Statement, ParseError> {
    let atom = parse_atom(cursor)?;
    let is_object = parse_object(cursor)?;

    Ok(Statement {
        atom,
        object: is_object,
    })
}

fn parse_set(cursor: &mut ReadCursor) -> Result<Set, ParseError> {
    let value_count = cursor
        .read_leb128_u64()
        .map_err(|_| ParseError::InvalidSetValueCount)?;

    let mut properties = Vec::new();

    for _ in 0..value_count {
        properties.push(parse_object(cursor)?);
    }

    Ok(Set::from(properties.as_mut_slice()))
}

fn collect_base(base: &[u8], knowledge: &mut Knowledge) -> Result<(), ParseError> {
    let mut cursor = ReadCursor::new(base);

    while cursor.remaining() > 0 {
        let atom = parse_atom(&mut cursor)?;
        let set = parse_set(&mut cursor)?;

        if set.size() > 0 {
            knowledge.map.insert(atom, set);
        }
    }

    Ok(())
}

fn apply_changes(change_log: &[u8], knowledge: &mut Knowledge) -> Result<(), ParseError> {
    let mut cursor = ReadCursor::new(change_log);

    while cursor.remaining() > 0 {
        let statement_change = cursor
            .read_u8()
            .map_err(|_| ParseError::ContentIsTooShortForStatementChange)
            .and_then(|raw_statement_change| {
                StatementChangeKind::try_from(raw_statement_change).map_err(|_| {
                    ParseError::InvalidStatementChange {
                        found_byte: raw_statement_change,
                    }
                })
            })?;

        let statement = parse_statement(&mut cursor)?;

        match statement_change {
            StatementChangeKind::Add => knowledge.add_statement(statement),
            StatementChangeKind::Remove => knowledge.remove_statement(&statement),
        }
    }

    Ok(())
}
