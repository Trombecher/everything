use core::str::{self, Chars};

#[derive(Clone)]
pub struct PeekableChars<'a> {
    peeked: Option<char>,
    chars: Chars<'a>,
}

impl<'a> PeekableChars<'a> {
    #[must_use]
    pub const fn new(chars: Chars<'a>) -> Self {
        Self {
            peeked: None,
            chars,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        if self.peeked.is_none() {
            self.peeked = self.chars.next();
        }

        self.peeked
    }

    pub fn as_str(&self) -> &'a str {
        let len_extension = self.peeked.map_or(0, char::len_utf8);
        let s = self.chars.as_str();

        // SAFETY:
        //               Chars will yield this
        //               |---------
        // __________PEEK??????????
        //
        // So we have to extend that by the peeked char utf-8 len.
        unsafe { str::from_raw_parts(s.as_ptr().sub(len_extension), s.len() + len_extension) }
    }
}

impl<'a> Iterator for PeekableChars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.peeked.take() {
            Some(c)
        } else {
            self.chars.next()
        }
    }
}
