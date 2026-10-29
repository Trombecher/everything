use scursor::ReadCursor;

#[derive(Debug, thiserror::Error)]
#[error("error while parsing a u64 in LEB128 form")]
pub struct Leb128U64ParsingError;

pub trait ReadCursorExt {
    fn read_leb128_u64(&mut self) -> Result<u64, Leb128U64ParsingError>;
}

impl ReadCursorExt for ReadCursor<'_> {
    fn read_leb128_u64(&mut self) -> Result<u64, Leb128U64ParsingError> {
        let mut result = 0_u64;
        let mut shift = 0_u32;

        loop {
            let byte = self.read_u8().map_err(|_| Leb128U64ParsingError)?;

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
}
