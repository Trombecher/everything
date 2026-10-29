use everything::{
    knowledge::{Knowledge, Statement},
    model::{Atom, ByteSet, CharacterSet, Construction, Object, Set, TextSet},
    num_bigint::BigUint,
};
use scursor::ReadCursor;

use super::parse_unsigned_non_zero_integer;
use crate::{
    StatementChange,
    format::{
        FileFormatApi, FileFormatApiContract, ParseError,
        v1::{
            FileFormatApiV1, OBJECT_TYPE_ATOM, OBJECT_TYPE_BYTE, OBJECT_TYPE_CONSTRUCTION,
            OBJECT_TYPE_TEXT,
        },
    },
};

/// Parses an integer from `bytes`, returning it together with the number of
/// bytes the cursor consumed.
fn parse_integer(bytes: &[u8]) -> Result<(BigUint, usize), ParseError> {
    let mut cursor = ReadCursor::new(bytes);
    let uint = parse_unsigned_non_zero_integer(&mut cursor)?;

    Ok((uint, cursor.position()))
}

#[test]
fn parses_single_byte_values() {
    for (bytes, value) in [
        (&[1_u8, 0x01][..], 1_u64),
        (&[1, 0x02], 0x02),
        (&[1, 0x2A], 0x2A),
        (&[1, 0xFF], 0xFF),
    ] {
        let (parsed, consumed) = parse_integer(bytes).unwrap();

        assert_eq!(parsed, BigUint::from(value), "bytes {bytes:02X?}");
        assert_eq!(consumed, bytes.len(), "bytes {bytes:02X?}");
    }
}

#[test]
fn parses_multi_byte_values_little_endian() {
    for (bytes, value) in [
        (&[2_u8, 0x00, 0x01][..], 0x0100_u64),
        (&[2, 0x34, 0x12], 0x1234),
        (&[4, 0x78, 0x56, 0x34, 0x12], 0x1234_5678),
        (
            &[8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
            u64::MAX,
        ),
    ] {
        let (parsed, consumed) = parse_integer(bytes).unwrap();

        assert_eq!(parsed, BigUint::from(value), "bytes {bytes:02X?}");
        assert_eq!(consumed, bytes.len(), "bytes {bytes:02X?}");
    }
}

#[test]
fn parses_values_wider_than_a_u64() {
    let mut bytes = vec![9];
    bytes.extend_from_slice(&[0xFF; 9]);

    let (parsed, consumed) = parse_integer(&bytes).unwrap();

    assert_eq!(parsed, (BigUint::from(1_u8) << 72) - BigUint::from(1_u8));
    assert_eq!(consumed, bytes.len());
}

#[test]
fn parses_multi_byte_lengths() {
    // A length of 128, whose LEB128 encoding is two bytes, followed by a value
    // whose only non-zero byte is byte fifteen, holding bit 7.
    let mut value_bytes = vec![0x00; 128];
    value_bytes[15] = 0x80;

    let mut bytes = vec![0x80, 0x01];
    bytes.extend_from_slice(&value_bytes);

    let (parsed, consumed) = parse_integer(&bytes).unwrap();

    assert_eq!(parsed, BigUint::from(1_u8) << 127);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn accepts_non_minimal_length_encodings() {
    // The length 1 is encoded redundantly as two LEB128 bytes.
    let (parsed, consumed) = parse_integer(&[0x81, 0x00, 0x2A]).unwrap();

    assert_eq!(parsed, BigUint::from(0x2A_u8));
    assert_eq!(consumed, 3);
}

#[test]
fn leaves_the_cursor_after_the_integer() {
    // Trailing data must not be swallowed by the parse.
    let (parsed, consumed) = parse_integer(&[0x02, 0x34, 0x12, 0xAA, 0xBB]).unwrap();

    assert_eq!(parsed, BigUint::from(0x1234_u32));
    assert_eq!(consumed, 3);
}

#[test]
fn rejects_zero_length() {
    assert_eq!(
        parse_integer(&[0x00]),
        Err(ParseError::InvalidIntegerEncodingFound0)
    );
}

#[test]
fn rejects_zero_with_a_non_zero_length() {
    for bytes in [
        // A single zero byte.
        &[1_u8, 0x00][..],
        // Redundant zero padding.
        &[3, 0x00, 0x00, 0x00],
        // The length itself is non-minimally encoded, but still zero.
        &[0x80, 0x00],
    ] {
        assert_eq!(
            parse_integer(bytes),
            Err(ParseError::InvalidIntegerEncodingFound0),
            "bytes {bytes:02X?}"
        );
    }
}

#[test]
fn rejects_missing_or_truncated_length() {
    for bytes in [
        // No length at all.
        &[][..],
        // A continuation byte without a following byte.
        &[0x80],
        &[0x80, 0x80],
    ] {
        assert_eq!(
            parse_integer(bytes),
            Err(ParseError::InvalidIntegerLength),
            "bytes {bytes:02X?}"
        );
    }
}

#[test]
fn rejects_lengths_wider_than_a_u64() {
    // Eleven continuation bytes.
    assert_eq!(
        parse_integer(&[0xFF; 10]),
        Err(ParseError::InvalidIntegerLength)
    );

    // The final byte carries bits that would fall out of a u64.
    assert_eq!(
        parse_integer(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x02]),
        Err(ParseError::InvalidIntegerLength)
    );
}

#[test]
fn rejects_content_shorter_than_the_length() {
    for bytes in [
        // The length claims one byte, but none follows.
        &[1_u8][..],
        // The length claims two bytes, but only one follows.
        &[2, 0x01],
        &[3, 0x01, 0x01],
    ] {
        assert_eq!(
            parse_integer(bytes),
            Err(ParseError::ContentTooShortForInteger),
            "bytes {bytes:02X?}"
        );
    }
}

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
fn parses_bytes() {
    let payload = v1_payload_with_object(&[OBJECT_TYPE_BYTE, 0x2A]);

    let knowledge =
        FileFormatApiV1::read_knowledge(&payload).expect("the payload must be parseable");

    assert_eq!(
        knowledge.map.get(&Atom(1)),
        Some(&Set::from([Object::Set(Set::Byte(ByteSet(0x2A)))]))
    );
}

#[test]
fn rejects_truncated_byte() {
    let payload = v1_payload_with_object(&[OBJECT_TYPE_BYTE]);

    assert_eq!(
        parse_error(&payload),
        ParseError::ContentIsTooShortForObject
    );
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

    assert_eq!(parse_error(&payload), ParseError::ContentIsTooShortForText);
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
