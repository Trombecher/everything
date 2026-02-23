use crate::Object;

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Statement {
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
pub struct StatementPattern {
    pub target: ObjectOrAny,
    pub tag: ObjectOrAny,
    pub value: ObjectOrAny,
}

impl From<Statement> for StatementPattern {
    fn from(Statement { target, tag, value }: Statement) -> Self {
        Self {
            target: ObjectOrAny::Object(target),
            tag: ObjectOrAny::Object(tag),
            value: ObjectOrAny::Object(value),
        }
    }
}
