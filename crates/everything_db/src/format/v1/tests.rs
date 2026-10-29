use everything::{
    knowledge::{Knowledge, Statement},
    model::{Atom, ByteSet, CharacterSet, Construction, Object, Set, TextSet},
    num_bigint::BigInt,
};

use crate::{
    StatementChange,
    format::{
        FileFormatApi, FileFormatApiContract,
        v1::{
            FileFormatApiV1, OBJECT_TYPE_ATOM, OBJECT_TYPE_NEGATIVE_INTEGER,
            OBJECT_TYPE_POSITIVE_INTEGER, OBJECT_TYPE_ZERO,
        },
    },
};

fn write_and_read(knowledge: &Knowledge) -> Knowledge {
    let mut bytes = Vec::new();

    FileFormatApi::write_knowledge(knowledge, &mut bytes);

    FileFormatApi::read_knowledge(&bytes).expect("the knowledge base must be parseable")
}

fn statement(atom: u128, object: Object) -> Statement {
    Statement {
        atom: Atom(atom),
        object,
    }
}

fn construction(constructor: Object, value: Object) -> Object {
    Construction::new(constructor, value).into()
}

/// Returns the raw bytes of the first object written for a knowledge base that
/// contains a single statement.
fn first_object_bytes(knowledge: &Knowledge) -> Vec<u8> {
    let mut bytes = Vec::new();

    FileFormatApiV1::write_knowledge(knowledge, &mut bytes);

    // The version 1 payload consists of the base length and 24 reserved bytes,
    // followed by the base. The base starts with the statement's atom (16
    // bytes) and the set's value count (one LEB128 byte for a single value).
    bytes[32 + 16 + 1..].to_vec()
}

#[test]
fn round_trips_atoms() {
    let mut knowledge = Knowledge::empty();

    knowledge.add_statement(statement(1, Object::Atom(Atom(2))));
    knowledge.add_statement(statement(1, Object::Atom(Atom(3))));
    knowledge.add_statement(statement(4, Object::Atom(Atom(5))));

    let parsed = write_and_read(&knowledge);

    assert_eq!(parsed.map, knowledge.map);
}

#[test]
fn round_trips_atom_zero() {
    let mut knowledge = Knowledge::empty();

    knowledge.add_statement(statement(1, Object::Atom(Atom::ZERO)));

    let parsed = write_and_read(&knowledge);

    assert_eq!(parsed.map, knowledge.map);
}

#[test]
fn round_trips_integers() {
    for integer in [
        BigInt::from(1),
        BigInt::from(-1),
        BigInt::from(i64::MAX),
        BigInt::from(i64::MIN),
        BigInt::from(u64::MAX) + BigInt::from(1),
        -(BigInt::from(u64::MAX) + BigInt::from(1)),
        BigInt::from(u128::MAX),
        -BigInt::from(u128::MAX),
    ] {
        let mut knowledge = Knowledge::empty();

        knowledge.add_statement(statement(1, Object::new_integer(integer.clone())));

        let parsed = write_and_read(&knowledge);

        assert_eq!(parsed.map, knowledge.map, "integer {integer}");
    }
}

#[test]
fn writes_atom_zero_as_zero_object() {
    let mut knowledge = Knowledge::empty();

    knowledge.add_statement(statement(1, Object::Atom(Atom::ZERO)));

    assert_eq!(first_object_bytes(&knowledge), vec![OBJECT_TYPE_ZERO]);
}

#[test]
fn writes_non_zero_atoms_as_atom_objects() {
    let mut knowledge = Knowledge::empty();

    knowledge.add_statement(statement(1, Object::Atom(Atom(0))));

    let mut expected = vec![OBJECT_TYPE_ATOM];
    expected.extend_from_slice(&0_u128.to_le_bytes());

    assert_eq!(first_object_bytes(&knowledge), expected);
}

#[test]
fn writes_integers_as_integer_objects() {
    for (integer, object_type) in [
        (BigInt::from(1), OBJECT_TYPE_POSITIVE_INTEGER),
        (BigInt::from(-1), OBJECT_TYPE_NEGATIVE_INTEGER),
        (
            BigInt::from(u64::MAX) + BigInt::from(1),
            OBJECT_TYPE_POSITIVE_INTEGER,
        ),
        (BigInt::from(i64::MIN), OBJECT_TYPE_NEGATIVE_INTEGER),
    ] {
        let mut knowledge = Knowledge::empty();

        knowledge.add_statement(statement(1, Object::new_integer(integer.clone())));

        let object_bytes = first_object_bytes(&knowledge);

        assert_eq!(object_bytes[0], object_type, "integer {integer}");
    }
}

#[test]
fn round_trips_empty_knowledge() {
    let knowledge = Knowledge::empty();

    let parsed = write_and_read(&knowledge);

    assert_eq!(parsed.map, knowledge.map);
}

#[test]
fn round_trips_nested_character_sets() {
    for character in ['A', '\u{DF}', '\u{20AC}', '\u{800}', '\u{1F600}'] {
        let mut knowledge = Knowledge::empty();

        knowledge.add_statement(statement(
            1,
            Object::Set(Set::Character(CharacterSet {
                representing: character,
            })),
        ));

        let parsed = write_and_read(&knowledge);

        assert_eq!(parsed.map, knowledge.map, "character {character:?}");
    }
}

#[test]
fn round_trips_nested_text_sets() {
    for text in [
        "a",
        "hello",
        "äöü",
        "€",
        "😀",
        &"x".repeat(200),
        "a longer text with multiple bytes per character: äöü €😀",
    ] {
        let mut knowledge = Knowledge::empty();

        knowledge.add_statement(statement(
            1,
            Object::Set(Set::Text(
                TextSet::new(text.into()).expect("the text is not empty"),
            )),
        ));

        let parsed = write_and_read(&knowledge);

        assert_eq!(parsed.map, knowledge.map, "text {text:?}");
    }
}

#[test]
fn round_trips_nested_byte_sets() {
    for byte in [0x00, 0x2A, 0xFF] {
        let mut knowledge = Knowledge::empty();

        knowledge.add_statement(statement(1, Object::Set(Set::Byte(ByteSet(byte)))));

        let parsed = write_and_read(&knowledge);

        assert_eq!(parsed.map, knowledge.map, "byte {byte:#04X}");
    }
}

#[test]
fn round_trips_constructions() {
    let mut knowledge = Knowledge::empty();

    knowledge.add_statement(statement(
        1,
        construction(Object::Atom(Atom(2)), Object::Atom(Atom(3))),
    ));

    let parsed = write_and_read(&knowledge);

    assert_eq!(parsed.map, knowledge.map);
}

#[test]
fn round_trips_nested_constructions() {
    let mut knowledge = Knowledge::empty();

    // Constructions associate to the right: `1 2 3` is `1 (2 3)`.
    knowledge.add_statement(statement(
        1,
        construction(
            Object::Atom(Atom(2)),
            construction(Object::Atom(Atom(3)), Object::Atom(Atom(4))),
        ),
    ));

    let parsed = write_and_read(&knowledge);

    assert_eq!(parsed.map, knowledge.map);
}

#[test]
fn round_trips_sets_of_constructions() {
    let mut knowledge = Knowledge::empty();

    let one = construction(Object::Atom(Atom(2)), Object::Atom(Atom(3)));
    let another = construction(Object::Atom(Atom(4)), Object::Atom(Atom(5)));

    // A single construction is specialized to a one-construction set, while
    // two different constructions end up in an arbitrary set.
    knowledge.add_statement(statement(1, Object::Set(Set::from([one.clone()]))));
    knowledge.add_statement(statement(6, Object::Set(Set::from([one, another]))));

    let parsed = write_and_read(&knowledge);

    assert_eq!(parsed.map, knowledge.map);
}

#[test]
fn applies_incremental_changes_after_a_full_write() {
    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(statement(1, Object::Atom(Atom(2))));

    let added = statement(3, Object::Atom(Atom(4)));
    let removed = statement(1, Object::Atom(Atom(2)));

    let mut bytes = Vec::new();
    FileFormatApi::write_knowledge(&knowledge, &mut bytes);
    FileFormatApi::change_knowledge_incremental(
        [
            StatementChange::Add(&added),
            StatementChange::Remove(&removed),
        ]
        .into_iter(),
        &mut bytes,
    );

    let mut expected = knowledge.clone();
    expected.add_statement(added);
    expected.remove_statement(&removed);

    let parsed =
        FileFormatApi::read_knowledge(&bytes).expect("the knowledge base must be parseable");

    assert_eq!(parsed.map, expected.map);
}
