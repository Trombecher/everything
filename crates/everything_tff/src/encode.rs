use core::fmt;
use std::{collections::HashMap, fmt::Write};

use base64::display::Base64Display;
use everything_objects::{Abstract, AnyComposite, BytesComposite, Composite, Object};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum DeduplicatedComposite {
    Any(AnyComposite),
    BytesOrText(BytesComposite),
}

pub struct Encoder<Out: Write> {
    encoded_composites: HashMap<DeduplicatedComposite, u64>,
    out: Out,
}

impl<Out: Write> Encoder<Out> {
    #[must_use]
    #[inline]
    pub fn new(out: Out) -> Self {
        Self {
            encoded_composites: HashMap::new(),
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
            Object::Composite(Composite::Empty) => self.out.write_char('E'),
            Object::Composite(Composite::Integer(i)) => {
                write!(self.out, "{i}")
            }
            Object::Composite(Composite::Any(a)) => {
                let ds = DeduplicatedComposite::Any(a);
                let index = *self.encoded_composites.get(&ds).unwrap();

                write!(self.out, "r{index}")
            }
            Object::Composite(Composite::Text(text)) => {
                let ds = DeduplicatedComposite::BytesOrText(text.into_bytes());
                let index = *&self.encoded_composites.get(&ds).unwrap();

                write!(self.out, "t{index}")
            }
            Object::Composite(Composite::Bytes(bytes)) => {
                let ds = DeduplicatedComposite::BytesOrText(bytes);
                let index = *&self.encoded_composites.get(&ds).unwrap();

                write!(self.out, "b{index}")
            }
            Object::Composite(Composite::Byte(byte)) => {
                write!(self.out, "x{:02x}", byte.0)
            }
            Object::Composite(Composite::Character(c)) => {
                write!(self.out, ">{c}")
            }
        }
    }

    fn ensure_refs_are_encoded(&mut self, object: Object) -> Result<(), fmt::Error> {
        match object {
            Object::Composite(Composite::Text(text)) => {
                let ds = DeduplicatedComposite::BytesOrText(text.into_bytes());

                if !self.encoded_composites.contains_key(&ds) {
                    self.encoded_composites
                        .insert(ds.clone(), self.encoded_composites.len() as u64);

                    let text_bytes = match ds {
                        DeduplicatedComposite::BytesOrText(bytes) => bytes,
                        _ => unreachable!(),
                    };

                    write!(self.out, "\nT{}:{}", text_bytes.as_ref().len(), unsafe {
                        str::from_utf8_unchecked(text_bytes.as_ref())
                    })?;
                }
            }
            Object::Composite(Composite::Bytes(bytes)) => {
                let ds = DeduplicatedComposite::BytesOrText(bytes);

                if !self.encoded_composites.contains_key(&ds) {
                    self.encoded_composites
                        .insert(ds.clone(), self.encoded_composites.len() as u64);

                    let bytes = match ds {
                        DeduplicatedComposite::BytesOrText(bytes) => bytes,
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
            Object::Composite(Composite::Any(any)) => {
                let ds = DeduplicatedComposite::Any(any);

                if !self.encoded_composites.contains_key(&ds) {
                    let any = match &ds {
                        DeduplicatedComposite::Any(any) => any,
                        _ => unreachable!(),
                    };

                    for property in any.properties() {
                        self.ensure_refs_are_encoded(property.tag)?;
                        self.ensure_refs_are_encoded(property.value)?;
                    }

                    self.encoded_composites
                        .insert(ds.clone(), self.encoded_composites.len() as u64);

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
