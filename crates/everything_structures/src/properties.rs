use std::fmt;

use crate::objects::Object;

/// A property has a tag and a value.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Property {
    pub tag: Object,
    pub value: Object,
}

impl fmt::Debug for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.tag)
            .field(&self.value)
            .finish()
    }
}
