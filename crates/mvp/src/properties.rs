use std::fmt;

use crate::objects::Id;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Property {
    pub tag: Id,
    pub value: Id,
}

impl fmt::Debug for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.tag)
            .field(&self.value)
            .finish()
    }
}
