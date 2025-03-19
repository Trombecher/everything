use std::{mem::transmute, sync::LazyLock};

use regex::Regex;

static REGEX: LazyLock<Regex> = LazyLock::new(||
    Regex::new(r"^([A-Z0-9_+-]+\.?)*[A-Z0-9_+-]@([A-Z0-9][A-Z0-9-]*\.)+[A-Z]{2,}$").unwrap());

/// An email. Wraps a `str`.
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct Email(str);

impl Email {
    #[inline]
    pub const fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_boxed(&self) -> Box<Email> {
        unsafe { transmute(self.as_str().to_string().into_boxed_str()) }
    }
}

impl Clone for Box<Email> {
    fn clone(&self) -> Self {
        unsafe {
            transmute(transmute::<_, &Box<str>>(self).clone())
        }
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