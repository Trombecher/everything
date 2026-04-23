use everything_structures::{Abstract, Object, Property};

use crate::ext::AbstractExt;

pub trait PropertyExt {
    /// Creates a new `(@CONTAINS, value)` property.
    fn new_contains(value: Object) -> Self;
}

impl PropertyExt for Property {
    fn new_contains(value: Object) -> Self {
        Self {
            tag: Object::Abstract(Abstract::CONTAINS),
            value,
        }
    }
}
