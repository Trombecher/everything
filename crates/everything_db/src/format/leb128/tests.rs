use super::*;

fn decode(bytes: &[u8]) -> Result<(u64, usize), Leb128U64ParsingError> {
    let mut cursor = ReadCursor::new(bytes);
    let value = read_leb128_u64(&mut cursor)?;
    Ok((value, cursor.position()))
}

#[test]
fn one_byte_values() {
    for (bytes, value) in [
        (&[0x00][..], 0x00),
        (&[0x01][..], 0x01),
        (&[0x2A][..], 0x2A),
        (&[0x7F][..], 0x7F),
    ] {
        let (decoded, length) = decode(bytes).unwrap();

        assert_eq!(decoded, value);
        assert_eq!(length, 1);
    }
}

#[test]
fn multi_byte_values() {
    for (bytes, value) in [
        (&[0x80, 0x01][..], 128),
        (&[0x81, 0x01][..], 129),
        (&[0xAC, 0x02][..], 300),
        // A well-known LEB128 example.
        (&[0xE5, 0x8E, 0x26][..], 624_485),
        (&[0x80, 0x80, 0x80, 0x20][..], 0x0400_0000),
    ] {
        let (decoded, length) = decode(bytes).unwrap();

        assert_eq!(decoded, value);
        assert_eq!(length, bytes.len());
    }
}

#[test]
fn maximal_value_encodes_in_ten_bytes() {
    let (decoded, length) =
        decode(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]).unwrap();

    assert_eq!(decoded, u64::MAX);
    assert_eq!(length, 10);
}

#[test]
fn non_minimal_encodings_are_accepted() {
    // A zero encoded redundantly.
    let (decoded, length) = decode(&[0x80, 0x00]).unwrap();
    assert_eq!((decoded, length), (0, 2));

    // One, likewise.
    let (decoded, _) = decode(&[0x81, 0x80, 0x80, 0x00]).unwrap();
    assert_eq!(decoded, 1);
}

#[test]
fn truncated_encodings_error() {
    for bytes in [
        (&[][..]),
        (&[0x80][..]),
        (&[0x80, 0x80][..]),
        (&[0xFF, 0xFF][..]),
    ] {
        assert!(decode(bytes).is_err());
    }
}

#[test]
fn encodings_wider_than_u64_error() {
    // Eleventh continuation byte.
    assert!(decode(&[0xFF; 10]).is_err());

    // High bits falling out of a u64 in the final byte.
    assert!(decode(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x02]).is_err());

    // Value would exceed bit 63 of the final byte.
    assert!(decode(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]).is_err());
}

#[test]
fn writing_round_trips_through_reading() {
    for value in [0, 1, 0x7F, 128, 300, 624_485, u64::MAX] {
        let mut bytes = Vec::new();

        write_leb128_u64(value, &mut bytes);

        let (decoded, length) = decode(&bytes).unwrap();

        assert_eq!(decoded, value);
        assert_eq!(length, bytes.len(), "value {value}");
    }
}

#[test]
fn cursor_ends_after_the_encoding() {
    // Trailing data must not be swallowed by the decode.
    let mut cursor = ReadCursor::new(&[0xAC, 0x02, 0xAA, 0xBB]);

    let value = read_leb128_u64(&mut cursor).unwrap();

    assert_eq!(value, 300);
    assert_eq!(cursor.position(), 2);
    assert_eq!(cursor.remaining(), 2);
}
