mod color;
mod email;
mod lang;
mod tests;
mod time;
mod url;
// TODO: mod schema;
// TODO: mod constraints;

pub use color::*;
pub use email::*;
pub use lang::*;
pub use time::*;
pub use url::*;

use crate::ff;
use crate::objects::ObjectId;
use crate::res::ResourceId;
use std::borrow::Borrow;
use std::fmt::{Debug, Formatter, Pointer, Write};
use std::mem::transmute;
use std::str::from_utf8_unchecked;
use tracing::warn;

/// An owned value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Value(pub [u64; 2]);

impl Value {
    pub fn decode(self) -> Option<DecodedValue> {
        let view: [u8; 16] = unsafe { transmute(self.0) };
        let slot_u64: u64 = u64::from_le(self.0[1]);
        let u32s: (u32, u32, u32, u32) = unsafe { transmute(self.0) };

        match view[0] {
            ff::INTEGER => Some(DecodedValue::Integer(slot_u64 as i64)),
            ff::FLOAT => Some(DecodedValue::Float(f64::from_bits(slot_u64))),
            ff::CHARACTER => Some(DecodedValue::Character(
                if let Ok(x) = char::try_from(u32s.1) {
                    x
                } else {
                    warn!(
                        "Encountered invalid character {:?} while decoding value. Replacing with U+FFFD.",
                        u32s.1
                    );
                    '\u{FFFD}'
                },
            )),
            _ => {
                warn!("Encountered invalid value {view:?}. Replacing with `None`.");
                None
            }
        }
    }
}

/// A value whose top layer has been decoded. More info at the docs.
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum DecodedValue {
    Integer(i64) = ff::INTEGER,
    Float(f64) = ff::FLOAT,
    Character(char) = ff::CHARACTER,
    Duration(Duration) = ff::DURATION,
    DateTime(DateTime) = ff::DATE_TIME,
    ObjectReference(Option<ObjectId>) = ff::OBJECT,
    Language(Language) = ff::LANGUAGE,
    Url(ValueContent<Url>) = ff::URL,
    Color(Color) = ff::COLOR,
    // TODO: Schema
    // TODO: Constraint
    Email(ValueContent<Email>) = ff::EMAIL,
    Text(ValueContent<str>) = ff::TEXT,
    Binary(ValueContent<[u8]>) = ff::BINARY,
    EncryptedEmail(ResourceId) = ff::ENC_EMAIL,
    EncryptedText(ResourceId) = ff::ENC_TEXT,
    EncryptedBinary(ResourceId) = ff::ENC_BINARY,
}

pub enum ValueContent<T: ResourceDependent + ?Sized> {
    Resource(ResourceId),
    InlineMax(T::InlineMax),
    Inline(T::Inline),
}

impl<T: ResourceDependent + ?Sized> Clone for ValueContent<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Resource(r) => Self::Resource(*r),
            Self::InlineMax(i) => Self::InlineMax(i.clone()),
            Self::Inline(i) => Self::Inline(i.clone()),
        }
    }
}

impl<T: ResourceDependent + ?Sized> Debug for ValueContent<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueContent::Resource(r) => r.fmt(f),
            ValueContent::InlineMax(i) => i.borrow().fmt(f),
            ValueContent::Inline(i) => i.borrow().fmt(f),
        }
    }
}

/// Indicates that the type is too big to fit into the value inline.
/// [Self::Inline] must be 15 bytes.
pub(crate) unsafe trait ResourceDependent {
    type InlineMax: Sized + Clone + Borrow<Self>;
    type Inline: Sized + Clone + Borrow<Self>;
}

unsafe impl ResourceDependent for [u8] {
    type InlineMax = [u8; 15];
    type Inline = InlineBytes;
}

#[derive(Clone)]
pub struct InlineBytes {
    len: u8,
    bytes: [u8; 14],
}

impl Borrow<[u8]> for InlineBytes {
    fn borrow(&self) -> &[u8] {
        &self.bytes[..self.len as usize]
    }
}

unsafe impl ResourceDependent for str {
    type InlineMax = InlineStrMax;
    type Inline = InlineStr;
}

#[derive(Clone)]
pub struct InlineStrMax([u8; 15]);

impl Borrow<str> for InlineStrMax {
    fn borrow(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.0) }
    }
}

#[derive(Clone)]
pub struct InlineStr {
    len: u8,
    text: [u8; 14],
}

impl Borrow<str> for InlineStr {
    fn borrow(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.text[..self.len as usize]) }
    }
}
