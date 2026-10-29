use std::io::Cursor;

use scursor::ReadCursor;

#[derive(Debug, thiserror::Error)]
#[error("error while parsing a u64 in LEB128 form")]
pub struct Leb128U64ParsingError;

pub trait ReadCursorExt {
    fn read_into(&mut self, buffer: &mut [u8]) -> usize;

    fn read_leb128_u64(&mut self) -> Result<u64, Leb128U64ParsingError>;
}

impl ReadCursorExt for ReadCursor<'_> {
    fn read_into(&mut self, buffer: &mut [u8]) -> usize {
        let wrote = buffer.len().min(self.remaining());

        buffer.copy_from_slice(self.read_bytes(wrote).unwrap());

        wrote
    }

    fn read_leb128_u64(&mut self) -> Result<u64, Leb128U64ParsingError> {
        let mut buffer = [0_u8; 10];
        self.read_into(&mut buffer);

        leb128::read::unsigned(&mut Cursor::new(buffer)).map_err(|_| Leb128U64ParsingError)
    }
}
