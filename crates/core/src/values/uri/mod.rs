mod regex;

use crate::values::inline::{InlineStr, InlineStrMax};
pub use regex::*;

#[derive(Clone, PartialEq, Debug)]
#[repr(transparent)]
pub struct Uri(InlineStr);

impl TryFrom<&str> for Uri {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(v) = InlineStr::try_from(value)
            && REGEX.is_match(value)
        {
            Ok(Self(v))
        } else {
            Err(())
        }
    }
}

impl AsRef<str> for Uri {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Clone, PartialEq, Debug)]
#[repr(transparent)]
pub struct UriMax(InlineStrMax);
