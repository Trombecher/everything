use scursor::ReadCursor;

use super::leb128::{self, Leb128U64ParsingError};

pub trait ReadCursorExt {
    fn read_leb128_u64(&mut self) -> Result<u64, Leb128U64ParsingError>;
}

impl ReadCursorExt for ReadCursor<'_> {
    fn read_leb128_u64(&mut self) -> Result<u64, Leb128U64ParsingError> {
        leb128::read_leb128_u64(self)
    }
}
