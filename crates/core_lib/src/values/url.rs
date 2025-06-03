use std::borrow::Borrow;
use std::mem::transmute;
use std::str::from_utf8_unchecked;
use crate::values::{ResourceDependent};

#[derive(Debug)]
#[repr(transparent)]
pub struct Url(str);

impl Url {
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(url: &str) -> &Self {
        unsafe { transmute(url) }
    }
}

unsafe impl ResourceDependent for Url {
    type InlineMax = InlineUrlMax;
    type Inline = InlineUrl;
}

#[derive(Clone)]
pub struct InlineUrlMax([u8; 15]);

impl Borrow<Url> for InlineUrlMax {
    fn borrow(&self) -> &Url {
        unsafe {
            Url::new_unchecked(from_utf8_unchecked(&self.0))
        }
    }
}

#[derive(Clone)]
pub struct InlineUrl {
    len: u8,
    data: [u8; 14],
}

impl Borrow<Url> for InlineUrl {
    fn borrow(&self) -> &Url {
        unsafe {
            Url::new_unchecked(from_utf8_unchecked(&self.data[..self.len as usize]))
        }
    }
}