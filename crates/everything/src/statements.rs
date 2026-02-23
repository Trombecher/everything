use std::fmt::Debug;

use crate::Object;

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Statement {
    pub target: Object,
    pub tag: Object,
    pub value: Object,
}

#[derive(Clone, PartialEq)]
pub enum ObjectOrAny {
    Object(Object),
    Any,
}

impl Debug for ObjectOrAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Object(o) => Object::fmt(o, f),
            Self::Any => write!(f, "_"),
        }
    }
}

/// Something that is provable.
pub struct StatementPattern {
    pub target: ObjectOrAny,
    pub tag: ObjectOrAny,
    pub value: ObjectOrAny,
}

impl Debug for StatementPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.target)
            .field(&self.tag)
            .field(&self.value)
            .finish()
    }
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
