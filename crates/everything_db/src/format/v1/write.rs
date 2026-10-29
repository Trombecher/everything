use everything::{
    knowledge::{Knowledge, Statement},
    model::{Atom, Object, Set, SetApi},
};

use crate::{
    StatementChange,
    format::{
        leb128,
        v1::{
            OBJECT_TYPE_ATOM, OBJECT_TYPE_BYTE, OBJECT_TYPE_CHARACTER_1, OBJECT_TYPE_CHARACTER_2,
            OBJECT_TYPE_CHARACTER_3, OBJECT_TYPE_CHARACTER_4, OBJECT_TYPE_CONSTRUCTION,
            OBJECT_TYPE_SET, STATEMENT_CHANGE_ADD, STATEMENT_CHANGE_REMOVE,
        },
    },
};

/// Writes the knowledge base content (the version 1 payload) to `out`.
///
/// Everything is written into the base; the change log is empty.
pub(super) fn write_knowledge(knowledge: &Knowledge, out: &mut Vec<u8>) {
    let mut base = Vec::new();

    for (atom, set) in &knowledge.map {
        write_atom(*atom, &mut base);
        write_set_values(set, &mut base);
    }

    let base_length_in_bytes = u64::try_from(base.len()).expect("the base fits in a u64");

    out.extend_from_slice(&base_length_in_bytes.to_le_bytes());
    // Reserved bytes, currently unused.
    out.extend_from_slice(&[0_u8; 24]);
    out.extend_from_slice(&base);
}

/// Appends the given statement changes to `out` in the change log format.
///
/// This appends to a file that was previously written by [`write_knowledge`]
/// (or by a previous incremental change), and never modifies the base.
pub(super) fn write_changes<'a>(
    changes: impl Iterator<Item = StatementChange<'a>>,
    out: &mut Vec<u8>,
) {
    for change in changes {
        match change {
            StatementChange::Add(statement) => {
                out.push(STATEMENT_CHANGE_ADD);
                write_statement(statement, out);
            }
            StatementChange::Remove(statement) => {
                out.push(STATEMENT_CHANGE_REMOVE);
                write_statement(statement, out);
            }
        }
    }
}

fn write_atom(atom: Atom, out: &mut Vec<u8>) {
    out.extend_from_slice(&atom.0.to_le_bytes());
}

fn write_statement(statement: &Statement, out: &mut Vec<u8>) {
    write_atom(statement.atom, out);
    write_object(&statement.object, out);
}

fn write_object(object: &Object, out: &mut Vec<u8>) {
    match object {
        Object::Atom(atom) => {
            out.push(OBJECT_TYPE_ATOM);
            write_atom(*atom, out);
        }
        Object::Construction(construction) => {
            out.push(OBJECT_TYPE_CONSTRUCTION);
            write_object(&construction.constructor, out);
            write_object(&construction.value, out);
        }
        Object::Set(set) => write_set_object(set, out),
    }
}

/// Writes a set that occurs as an object (a nested set).
fn write_set_object(set: &Set, out: &mut Vec<u8>) {
    match set {
        // The payload of an integer object is not defined by the version
        // 1 reader yet (it is a `todo!` there), so it cannot be inferred
        // from it either.
        Set::Integer(_) => todo!("write the payload of an integer object"),
        Set::Character(character) => write_character(character.representing, out),
        Set::Byte(byte) => {
            out.push(OBJECT_TYPE_BYTE);
            out.push(byte.0);
        }
        Set::OneConstruction(_) | Set::Any(_) => {
            out.push(OBJECT_TYPE_SET);
            write_set_values(set, out);
        }
    }
}

/// Writes the values of a set, prefixed by their count.
fn write_set_values(set: &Set, out: &mut Vec<u8>) {
    leb128::write_leb128_u64(set.size(), out);

    for value in set.clone() {
        write_object(&value, out);
    }
}

/// Writes a character using the smallest of the four character object types.
fn write_character(character: char, out: &mut Vec<u8>) {
    let bytes = u32::from(character).to_le_bytes();

    let significant_byte_count = bytes
        .iter()
        .rposition(|&byte| byte != 0)
        .map_or(1, |index| index + 1);

    match significant_byte_count {
        1 => {
            out.push(OBJECT_TYPE_CHARACTER_1);
            out.push(bytes[0]);
        }
        2 => {
            out.push(OBJECT_TYPE_CHARACTER_2);
            out.extend_from_slice(&bytes[..2]);
        }
        3 => {
            out.push(OBJECT_TYPE_CHARACTER_3);
            out.extend_from_slice(&bytes[..3]);
        }
        4 => {
            out.push(OBJECT_TYPE_CHARACTER_4);
            out.extend_from_slice(&bytes);
        }
        _ => unreachable!("a `u32` has at most four bytes"),
    }
}
