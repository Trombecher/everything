use regex::Regex;
use std::sync::LazyLock;
use arrayvec::ArrayString;

pub static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\A(?=[a-z0-9@.!#$%&'*+/=?^_`{|}~-]{6,254}\z)
 (?=[a-z0-9.!#$%&'*+/=?^_`{|}~-]{1,64}@)
 [a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*
@ (?:(?=[a-z0-9-]{1,63}\.)[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+
 {2}(?=[a-z0-9-]{1,63}\z)[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\z",
    )
    .unwrap()
});

#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct Email(ArrayString<15>);

impl TryFrom<&str> for Email {
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

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}