use std::{fmt::Debug, mem};

use crate::model::{CompositeApi, Construction, Object};

/// Specialization for composites that only contain one construction.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OneConstructionComposite {
    construction: Construction,
}

impl Debug for OneConstructionComposite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entry(&self.construction).finish()
    }
}

impl OneConstructionComposite {
    /// Creates a composite that consists of exactly the given construction.
    #[must_use]
    pub(super) fn new(construction: Construction) -> Self {
        Self { construction }
    }

    /// Tries to create a new [`OneConstructionComposite`] from the given
    /// slice of objects.
    ///
    /// If it succeeded, then `objects` is arbitrary.
    ///
    /// This must only be called during specialization, after all more
    /// specific one-construction specializations (e.g. integer and
    /// character composites) have been tried. Otherwise it would hoist
    /// constructions that are represented more optimally by those
    /// specializations and thus create a non-canonical composite.
    ///
    /// DO NOT EXPOSE THIS!
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // this function does not panic!
    pub(super) fn specialize_from_moving(objects: &mut [Object]) -> Option<Self> {
        match objects {
            [Object::Construction(_)] => Some(Self {
                construction: mem::take(objects.first_mut().unwrap())
                    .destructure_construction()
                    .unwrap(),
            }),
            _ => None,
        }
    }
}

impl CompositeApi for OneConstructionComposite {
    fn size(&self) -> u64 {
        1
    }

    fn has(&self, object: &Object) -> bool {
        matches!(object, Object::Construction(construction2) if construction2 == &self.construction)
    }
}

impl IntoIterator for OneConstructionComposite {
    type Item = Object;
    type IntoIter = OneConstructionCompositeValues;

    fn into_iter(self) -> Self::IntoIter {
        OneConstructionCompositeValues {
            to_yield: Some(self.construction),
        }
    }
}

#[derive(Clone)]
pub struct OneConstructionCompositeValues {
    to_yield: Option<Construction>,
}

impl Iterator for OneConstructionCompositeValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.to_yield.take().map(Object::Construction)
    }
}
