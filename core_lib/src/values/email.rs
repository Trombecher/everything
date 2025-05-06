use std::{mem::transmute, sync::LazyLock};
use std::borrow::Borrow;
use std::str::from_utf8_unchecked;
use regex::Regex;
use crate::values::ResourceDependent;

static REGEX: LazyLock<Regex> = LazyLock::new(||
    Regex::new(r"^([A-Z0-9_+-]+\.?)*[A-Z0-9_+-]@([A-Z0-9][A-Z0-9-]*\.)+[A-Z]{2,}$").unwrap());

/// An email. Wraps a `str`.
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct Email(str);

impl Borrow<str> for Email {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Email {
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(text: &str) -> &Self {
        unsafe { transmute(text) }
    }

    #[inline]
    #[must_use]
    pub fn new(text: &str) -> Option<&Self> {
        REGEX.is_match(text)
            .then_some(unsafe { Self::new_unchecked(text) })
    }
}

unsafe impl ResourceDependent for Email {
    type InlineMax = InlineEmailMax;
    type Inline = InlineEmail;
}

#[derive(Clone)]
pub struct InlineEmailMax([u8; 15]);

impl Borrow<Email> for InlineEmailMax {
    fn borrow(&self) -> &Email {
        unsafe {
            Email::new_unchecked(from_utf8_unchecked(&self.0))
        }   
    }
}

#[derive(Clone)]
pub struct InlineEmail {
    len: u8,
    bytes: [u8; 14]
}

impl Borrow<Email> for InlineEmail {
    fn borrow(&self) -> &Email {
        unsafe {
            Email::new_unchecked(from_utf8_unchecked(
                &self.bytes[..self.len.min(14) as usize]
            ))
        }
    }
}