use std::fmt::{Debug, Formatter};

// region InlineBytes

#[derive(Clone)]
pub struct InlineBytes {
    len: u8,
    content: [u8; 14],
}

impl TryFrom<&[u8]> for InlineBytes {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() > 14 {
            Err(())
        } else {
            let mut content = [0_u8; 14];
            (&mut content[..value.len()]).copy_from_slice(value);

            Ok(Self {
                len: value.len() as u8,
                content,
            })
        }
    }
}

impl AsRef<[u8]> for InlineBytes {
    fn as_ref(&self) -> &[u8] {
        &self.content[..self.len as usize]
    }
}

impl PartialEq for InlineBytes {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Debug for InlineBytes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

// endregion

// region InlineStr

#[derive(Clone)]
#[repr(transparent)]
pub struct InlineStr(InlineBytes);

impl TryFrom<&str> for InlineStr {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > 14 {
            Err(())
        } else {
            Ok(Self(InlineBytes::try_from(value.as_bytes())?))
        }
    }
}

impl AsRef<str> for InlineStr {
    fn as_ref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.0.as_ref()) }
    }
}

impl AsRef<[u8]> for InlineStr {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Debug for InlineStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = self.as_ref();
        s.fmt(f)
    }
}

impl PartialEq for InlineStr {
    fn eq(&self, other: &Self) -> bool {
        let a: &[u8] = self.as_ref();
        let b: &[u8] = other.as_ref();

        a == b
    }
}

// endregion

#[derive(Clone, PartialEq, Debug)]
#[repr(transparent)]
pub struct InlineStrMax([u8; 15]);

impl AsRef<[u8]> for InlineStrMax {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<str> for InlineStrMax {
    fn as_ref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_ref()) }
    }
}
