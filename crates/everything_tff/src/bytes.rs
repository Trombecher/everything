/// An iterator over bytes of a string.
#[derive(Debug, Clone)]
pub struct Bytes<'a> {
    slice: &'a str,
    index: usize,
}

impl<'a> Bytes<'a> {
    #[inline]
    #[must_use]
    pub const fn new(source: &'a str) -> Self {
        Self {
            slice: source,
            index: 0,
        }
    }

    #[inline]
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }

    #[inline]
    #[must_use]
    pub fn peek(&mut self) -> Option<u8> {
        self.slice.as_bytes().get(self.index).cloned()
    }

    #[inline]
    #[must_use]
    pub const fn whole_str(&self) -> &'a str {
        self.slice
    }

    #[must_use]
    pub const fn remaining_bytes(&self) -> usize {
        self.slice.len() - self.index
    }

    pub fn try_next_chunk<const N: usize>(&mut self) -> Option<[u8; N]> {
        if self.remaining_bytes() >= N {
            Some(self.next_chunk::<N>().unwrap())
        } else {
            None
        }
    }
}

impl<'a> Iterator for Bytes<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.peek().inspect(|_| {
            self.index += 1;
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining_bytes(), Some(self.remaining_bytes()))
    }
}
