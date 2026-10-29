use everything::{
    knowledge::{Knowledge, Statement},
    model::{Atom, CharacterSet, Object, Set, TextSet},
};

use crate::{
    StatementChange,
    format::{FileFormatApi, FileFormatApiContract, ParseError},
};

use super::{FileFormatApiV1, OBJECT_TYPE_TEXT};

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
            Object::Set(Set::Text(TextSet::new(text.into()).expect("the text is not empty"))),
        ));

        let parsed = write_and_read(&knowledge);

        assert_eq!(parsed.map, knowledge.map, "text {text:?}");
    }
}

/// Builds a version 1 payload whose base contains a single atom with one
/// object, the raw bytes of which are given.
fn v1_payload_with_text_object(object_bytes: &[u8]) -> Vec<u8> {
    let mut base = Vec::new();
    base.extend_from_slice(&1_u128.to_le_bytes());
    base.push(1); // The atom's set has one value.
    base.extend_from_slice(object_bytes);

    let mut payload = Vec::new();
    payload.extend_from_slice(
        &u64::try_from(base.len())
            .expect("the base fits in a u64")
            .to_le_bytes(),
    );
    payload.extend_from_slice(&[0_u8; 24]);
    payload.extend_from_slice(&base);

    payload
}

fn parse_error(payload: &[u8]) -> ParseError {
    match FileFormatApiV1::read_knowledge(payload) {
        Ok(_) => panic!("the payload must not be parseable"),
        Err(error) => error,
    }
}

#[test]
fn rejects_empty_text() {
    let payload = v1_payload_with_text_object(&[OBJECT_TYPE_TEXT, 0]);

    assert_eq!(parse_error(&payload), ParseError::EmptyText);
}

#[test]
fn rejects_invalid_text() {
    // A single length byte followed by a byte that is not valid UTF-8.
    let payload = v1_payload_with_text_object(&[OBJECT_TYPE_TEXT, 1, 0xFF]);

    assert_eq!(parse_error(&payload), ParseError::InvalidText);
}

#[test]
fn rejects_truncated_text() {
    // The length claims five bytes, but only one follows.
    let payload = v1_payload_with_text_object(&[OBJECT_TYPE_TEXT, 5, b'a']);

    assert_eq!(
        parse_error(&payload),
        ParseError::ContentIsTooShortForText
    );
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
