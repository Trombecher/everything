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
    #[must_use]
    pub fn new(properties: &mut [Property]) -> Self {
        properties.sort();
        Self(Arc::from(properties))
    }
}
