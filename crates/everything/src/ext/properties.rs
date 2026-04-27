use everything_structures::{Abstract, Object, Property};

use crate::ext::AbstractExt;

pub trait PropertyExt {
    /// Creates a new `(@CONTAINS, value)` property.
    fn new_contains(value: Object) -> Self;

    fn new_node_add_left(value: Object) -> Self;

    fn new_node_add_right(value: Object) -> Self;

    fn new_node_parameter(depth: usize) -> Self;

    fn new_statement_subject(value: Object) -> Self;

    fn new_statement_tag(value: Object) -> Self;

    fn new_statement_value(value: Object) -> Self;
}

impl PropertyExt for Property {
    fn new_contains(value: Object) -> Self {
        Self {
            tag: Abstract::CONTAINS.into(),
            value,
        }
    }

    fn new_node_add_left(value: Object) -> Self {
        Self {
            tag: Abstract::NODE_ADD_LEFT.into(),
            value,
        }
    }

    fn new_node_add_right(value: Object) -> Self {
        Self {
            tag: Abstract::NODE_ADD_RIGHT.into(),
            value,
        }
    }

    fn new_node_parameter(depth: usize) -> Self {
        Self {
            tag: Abstract::NODE_PARAMETER.into(),
            value: Object::new_natural_number(depth as u128),
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
