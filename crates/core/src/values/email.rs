use arrayvec::ArrayString;

#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct Email(ArrayString<15>);

impl TryFrom<ArrayString<15>> for Email {
    type Error = ();

    fn try_from(value: ArrayString<15>) -> Result<Self, Self::Error> {
        // TODO: validation
        Ok(Self(value))
    }
}