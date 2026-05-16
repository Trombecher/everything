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
    fn new_node_equal_left(left: Object) -> Self;

    #[must_use]
    fn new_node_equal_right(right: Object) -> Self;

    #[must_use]
    fn new_node_and_left(left: Object) -> Self;

    #[must_use]
    fn new_node_and_right(right: Object) -> Self;

    #[must_use]
    fn new_node_or_left(left: Object) -> Self;

    #[must_use]
    fn new_node_or_right(right: Object) -> Self;

    #[must_use]
    fn new_node_xor_left(left: Object) -> Self;

    #[must_use]
    fn new_node_xor_right(right: Object) -> Self;

    #[must_use]
    fn new_node_union_left(left: Object) -> Self;

    #[must_use]
    fn new_node_union_right(right: Object) -> Self;

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

    fn new_node_union_left(left: Object) -> Self {
        Self {
            tag: Abstract::NODE_UNION_LEFT.into(),
            value: left,
        }
    }

    fn new_node_union_right(right: Object) -> Self {
        Self {
            tag: Abstract::NODE_UNION_RIGHT.into(),
            value: right,
        }
    }

    fn new_node_parameter(depth: usize) -> Self {
        Self {
            tag: Abstract::NODE_PARAMETER.into(),
            value: Object::new_integer(depth as i128),
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

    fn new_node_and_left(left: Object) -> Self {
        Self {
            tag: Abstract::NODE_AND_LEFT.into(),
            value: left,
        }
    }

    fn new_node_and_right(right: Object) -> Self {
        Self {
            tag: Abstract::NODE_AND_RIGHT.into(),
            value: right,
        }
    }

    fn new_node_equal_left(left: Object) -> Self {
        Self {
            tag: Abstract::NODE_EQUAL_LEFT.into(),
            value: left,
        }
    }

    fn new_node_equal_right(right: Object) -> Self {
        Self {
            tag: Abstract::NODE_EQUAL_RIGHT.into(),
            value: right,
        }
    }

    fn new_node_or_left(left: Object) -> Self {
        Self {
            tag: Abstract::NODE_OR_LEFT.into(),
            value: left,
        }
    }

    fn new_node_or_right(right: Object) -> Self {
        Self {
            tag: Abstract::NODE_OR_RIGHT.into(),
            value: right,
        }
    }

    fn new_node_xor_left(left: Object) -> Self {
        Self {
            tag: Abstract::NODE_XOR_LEFT.into(),
            value: left,
        }
    }

    fn new_node_xor_right(right: Object) -> Self {
        Self {
            tag: Abstract::NODE_XOR_RIGHT.into(),
            value: right,
        }
    }
}
