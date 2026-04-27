use everything_structures::{Abstract, Object, Property};

use crate::ext::AbstractExt;

pub trait PropertyExt {
    /// Creates a new `(@CONTAINS, value)` property.
    fn new_contains(value: Object) -> Self;

    fn new_node_add_left(value: Object) -> Self;

    fn new_node_add_right(value: Object) -> Self;
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
}
