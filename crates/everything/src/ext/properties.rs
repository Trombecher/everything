use everything_structures::{Abstract, Object, Property};

use crate::ext::AbstractExt;

pub trait PropertyExt {
    /// Creates a new `(@CONTAINS, value)` property.
    #[must_use]
    fn new_contains(value: Object) -> Self;

    #[must_use]
    fn new_statement_subject(value: Object) -> Self;

    #[must_use]
    fn new_statement_tag(value: Object) -> Self;

    #[must_use]
    fn new_statement_value(value: Object) -> Self;
}

impl PropertyExt for Property {
    fn new_contains(value: Object) -> Self {
        Self {
            tag: Abstract::CONTAINS.into(),
            value,
        }
    }

    fn new_statement_subject(value: Object) -> Self {
        Self {
            tag: Abstract::STATEMENT_SUBJECT.into(),
            value,
        }
    }

    fn new_statement_tag(value: Object) -> Self {
        Self {
            tag: Abstract::STATEMENT_TAG.into(),
            value,
        }
    }

    fn new_statement_value(value: Object) -> Self {
        Self {
            tag: Abstract::STATEMENT_VALUE.into(),
            value,
        }
    }
}
