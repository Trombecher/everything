use everything::num_bigint::BigUint;
use scursor::ReadCursor;

use super::parse_unsigned_non_zero_integer;
use crate::format::ParseError;

/// Parses an integer from `bytes`, returning it together with the number of
/// bytes the cursor consumed.
fn parse(bytes: &[u8]) -> Result<(BigUint, usize), ParseError> {
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
        let (parsed, consumed) = parse(bytes).unwrap();

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
        let (parsed, consumed) = parse(bytes).unwrap();

        assert_eq!(parsed, BigUint::from(value), "bytes {bytes:02X?}");
        assert_eq!(consumed, bytes.len(), "bytes {bytes:02X?}");
    }
}

#[test]
fn parses_values_wider_than_a_u64() {
    let mut bytes = vec![9];
    bytes.extend_from_slice(&[0xFF; 9]);

    let (parsed, consumed) = parse(&bytes).unwrap();

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

    let (parsed, consumed) = parse(&bytes).unwrap();

    assert_eq!(parsed, BigUint::from(1_u8) << 127);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn accepts_non_minimal_length_encodings() {
    // The length 1 is encoded redundantly as two LEB128 bytes.
    let (parsed, consumed) = parse(&[0x81, 0x00, 0x2A]).unwrap();

    assert_eq!(parsed, BigUint::from(0x2A_u8));
    assert_eq!(consumed, 3);
}

#[test]
fn leaves_the_cursor_after_the_integer() {
    // Trailing data must not be swallowed by the parse.
    let (parsed, consumed) = parse(&[0x02, 0x34, 0x12, 0xAA, 0xBB]).unwrap();

    assert_eq!(parsed, BigUint::from(0x1234_u32));
    assert_eq!(consumed, 3);
}

#[test]
fn rejects_zero_length() {
    assert_eq!(
        parse(&[0x00]),
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
            parse(bytes),
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
            parse(bytes),
            Err(ParseError::InvalidIntegerLength),
            "bytes {bytes:02X?}"
        );
    }
}

#[test]
fn rejects_lengths_wider_than_a_u64() {
    // Eleven continuation bytes.
    assert_eq!(parse(&[0xFF; 10]), Err(ParseError::InvalidIntegerLength));

    // The final byte carries bits that would fall out of a u64.
    assert_eq!(
        parse(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x02]),
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
            parse(bytes),
            Err(ParseError::ContentTooShortForInteger),
            "bytes {bytes:02X?}"
        );
    }
}
