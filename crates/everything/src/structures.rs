use std::sync::Arc;

use crate::Object;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Property {
    tag: Object,
    value: Object,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Structure(Arc<[Property]>);

impl Structure {
    pub unsafe fn new_unchecked(properties: &[Property]) -> Self {
        Self(Arc::from(properties))
    }

    pub fn new(properties: &mut [Property]) -> Self {
        properties.sort();
        unsafe { Self::new_unchecked(properties) }
    }
}
