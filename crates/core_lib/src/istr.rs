/// The `str` equivalent of `[u8; N]`.
pub struct InlineStr<const N: usize>([u8; N]);

impl<const N: usize> AsRef<str> for InlineStr<N> {
    fn as_ref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl<const N: usize> InlineStr<N> {
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(data: [u8; N]) -> Self {
        Self(data)
    }

    #[inline]
    #[must_use]
    pub fn new(data: [u8; N]) -> Option<Self> {
        std::str::from_utf8(&data)
            .ok()
            .map(|_| unsafe { Self::new_unchecked(data) })
    }
}
