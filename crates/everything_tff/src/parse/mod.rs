#[cfg(test)]
mod tests;

use base64::Engine;
use everything_objects::{Abstract, BytesComposite, Composite, Object, Property, TextComposite};

use crate::bytes::Bytes;

#[derive(PartialEq, Debug, Clone, thiserror::Error)]
#[error("error while parsing: expected '{expected}' at byte index {found_at}")]
pub struct Error {
    pub found_at: usize,
    pub expected: &'static str,
}

macro_rules! bail {
    ($found_at:expr, $expected:literal) => {
        return Err(Error {
            found_at: $found_at,
            expected: $expected,
        })
    };
}

#[derive(Debug, Clone)]
pub struct Parser<'source> {
    bytes: Bytes<'source>,
    previous_aliases: Vec<Composite>,
}

impl<'source> Parser<'source> {
    pub const fn new(source: &'source str) -> Self {
        Self {
            bytes: Bytes::new(source),
            previous_aliases: Vec::new(),
        }
    }

    pub fn parse_root(&mut self) -> Result<Object, Error> {
        if Some(*b"EVERYTHINGTS001\n") != self.bytes.next_chunk::<16>().ok() {
            bail!(self.bytes.index(), "invalid header")
        }

        while let Some(alias) = self.try_parse_composite_alias()? {
            self.previous_aliases.push(alias);

            // Every alias ends with a LF.
            match self.bytes.peek() {
                Some(b'\n') => {
                    self.bytes.next();
                }
                _ => bail!(self.bytes.index(), "'\\n'"),
            }
        }

        let object = self.parse_object()?;

        // Skip trailing LF.
        if let Some(b'\n') = self.bytes.peek() {
            self.bytes.next();
        }

        let None = self.bytes.peek() else {
            bail!(self.bytes.index(), "end of input")
        };

        Ok(object)
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

    fn try_parse_composite_alias(&mut self) -> Result<Option<Composite>, Error> {
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

                Ok(Some(BytesComposite::new(text).map_or(
                    Composite::Empty,
                    |bytes| {
                        Composite::Text(unsafe {
                            // SAFETY: start is at a char boundary, and end is too.
                            // Also, the whole source is a string, so this is safe.
                            TextComposite::new_unchecked(bytes)
                        })
                    },
                )))
            }
            Some(b'A') => {
                self.bytes.next();
                // Any Composite

                let number_of_properties = self.parse_u64()?;

                let mut properties = Vec::with_capacity(number_of_properties as usize);

                for _ in 0..number_of_properties {
                    properties.push(self.parse_property()?);
                }

                Ok(Some(Composite::new(&mut properties)))
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

                Ok(Some(
                    BytesComposite::new(&bytes).map_or(Composite::Empty, Composite::Bytes),
                ))
            }
            _ => Ok(None),
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

    fn get_text_composite(&mut self, index: u64) -> Result<TextComposite, Error> {
        let Some(composite) = self.previous_aliases.get_mut(index as usize) else {
            bail!(self.bytes.index(), "invalid Composite reference")
        };

        match composite.clone() {
            Composite::Text(text) => Ok(text),
            Composite::Bytes(bytes) => {
                if str::from_utf8(bytes.as_ref()).is_ok() {
                    let ret = unsafe { TextComposite::new_unchecked(bytes) };
                    *composite = Composite::Text(ret.clone());

                    Ok(ret)
                } else {
                    bail!(self.bytes.index(), ":/")
                }
            }
            _ => bail!(self.bytes.index(), "does not reference text-like Composite"),
        }
    }

    fn get_bytes_composite(&mut self, index: u64) -> Result<BytesComposite, Error> {
        let Some(composite) = self.previous_aliases.get_mut(index as usize) else {
            bail!(self.bytes.index(), "invalid Composite reference")
        };

        match composite.clone() {
            Composite::Bytes(bytes) => Ok(bytes),
            Composite::Text(text) => Ok(text.into_bytes()),
            _ => bail!(self.bytes.index(), "does not reference text-like Composite"),
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

                Ok(Composite::Character(c).into())
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

                let Some(composite) = self.previous_aliases.get(index as usize) else {
                    bail!(self.bytes.index(), "invalid Composite reference")
                };

                Ok(Object::Composite(composite.clone()))
            }
            Some(b't') => {
                self.bytes.next();

                let index = self.parse_u64()?;

                self.get_text_composite(index)
                    .map(Composite::Text)
                    .map(Object::Composite)
            }
            Some(b'b') => {
                self.bytes.next();

                let index = self.parse_u64()?;

                self.get_bytes_composite(index)
                    .map(Composite::Bytes)
                    .map(Object::Composite)
            }
            Some(b'E') => {
                self.bytes.next();

                Ok(Composite::Empty.into())
            }
            _ => bail!(self.bytes.index(), "an ASCII digit, '@', or 'R'"),
        }
    }
}
