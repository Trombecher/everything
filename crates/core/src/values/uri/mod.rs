mod regex;

use arrayvec::ArrayString;
pub use regex::*;

#[derive(Clone, PartialEq, Debug)]
#[repr(transparent)]
pub struct Uri(ArrayString<15>);

impl TryFrom<&str> for Uri {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(v) = ArrayString::try_from(value)
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