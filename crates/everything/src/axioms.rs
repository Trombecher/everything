use crate::Object;

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Fact {
    pub target: Object,
    pub tag: Object,
    pub value: Object,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectOrAny {
    Object(Object),
    Any,
}

/// Something that is provable.
pub struct FactPattern {
    pub target: ObjectOrAny,
    pub tag: ObjectOrAny,
    pub value: ObjectOrAny,
}

impl From<Fact> for FactPattern {
    fn from(Fact { target, tag, value }: Fact) -> Self {
        Self {
            target: ObjectOrAny::Object(target),
            tag: ObjectOrAny::Object(tag),
            value: ObjectOrAny::Object(value),
        }
    }
}
