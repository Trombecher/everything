#[cfg(test)]
mod tests;

use base64::{Engine, alphabet, engine::general_purpose::URL_SAFE_NO_PAD};
use everything::knowledge::{Knowledge, Statement};
use everything_objects::{Abstract, BytesComposite, Composite, Object, Property, TextComposite};

use crate::bytes::Bytes;

fn unescape(byte: u8) -> Option<u8> {
    match byte {
        b'"' | b'\\' => Some(byte),
        b'n' => Some(b'\n'),
        b'r' => Some(b'\r'),
        b'0' => Some(b'\0'),
        b't' => Some(b'\t'),
        _ => None,
    }
}

#[derive(PartialEq, Debug, Clone, thiserror::Error)]
#[error("error while parsing: '{message}' at byte index {found_at}")]
pub struct Error {
    pub found_at: usize,
    pub message: &'static str,
}

macro_rules! bail {
    ($found_at:expr, $expected:literal) => {
        return Err(Error {
            found_at: $found_at,
            message: $expected,
        })
    };
}

#[derive(Debug, Clone)]
pub struct Parser<'source> {
    bytes: Bytes<'source>,
    cached_vec: Vec<u8>,
}

impl<'source> Parser<'source> {
    const MAGIC_BYTES: [u8; 24] = *b"EVERYTHINGTEXTDATABASE01";

    pub const fn new(source: &'source str) -> Self {
        Self {
            bytes: Bytes::new(source),
            cached_vec: Vec::new(),
        }
    }

    fn try_parse_line_break(&mut self) -> Result<Option<()>, Error> {
        match self.bytes.next() {
            Some(b'\r') => {}
            Some(b'\n') => return Ok(Some(())),
            None => return Ok(None),
            _ => bail!(self.bytes.index(), "expected '\\r', '\\n', or end of input"),
        }

        if let Some(b'\n') = self.bytes.peek() {
            self.bytes.next();
        }

        Ok(Some(()))
    }

    fn try_parse_statement(&mut self) -> Result<Option<Statement>, Error> {
        if self.try_parse_line_break()?.is_none() {
            return Ok(None);
        }

        match self.bytes.peek() {
            Some(b'@') => {}
            None => return Ok(None),
            _ => bail!(self.bytes.index(), "expected '@' or end of input"),
        }

        let subject = match self.parse_object()? {
            Object::Abstract(a) => a,
            Object::Composite(_) => unreachable!("unreachable since we start with '@'"),
        };

        match self.bytes.next() {
            Some(b' ') => {}
            _ => bail!(self.bytes.index(), "expected ' '"),
        }

        let tag = self.parse_object()?;

        match self.bytes.next() {
            Some(b' ') => {}
            _ => bail!(self.bytes.index(), "expected ' '"),
        }

        let value = self.parse_object()?;

        // TODO: additional data

        Ok(Some(Statement {
            subject,
            tag,
            value,
            additional_properties: Composite::Empty,
        }))
    }

    pub fn parse_knowledge(&mut self) -> Result<Knowledge, Error> {
        if Some(Self::MAGIC_BYTES) != self.bytes.next_chunk::<24>().ok() {
            bail!(self.bytes.index(), "invalid magic bytes")
        }

        let mut knowledge = Knowledge::new();

        while let Some(statement) = self.try_parse_statement()? {
            knowledge.add_mut([statement].into_iter());
        }

        Ok(knowledge)
    }

    fn try_parse_composite_alias(&mut self) -> Result<Option<Composite>, Error> {
        match self.bytes.peek() {
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

    fn parse_property(&mut self) -> Result<Property, Error> {
        let tag = self.parse_object()?;

        let Some(b':') = self.bytes.peek() else {
            bail!(self.bytes.index(), "expected ':'")
        };

        self.bytes.next();

        let value = self.parse_object()?;

        Ok(Property { tag, value })
    }

    fn parse_object(&mut self) -> Result<Object, Error> {
        match self.bytes.peek() {
            Some(b'"') => {
                // TODO: maybe optimize this

                self.bytes.next();
                self.cached_vec.clear();

                loop {
                    match self.bytes.next() {
                        Some(b'"') => break,
                        Some(b'\\') => {
                            if let Some(real) = self.bytes.next().and_then(unescape) {
                                self.cached_vec.push(real);
                            } else {
                                bail!(self.bytes.index(), "expected 'n', '\"', 'r', '0', ...")
                            }
                        }
                        Some(byte) => self.cached_vec.push(byte),
                        None => bail!(self.bytes.index(), "expected '\"'"),
                    }
                }

                // This could be unchecked (unsafe).
                Ok(Composite::from(str::from_utf8(self.cached_vec.as_slice()).unwrap()).into())
            }
            Some(b'(') => {
                self.bytes.next();

                let mut properties = vec![];

                if Some(b')') != self.bytes.peek() {
                    loop {
                        properties.push(self.parse_property()?);

                        match self.bytes.next() {
                            Some(b')') => break,
                            Some(b',') => {
                                // Skip ','
                                self.bytes.next();
                            }
                            _ => bail!(self.bytes.index(), "expected ',' or ')'"),
                        }
                    }
                }

                // Skip ')'
                self.bytes.next();

                Ok(Object::Composite(Composite::new(&mut properties)))
            }
            Some(b'U') => {
                self.bytes.next();

                let mut value = 0_u32;

                loop {
                    match self.bytes.peek() {
                        Some(x @ b'0'..=b'9') => {
                            self.bytes.next();

                            value = value
                                .checked_mul(10)
                                .and_then(|value| value.checked_add((x - b'0') as u32))
                                .ok_or_else(|| Error {
                                    message: "",
                                    found_at: self.bytes.index(),
                                })?;
                        }
                        _ => break,
                    }
                }

                let Some(c) = char::from_u32(value) else {
                    return Err(Error {
                        found_at: self.bytes.index(),
                        message: "invalid char",
                    });
                };

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
            Some(b'<') => {
                self.bytes.next();

                loop {
                    match self.bytes.next() {
                        Some(b'>') => break,
                        Some(a) if alphabet::URL_SAFE.as_str().contains(a as char) => {}
                        _ => bail!(self.bytes.index(), ""),
                    }
                }
            }
            Some(b'E') => {
                self.bytes.next();

                Ok(Composite::Empty.into())
            }
            _ => bail!(self.bytes.index(), "an ASCII digit, '@', or 'R'"),
        }
    }
}
