use everything::{
    knowledge::{Knowledge, Statement},
    model::{Atom, CharacterSet, Construction, Object, Set, TextSet},
};

use crate::{
    StatementChange,
    format::{FileFormatApi, FileFormatApiContract, ParseError},
};

use super::{FileFormatApiV1, OBJECT_TYPE_ATOM, OBJECT_TYPE_CONSTRUCTION, OBJECT_TYPE_TEXT};

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
fn parses_constructions() {
    let mut object_bytes = vec![OBJECT_TYPE_CONSTRUCTION, OBJECT_TYPE_ATOM];
    object_bytes.extend_from_slice(&2_u128.to_le_bytes());
    object_bytes.push(OBJECT_TYPE_ATOM);
    object_bytes.extend_from_slice(&3_u128.to_le_bytes());

    let payload = v1_payload_with_object(&object_bytes);

    let knowledge =
        FileFormatApiV1::read_knowledge(&payload).expect("the payload must be parseable");

    assert_eq!(
        knowledge.map.get(&Atom(1)),
        Some(&Set::from([construction(
            Object::Atom(Atom(2)),
            Object::Atom(Atom(3)),
        )]))
    );
}

/// Builds a version 1 payload whose base contains a single atom with one
/// object, the raw bytes of which are given.
fn v1_payload_with_object(object_bytes: &[u8]) -> Vec<u8> {
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
    let payload = v1_payload_with_object(&[OBJECT_TYPE_TEXT, 0]);

    assert_eq!(parse_error(&payload), ParseError::EmptyText);
}

#[test]
fn rejects_invalid_text() {
    // A single length byte followed by a byte that is not valid UTF-8.
    let payload = v1_payload_with_object(&[OBJECT_TYPE_TEXT, 1, 0xFF]);

    assert_eq!(parse_error(&payload), ParseError::InvalidText);
}

#[test]
fn rejects_truncated_text() {
    // The length claims five bytes, but only one follows.
    let payload = v1_payload_with_object(&[OBJECT_TYPE_TEXT, 5, b'a']);

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
