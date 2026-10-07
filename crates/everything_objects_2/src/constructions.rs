use std::fmt::Debug;

use crate::Object;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Construction {
    pub constructor: Box<Object>,
    pub value: Box<Object>,
}

impl Debug for Construction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.constructor)
            .field(&self.value)
            .finish()
    }
}
