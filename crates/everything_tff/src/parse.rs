use base64::Engine;
use everything_structures::{Abstract, BytesStructure, Object, Property, Structure, TextStructure};

use crate::bytes::Bytes;

#[derive(Clone, Debug)]
pub enum ParserError {
    InvalidHeader,
    ExpectedStructure,
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

    pub fn parse_root(&mut self) -> Result<Structure, ParserError> {
        if Some(*b"EVERYTHINGTS001\n") != self.bytes.next_chunk::<16>().ok() {
            return Err(ParserError::InvalidHeader);
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
                _ => todo!(),
            }
        }

        Ok(structure_aliases.last().unwrap().clone().into())
    }

    fn parse_property(&mut self, previous_aliases: &[Structure]) -> Result<Property, ParserError> {
        let Some(b'\n') = self.bytes.next() else {
            todo!()
        };

        let tag = self.parse_object(previous_aliases)?;

        let Some(b':') = self.bytes.next() else {
            todo!()
        };

        let value = self.parse_object(previous_aliases)?;

        Ok(Property { tag, value })
    }

    fn parse_structure_alias(
        &mut self,
        previous_aliases: &[Structure],
    ) -> Result<Structure, ParserError> {
        match self.bytes.next() {
            Some(b'T') => {
                // Inline text

                let byte_length = self.parse_u64()?;

                match self.bytes.next() {
                    Some(b':') => {}
                    _ => todo!(),
                }

                let start = self.bytes.index();

                let Ok(()) = self.bytes.advance_by(byte_length as usize) else {
                    todo!()
                };

                let end = self.bytes.index();

                let Some(text) = self.bytes.whole_str().get(start..end) else {
                    todo!()
                };

                Ok(
                    BytesStructure::new(text.as_bytes()).map_or(Structure::Empty, |bytes| {
                        Structure::Text(unsafe { TextStructure::new_unchecked(bytes) })
                    }),
                )
            }
            Some(b'A') => {
                // Any structure

                let number_of_properties = self.parse_u64()?;

                let mut properties = Vec::with_capacity(number_of_properties as usize);

                for _ in 0..number_of_properties {
                    properties.push(self.parse_property(previous_aliases)?);
                }

                Ok(Structure::new(&mut properties))
            }
            Some(b'B') => {
                // Inline bytes

                let byte_length = self.parse_u64()?;

                match self.bytes.next() {
                    Some(b':') => {}
                    _ => todo!(),
                }

                let start = self.bytes.index();

                let Ok(()) = self.bytes.advance_by(byte_length as usize) else {
                    todo!()
                };

                let end = self.bytes.index();

                let base64_encoded_bytes = &self.bytes.whole_str().as_bytes()[start..end];

                let bytes = base64::engine::general_purpose::STANDARD
                    .decode(base64_encoded_bytes)
                    .map_err(|_| todo!())?;

                Ok(BytesStructure::new(&bytes).map_or(Structure::Empty, Structure::Bytes))
            }
            _ => Err(ParserError::ExpectedStructure),
        }
    }

    fn parse_u64(&mut self) -> Result<u64, ParserError> {
        let mut n = match self.bytes.next() {
            Some(n @ b'0'..=b'9') => (n - b'0') as u64,
            _ => todo!(),
        };

        while let Some(b @ b'0'..=b'9') = self.bytes.peek() {
            n = n
                .checked_mul(10)
                .and_then(|n| n.checked_add((b - b'0') as u64))
                .ok_or_else(|| todo!())?;

            self.bytes.next();
        }

        Ok(n)
    }

    fn parse_object(&mut self, previous_aliases: &[Structure]) -> Result<Object, ParserError> {
        match self.bytes.next() {
            Some(b'@') => {
                let mut id = match self.bytes.next() {
                    Some(b @ b'0'..=b'9') => (b - b'0') as u128,
                    _ => todo!(),
                };

                while let Some(b @ b'0'..=b'9') = self.bytes.peek() {
                    id = id
                        .checked_mul(10)
                        .and_then(|n| n.checked_add((b - b'0') as u128))
                        .ok_or_else(|| todo!())?;

                    self.bytes.next();
                }

                Ok(Object::Abstract(Abstract(id)))
            }
            Some(b'R') => {
                let index = self.parse_u64()?;

                let Some(structure) = previous_aliases.get(index as usize) else {
                    todo!()
                };

                Ok(Object::Structure(structure.clone()))
            }
            _ => todo!(),
        }
    }
}
