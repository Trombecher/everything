use everything::{
    knowledge::{Knowledge, Statement},
    model::{Atom, NonZeroBigInt, Object, Set, SetApi, TextSet},
    num_bigint::Sign,
};

use crate::{
    StatementChange,
    format::{
        leb128,
        v1::{
            OBJECT_TYPE_ATOM, OBJECT_TYPE_BYTE, OBJECT_TYPE_CHARACTER_1, OBJECT_TYPE_CHARACTER_2,
            OBJECT_TYPE_CHARACTER_3, OBJECT_TYPE_CHARACTER_4, OBJECT_TYPE_CONSTRUCTION,
            OBJECT_TYPE_NEGATIVE_INTEGER, OBJECT_TYPE_POSITIVE_INTEGER, OBJECT_TYPE_SET,
            OBJECT_TYPE_TEXT, OBJECT_TYPE_ZERO, STATEMENT_CHANGE_ADD, STATEMENT_CHANGE_REMOVE,
        },
    },
};

/// Writes the knowledge base content (the version 1 payload) to `out`.
///
/// Everything is written into the base; the change log is empty.
pub(super) fn write_knowledge(knowledge: &Knowledge, out: &mut Vec<u8>) {
    // Reserve the base length and the reserved bytes (32 bytes in total).
    // The base length is filled in once the base has been written.
    let base_length_position = out.len();
    out.extend_from_slice(&[0_u8; 32]);

    let base_position = out.len();

    for (atom, set) in &knowledge.map {
        write_atom(*atom, out);
        write_set_values(set, out);
    }

    let base_length_in_bytes =
        u64::try_from(out.len() - base_position).expect("the base fits in a u64");

    out[base_length_position..base_length_position + 8]
        .copy_from_slice(&base_length_in_bytes.to_le_bytes());
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
        Object::Atom(Atom::ZERO) => out.push(OBJECT_TYPE_ZERO),
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
        Set::Integer(integer) => write_integer(&integer.representing, out),
        Set::Character(character) => write_character(character.representing, out),
        Set::Byte(byte) => {
            out.push(OBJECT_TYPE_BYTE);
            out.push(byte.0);
        }
        Set::Text(text) => {
            out.push(OBJECT_TYPE_TEXT);
            write_text(text, out);
        }
        Set::OneConstruction(_) | Set::Any(_) => {
            out.push(OBJECT_TYPE_SET);
            write_set_values(set, out);
        }
    }
}

/// Writes an integer set: its sign and the magnitude of the represented
/// integer as an unsigned non-zero integer.
fn write_integer(integer: &NonZeroBigInt, out: &mut Vec<u8>) {
    let (sign, magnitude) = integer.get_as_ref().to_bytes_le();

    match sign {
        Sign::Plus => out.push(OBJECT_TYPE_POSITIVE_INTEGER),
        Sign::Minus => out.push(OBJECT_TYPE_NEGATIVE_INTEGER),
        Sign::NoSign => unreachable!("an integer set never represents zero"),
    }

    write_unsigned_non_zero_integer(&magnitude, out);
}

/// Writes an unsigned non-zero integer: its length in bytes (LEB128) followed
/// by its little-endian content.
fn write_unsigned_non_zero_integer(magnitude: &[u8], out: &mut Vec<u8>) {
    leb128::write_leb128_u64(
        u64::try_from(magnitude.len()).expect("the integer fits in a u64"),
        out,
    );
    out.extend_from_slice(magnitude);
}

/// Writes a text set: its length in bytes (LEB128) followed by its UTF-8
/// content.
fn write_text(text: &TextSet, out: &mut Vec<u8>) {
    let content = text.as_str();

    leb128::write_leb128_u64(
        u64::try_from(content.len()).expect("the text fits in a u64"),
        out,
    );
    out.extend_from_slice(content.as_bytes());
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
