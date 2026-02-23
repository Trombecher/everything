use std::sync::Arc;

use crate::Object;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct AxiomaticProperty {
    pub tag: Object,
    pub value: Object,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Structure(Arc<[AxiomaticProperty]>);

impl Structure {
    #[must_use]
    pub fn new(properties: &mut [AxiomaticProperty]) -> Self {
        properties.sort();
        Self(Arc::from(properties))
    }

    pub fn properties(&self) -> &[AxiomaticProperty] {
        &self.0
    }
}
