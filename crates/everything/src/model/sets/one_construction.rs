use std::{fmt::Debug, mem};

use crate::model::{Construction, Object, SetApi};

/// Specialization for sets that only contain one construction.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OneConstructionSet {
    construction: Construction,
}

impl Debug for OneConstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entry(&self.construction).finish()
    }
}

impl OneConstructionSet {
    /// Tries to create a new [`OneConstructionSet`] from the given
    /// slice of objects.
    ///
    /// If it succeeded, then `objects` is arbitrary.
    ///
    /// This must only be called during specialization, after all more
    /// specific one-construction specializations (e.g. integer and
    /// character sets) have been tried. Otherwise it would hoist
    /// constructions that are represented more optimally by those
    /// specializations and thus create a non-canonical set.
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

impl SetApi for OneConstructionSet {
    fn size(&self) -> u64 {
        1
    }

    fn has(&self, object: &Object) -> bool {
        matches!(object, Object::Construction(construction2) if construction2 == &self.construction)
    }
}

impl IntoIterator for OneConstructionSet {
    type Item = Object;
    type IntoIter = OneConstructionSetValues;

    fn into_iter(self) -> Self::IntoIter {
        OneConstructionSetValues {
            to_yield: Some(self.construction),
        }
    }
}

#[derive(Clone)]
pub struct OneConstructionSetValues {
    to_yield: Option<Construction>,
}

impl Iterator for OneConstructionSetValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.to_yield.take().map(Object::Construction)
    }
}
