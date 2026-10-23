use std::fmt::Debug;

use crate::model::Object;

/// A construction `<constructor> <value>`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Construction {
    pub constructor: Box<Object>,
    pub value: Box<Object>,
}

impl Construction {
    /// Shorthand constructor for [`Construction`].
    #[must_use]
    pub fn new(constructor: Object, value: Object) -> Self {
        Self {
            constructor: Box::new(constructor),
            value: Box::new(value),
        }
    }
}

impl Debug for Construction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.constructor)
            .field(&self.value)
            .finish()
    }
}
