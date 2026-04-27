use everything_structures::{Abstract, Object, Property};

use crate::ext::AbstractExt;

pub trait PropertyExt {
    /// Creates a new `(@CONTAINS, value)` property.
    #[must_use]
    fn new_contains(value: Object) -> Self;

    #[must_use]
    fn new_node_add_left(value: Object) -> Self;

    #[must_use]
    fn new_node_add_right(value: Object) -> Self;

    #[must_use]
    fn new_node_parameter(depth: usize) -> Self;

    #[must_use]
    fn new_statement_subject(value: Object) -> Self;

    #[must_use]
    fn new_statement_tag(value: Object) -> Self;

    #[must_use]
    fn new_statement_value(value: Object) -> Self;

    #[must_use]
    fn new_computed(body: Object) -> Self;

    #[must_use]
    fn new_node_equal(node: Object) -> Self;

    #[must_use]
    fn new_node_and(node: Object) -> Self;

    #[must_use]
    fn new_node_or(node: Object) -> Self;

    #[must_use]
    fn new_node_xor(node: Object) -> Self;

    #[must_use]
    fn new_node_not(node: Object) -> Self;

    #[must_use]
    fn new_node_literal(object: Object) -> Self;

    #[must_use]
    fn new_node_query(query: Object) -> Self;

    #[must_use]
    fn new_node_count(object: Object) -> Self;
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

    fn new_computed(body: Object) -> Self {
        Self {
            tag: Abstract::COMPUTED.into(),
            value: body,
        }
    }

    fn new_node_equal(node: Object) -> Self {
        Self {
            tag: Abstract::NODE_EQUAL.into(),
            value: node,
        }
    }

    fn new_node_and(node: Object) -> Self {
        Self {
            tag: Abstract::NODE_AND.into(),
            value: node,
        }
    }

    fn new_node_or(node: Object) -> Self {
        Self {
            tag: Abstract::NODE_OR.into(),
            value: node,
        }
    }

    fn new_node_xor(node: Object) -> Self {
        Self {
            tag: Abstract::NODE_XOR.into(),
            value: node,
        }
    }

    fn new_node_not(node: Object) -> Self {
        Self {
            tag: Abstract::NODE_NOT.into(),
            value: node,
        }
    }

    fn new_node_literal(object: Object) -> Self {
        Self {
            tag: Abstract::NODE_LITERAL.into(),
            value: object,
        }
    }

    fn new_node_query(query: Object) -> Self {
        Self {
            tag: Abstract::NODE_QUERY.into(),
            value: query,
        }
    }

    fn new_node_count(object: Object) -> Self {
        Self {
            tag: Abstract::NODE_COUNT.into(),
            value: object,
        }
    }
}
