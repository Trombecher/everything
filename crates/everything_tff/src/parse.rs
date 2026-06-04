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
}

impl<'source> Parser<'source> {
    pub const fn new(source: &'source str) -> Self {
        Self {
            bytes: Bytes::new(source),
        }
    }

    pub fn parse_root(&mut self) -> Result<Structure, Error> {
        if Some(*b"EVERYTHINGTS001\n") != self.bytes.next_chunk::<16>().ok() {
            bail!(self.bytes.index(), "invalid header")
        }

        let mut structure_aliases = Vec::new();

        loop {
            let alias = self.parse_structure_alias(&structure_aliases)?;

            structure_aliases.push(alias);

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

        Ok(structure_aliases.last().unwrap().clone().into())
    }

    fn parse_property(&mut self, previous_aliases: &[Structure]) -> Result<Property, Error> {
        let Some(b'\n') = self.bytes.peek() else {
            bail!(self.bytes.index(), "'\\n'")
        };

        self.bytes.next();

        let tag = self.parse_object(previous_aliases)?;

        let Some(b':') = self.bytes.peek() else {
            bail!(self.bytes.index(), "':'")
        };

        self.bytes.next();

        let value = self.parse_object(previous_aliases)?;

        Ok(Property { tag, value })
    }

    fn parse_structure_alias(
        &mut self,
        previous_aliases: &[Structure],
    ) -> Result<Structure, Error> {
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
                    properties.push(self.parse_property(previous_aliases)?);
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

        while let Some(b @ b'0'..=b'9') = self.bytes.peek() {
            if let Some(next_n) = n
                .checked_mul(10)
                .and_then(|n| n.checked_add((b - b'0') as u64))
            {
                n = next_n
            } else {
                bail!(self.bytes.index(), "number too big")
            }

            self.bytes.next();
        }

        Ok(n)
    }

    fn parse_object(&mut self, previous_aliases: &[Structure]) -> Result<Object, Error> {
        match self.bytes.peek() {
            Some(b'@') => {
                self.bytes.next();

                let mut id = match self.bytes.peek() {
                    Some(n @ b'0'..=b'9') => {
                        self.bytes.next();

                        (n - b'0') as u128
                    }
                    _ => bail!(self.bytes.index(), "an ASCII digit"),
                };

                while let Some(b @ b'0'..=b'9') = self.bytes.peek() {
                    if let Some(next_n) = id
                        .checked_mul(10)
                        .and_then(|n| n.checked_add((b - b'0') as u128))
                    {
                        id = next_n
                    } else {
                        bail!(self.bytes.index(), "abstract object number too big")
                    }

                    self.bytes.next();
                }

                Ok(Object::Abstract(Abstract(id)))
            }
            Some(b'R') => {
                self.bytes.next();

                let index = self.parse_u64()?;

                let Some(structure) = previous_aliases.get(index as usize) else {
                    bail!(self.bytes.index(), "invalid structure reference")
                };

                Ok(Object::Structure(structure.clone()))
            }
            _ => bail!(self.bytes.index(), "an ASCII digit, '@', or 'R'"),
        }
    }
}
