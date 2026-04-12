use crate::{BlobStructure, GlobalRegistry, Registry};

/// Represents an array of unicode characters.
#[derive(Clone)]
pub struct TextStructure<R: Registry = GlobalRegistry>(BlobStructure<R>);

impl AsRef<str> for TextStructure {
    fn as_ref(&self) -> &str {
        // SAFETY: .0 always contains valid UTF-8 and it cannot be changed.
        unsafe { str::from_utf8_unchecked(self.0.as_ref()) }
    }
}
