#![no_std]

#[derive(Copy, Clone)]
pub struct Cursor<'slice> {
    slice: &'slice [u8],
}

impl<'slice> Cursor<'slice> {
    #[must_use]
    #[inline(always)]
    pub const fn new(slice: &'slice [u8]) -> Self {
        Self { slice }
    }

    #[must_use]
    #[inline(always)]
    pub const fn peek(self) -> Option<u8> {
        self.slice.first().copied()
    }

    #[must_use]
    #[inline(always)]
    pub fn peek_n(self, n: usize) -> Option<u8> {
        self.slice.get(n).copied()
    }
}

impl Iterator for Cursor<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let byte = self.peek();

        if byte.is_some() {
            self.slice = &self.slice[1..];
        }

        byte
    }
}
