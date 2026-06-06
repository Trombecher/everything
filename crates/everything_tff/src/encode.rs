use core::fmt;
use std::{collections::HashMap, fmt::Write};

use base64::display::Base64Display;
use everything_structures::{Abstract, AnyStructure, BytesStructure, Object, Structure};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum DeduplicatedStructure {
    Any(AnyStructure),
    BytesOrText(BytesStructure),
}

pub struct Encoder<Out: Write> {
    encoded_structures: HashMap<DeduplicatedStructure, u64>,
    out: Out,
}

impl<Out: Write> Encoder<Out> {
    #[must_use]
    #[inline]
    pub fn new(out: Out) -> Self {
        Self {
            encoded_structures: HashMap::new(),
            out,
        }
    }

    fn encode_object(&mut self, object: Object) -> Result<(), fmt::Error> {
        match object {
            Object::Abstract(Abstract::ZERO) => self.out.write_char('0'),
            Object::Abstract(a) => {
                write!(
                    self.out,
                    "@{}",
                    Base64Display::new(
                        &a.0.to_be_bytes(),
                        &base64::engine::general_purpose::URL_SAFE_NO_PAD
                    )
                )
            }
            Object::Structure(Structure::Empty) => self.out.write_char('E'),
            Object::Structure(Structure::Integer(i)) => {
                write!(self.out, "{i}")
            }
            Object::Structure(Structure::Any(a)) => {
                let ds = DeduplicatedStructure::Any(a);
                let index = *self.encoded_structures.get(&ds).unwrap();

                write!(self.out, "r{index}")
            }
            Object::Structure(Structure::Text(text)) => {
                let ds = DeduplicatedStructure::BytesOrText(text.into_bytes());
                let index = *&self.encoded_structures.get(&ds).unwrap();

                write!(self.out, "t{index}")
            }
            Object::Structure(Structure::Bytes(bytes)) => {
                let ds = DeduplicatedStructure::BytesOrText(bytes);
                let index = *&self.encoded_structures.get(&ds).unwrap();

                write!(self.out, "b{index}")
            }
            Object::Structure(Structure::Byte(byte)) => {
                write!(self.out, "x{:02x}", byte.0)
            }
            Object::Structure(Structure::Character(c)) => {
                write!(self.out, ">{c}")
            }
        }
    }

    fn ensure_refs_are_encoded(&mut self, object: Object) -> Result<(), fmt::Error> {
        match object {
            Object::Structure(Structure::Text(text)) => {
                let ds = DeduplicatedStructure::BytesOrText(text.into_bytes());

                if !self.encoded_structures.contains_key(&ds) {
                    self.encoded_structures
                        .insert(ds.clone(), self.encoded_structures.len() as u64);

                    let text_bytes = match ds {
                        DeduplicatedStructure::BytesOrText(bytes) => bytes,
                        _ => unreachable!(),
                    };

                    write!(self.out, "\nT{}:{}", text_bytes.as_ref().len(), unsafe {
                        str::from_utf8_unchecked(text_bytes.as_ref())
                    })?;
                }
            }
            Object::Structure(Structure::Bytes(bytes)) => {
                let ds = DeduplicatedStructure::BytesOrText(bytes);

                if !self.encoded_structures.contains_key(&ds) {
                    self.encoded_structures
                        .insert(ds.clone(), self.encoded_structures.len() as u64);

                    let bytes = match ds {
                        DeduplicatedStructure::BytesOrText(bytes) => bytes,
                        _ => unreachable!(),
                    };

                    write!(
                        self.out,
                        "\nB{}:{}",
                        bytes.as_ref().len(),
                        Base64Display::new(
                            bytes.as_ref(),
                            &base64::engine::general_purpose::URL_SAFE_NO_PAD
                        )
                    )?;
                }
            }
            Object::Structure(Structure::Any(any)) => {
                let ds = DeduplicatedStructure::Any(any);

                if !self.encoded_structures.contains_key(&ds) {
                    let any = match &ds {
                        DeduplicatedStructure::Any(any) => any,
                        _ => unreachable!(),
                    };

                    for property in any.properties() {
                        self.ensure_refs_are_encoded(property.tag)?;
                        self.ensure_refs_are_encoded(property.value)?;
                    }

                    self.encoded_structures
                        .insert(ds.clone(), self.encoded_structures.len() as u64);

                    write!(self.out, "\nA{}", any.as_ref().len())?;

                    for property in any.properties() {
                        self.out.write_char('\n')?;

                        self.encode_object(property.tag)?;

                        self.out.write_char(':')?;

                        self.encode_object(property.value)?;
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn encode_root(&mut self, object: Object) -> Result<(), fmt::Error> {
        self.out.write_str("EVERYTHINGTS001")?;

        self.ensure_refs_are_encoded(object.clone())?;

        self.out.write_char('\n')?;
        self.encode_object(object)
    }
}
