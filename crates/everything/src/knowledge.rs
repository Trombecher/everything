use crate::{Error, Object};

#[derive(Clone, Debug, Copy)]
pub struct Knowledge<'a>(UncheckedKnowlege<'a>);

impl<'a> Knowledge<'a> {
    #[must_use]
    pub fn new(uk: UncheckedKnowlege<'a>) -> Result<Self, Error> {
        uk.check().map(|()| Self(uk))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Association {
    pub target: Object,
    pub tag: Object,
    pub value: Object,
}

#[derive(Clone, Debug, Copy)]
pub struct UncheckedKnowlege<'a> {
    pub associations: &'a [Association],
}

impl<'a> UncheckedKnowlege<'a> {
    fn check(&self) -> Result<(), Error> {
        Ok(())
    }

    fn eval(&self) -> Object {
        todo!()
    }
}
