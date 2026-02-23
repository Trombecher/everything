use std::{fmt::Debug, sync::Arc};

use crate::Object;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct AxiomaticProperty {
    pub tag: Object,
    pub value: Object,
}

impl Debug for AxiomaticProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.tag)
            .field(&self.value)
            .finish()
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
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

impl Debug for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.0.iter()).finish()
    }
}
