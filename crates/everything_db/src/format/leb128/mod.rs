use scursor::ReadCursor;

#[cfg(test)]
mod tests;

#[derive(Debug, thiserror::Error)]
#[error("error while parsing a u64 in LEB128 form")]
pub struct Leb128U64ParsingError;

/// Reads a [`u64`] encoded in LEB128 form from the cursor,
/// and leaves the cursor positioned after the encoding.
///
/// * errors when the encoding is truncated (end of input mid-sequence);
/// * errors when the encoding would not fit in a [`u64`];
/// * errors when the encoding is longer than ten bytes;
/// * accepts non-minimal (redundant) encodings terminated by a zero continuation.
pub fn read_leb128_u64(cursor: &mut ReadCursor) -> Result<u64, Leb128U64ParsingError> {
    let mut result = 0_u64;
    let mut shift = 0_u32;

    loop {
        let byte = cursor.read_u8().map_err(|_| Leb128U64ParsingError)?;

        let chunk = u64::from(byte & 0x7F);

        if shift >= 64 {
            return Err(Leb128U64ParsingError);
        }

        if shift + 7 >= 64 && (chunk >> (64 - shift)) != 0 {
            // The high bits of this byte would fall out of a u64.
            return Err(Leb128U64ParsingError);
        }

        result |= chunk << shift;

        if byte & 0x80 == 0 {
            return Ok(result);
        }

        shift += 7;
    }
}

/// Appends the given [`u64`] to `out` in LEB128 form.
pub fn write_leb128_u64(mut value: u64, out: &mut Vec<u8>) {
    loop {
        let mut byte = u8::try_from(value & 0x7F).expect("the value is masked to seven bits");

        value >>= 7;

        if value != 0 {
            byte |= 0x80;
        }

        out.push(byte);

        if value == 0 {
            return;
        }
    }
}
