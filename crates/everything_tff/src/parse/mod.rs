#[cfg(test)]
mod tests;

use base64::Engine;
use everything_structures::{Abstract, BytesStructure, Object, Property, Structure, TextStructure};

use crate::bytes::Bytes;

pub type Error = Box<ErrorInfo>;

#[derive(PartialEq, Debug, Clone)]
pub struct ErrorInfo {
    pub found_at: usize,
    pub expected: &'static str,
}

macro_rules! bail {
    ($found_at:expr, $expected:literal) => {
        return Err(Box::new(ErrorInfo {
            found_at: $found_at,
            expected: $expected,
        }))
    };
}

#[derive(Debug, Clone)]
pub struct Parser<'source> {
    bytes: Bytes<'source>,
    previous_aliases: Vec<Structure>,
}

impl<'source> Parser<'source> {
    pub const fn new(source: &'source str) -> Self {
        Self {
            bytes: Bytes::new(source),
            previous_aliases: Vec::new(),
        }
    }

    pub fn parse_root(&mut self) -> Result<Structure, Error> {
        if Some(*b"EVERYTHINGTS001\n") != self.bytes.next_chunk::<16>().ok() {
            bail!(self.bytes.index(), "invalid header")
        }

        loop {
            let alias = self.parse_structure_alias()?;

            self.previous_aliases.push(alias);

            match self.bytes.next() {
                None => break,
                Some(b'\n') => {
                    if let None = self.bytes.peek() {
                        break;
                    }
                }
                _ => bail!(self.bytes.index() - 1, "end of input or '\\n'"),
            }
        }

        Ok(self.previous_aliases.last().unwrap().clone().into())
    }

    fn parse_property(&mut self) -> Result<Property, Error> {
        let Some(b'\n') = self.bytes.peek() else {
            bail!(self.bytes.index(), "'\\n'")
        };

        self.bytes.next();

        let tag = self.parse_object()?;

        let Some(b':') = self.bytes.peek() else {
            bail!(self.bytes.index(), "':'")
        };

        self.bytes.next();

        let value = self.parse_object()?;

        Ok(Property { tag, value })
    }

    fn parse_structure_alias(&mut self) -> Result<Structure, Error> {
        match self.bytes.peek() {
            Some(b'T') => {
                self.bytes.next();

                // Inline text

                let byte_length = self.parse_u64()?;

                match self.bytes.peek() {
                    Some(b':') => {
                        self.bytes.next();
                    }
                    _ => bail!(self.bytes.index(), "an ASCII digit or ':'"),
                }

                let start = self.bytes.index();

                let Ok(()) = self.bytes.advance_by(byte_length as usize) else {
                    bail!(
                        self.bytes.index(),
                        "invalid text byte length (expected some more bytes)"
                    )
                };

                let end = self.bytes.index();

                if !self.bytes.whole_str().is_char_boundary(end) {
                    bail!(
                        end,
                        "invalid text byte length (not a UTF-8 char boundary at the end)"
                    )
                }

                let text = &self.bytes.whole_str().as_bytes()[start..end];

                Ok(BytesStructure::new(text).map_or(Structure::Empty, |bytes| {
                    Structure::Text(unsafe {
                        // SAFETY: start is at a char boundary, and end is too.
                        // Also, the whole source is a string, so this is safe.
                        TextStructure::new_unchecked(bytes)
                    })
                }))
            }
            Some(b'A') => {
                self.bytes.next();
                // Any structure

                let number_of_properties = self.parse_u64()?;

                let mut properties = Vec::with_capacity(number_of_properties as usize);

                for _ in 0..number_of_properties {
                    properties.push(self.parse_property()?);
                }

                Ok(Structure::new(&mut properties))
            }
            Some(b'B') => {
                self.bytes.next();
                // Inline bytes

                let byte_length = self.parse_u64()?;

                match self.bytes.peek() {
                    Some(b':') => {
                        self.bytes.next();
                    }
                    _ => bail!(self.bytes.index(), "':'"),
                }

                let start = self.bytes.index();

                let Ok(()) = self.bytes.advance_by(byte_length as usize) else {
                    bail!(self.bytes.index(), "invalid bytes length")
                };

                let end = self.bytes.index();

                let base64_encoded_bytes = &self.bytes.whole_str().as_bytes()[start..end];

                let Ok(bytes) =
                    base64::engine::general_purpose::STANDARD.decode(base64_encoded_bytes)
                else {
                    bail!(self.bytes.index(), "invalid base64")
                };

                Ok(BytesStructure::new(&bytes).map_or(Structure::Empty, Structure::Bytes))
            }
            _ => bail!(self.bytes.index(), "expected 'T', 'A', or 'B'"),
        }
    }

    fn parse_u64(&mut self) -> Result<u64, Error> {
        let mut n = match self.bytes.peek() {
            Some(n @ b'0'..=b'9') => {
                self.bytes.next();

                (n - b'0') as u64
            }
            _ => bail!(self.bytes.index(), "an ASCII digit"),
        };

        while let Some(digit @ b'0'..=b'9') = self.bytes.peek() {
            if let Some(next_n) = n
                .checked_mul(10)
                .and_then(|n| n.checked_add((digit - b'0') as u64))
            {
                n = next_n
            } else {
                bail!(self.bytes.index(), "number too big")
            }

            self.bytes.next();
        }

        Ok(n)
    }

    fn get_text_structure(&mut self, index: u64) -> Result<TextStructure, Error> {
        let Some(structure) = self.previous_aliases.get_mut(index as usize) else {
            bail!(self.bytes.index(), "invalid structure reference")
        };

        match structure.clone() {
            Structure::Text(text) => Ok(text),
            Structure::Bytes(bytes) => {
                if let Ok(_) = str::from_utf8(bytes.as_ref()) {
                    let ret = unsafe { TextStructure::new_unchecked(bytes) };
                    *structure = Structure::Text(ret.clone());

                    Ok(ret)
                } else {
                    bail!(self.bytes.index(), ":/")
                }
            }
            _ => bail!(self.bytes.index(), "does not reference text-like structure"),
        }
    }

    fn get_bytes_structure(&mut self, index: u64) -> Result<BytesStructure, Error> {
        let Some(structure) = self.previous_aliases.get_mut(index as usize) else {
            bail!(self.bytes.index(), "invalid structure reference")
        };

        match structure.clone() {
            Structure::Bytes(bytes) => Ok(bytes),
            Structure::Text(text) => Ok(text.into_bytes()),
            _ => bail!(self.bytes.index(), "does not reference text-like structure"),
        }
    }

    fn parse_u128(&mut self, start: u128) -> Result<u128, Error> {
        let mut n = start;

        while let Some(digit @ b'0'..=b'9') = self.bytes.peek() {
            if let Some(next_n) = n
                .checked_mul(10)
                .and_then(|n| n.checked_add((digit - b'0') as u128))
            {
                n = next_n
            } else {
                bail!(self.bytes.index(), "number too big")
            }

            self.bytes.next();
        }

        Ok(n)
    }

    fn parse_object(&mut self) -> Result<Object, Error> {
        match self.bytes.peek() {
            Some(b'>') => {
                self.bytes.next();

                let remaining_str =
                    unsafe { self.bytes.whole_str().get_unchecked(self.bytes.index()..) };

                let Some(c) = remaining_str.chars().next() else {
                    bail!(self.bytes.index(), "char")
                };

                let _ = self.bytes.advance_by(c.len_utf8());

                Ok(Structure::Character(c).into())
            }
            Some(b'-') => {
                self.bytes.next();

                let n = self.parse_u128(0)?;

                if n == 1_u128 << 127 {
                    Ok(Object::new_integer(i128::MIN))
                } else if let Ok(positive) = i128::try_from(n) {
                    Ok(Object::new_integer(-positive))
                } else {
                    bail!(self.bytes.index(), "integer too small")
                }
            }
            Some(n @ b'0'..=b'9') => {
                self.bytes.next();

                let n = self.parse_u128((n - b'0') as u128)?;

                if let Ok(i) = i128::try_from(self.parse_u128(n)?) {
                    Ok(Object::new_integer(i))
                } else {
                    bail!(self.bytes.index(), "integer too large")
                }
            }
            Some(b'@') => {
                self.bytes.next();

                let Ok(base64_encoded_id) = self.bytes.next_chunk::<22>() else {
                    bail!(self.bytes.index(), "expected 22 bytes of base64 encoded id")
                };

                let mut out = [0_u8; 16];

                let Ok(_) = base64::engine::general_purpose::URL_SAFE_NO_PAD
                    .decode_slice(base64_encoded_id, &mut out)
                else {
                    bail!(self.bytes.index(), "invalid base64 id")
                };

                Ok(Object::Abstract(Abstract(u128::from_be_bytes(out))))
            }
            Some(b'r') => {
                self.bytes.next();

                let index = self.parse_u64()?;

                let Some(structure) = self.previous_aliases.get(index as usize) else {
                    bail!(self.bytes.index(), "invalid structure reference")
                };

                Ok(Object::Structure(structure.clone()))
            }
            Some(b't') => {
                self.bytes.next();

                let index = self.parse_u64()?;

                self.get_text_structure(index)
                    .map(Structure::Text)
                    .map(Object::Structure)
            }
            Some(b'b') => {
                self.bytes.next();

                let index = self.parse_u64()?;

                self.get_bytes_structure(index)
                    .map(Structure::Bytes)
                    .map(Object::Structure)
            }
            Some(b'E') => {
                self.bytes.next();

                Ok(Structure::Empty.into())
            }
            _ => bail!(self.bytes.index(), "an ASCII digit, '@', or 'R'"),
        }
    }
}
