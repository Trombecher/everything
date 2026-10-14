use std::{cmp::Ordering, fmt::Debug, sync::Arc};

use crate::model::{Object, SetApi};

/// A set that is no specialization.
#[derive(Clone)]
pub struct AnySet {
    objects: Arc<[Object]>,
}

impl SetApi for AnySet {
    fn size(&self) -> u64 {
        self.as_slice().len() as u64
    }

    fn has(&self, object: &Object) -> bool {
        self.objects.binary_search(object).is_ok()
    }
}

impl AnySet {
    /// DO NOT EXPOSE THIS!
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub(super) fn new(objects: &[Object]) -> Self {
        let mut objects: Arc<[Object]> = Arc::from(objects);
        Arc::get_mut(&mut objects).unwrap().sort();

        Self { objects }
    }

    pub(super) fn new_move<const N: usize>(mut objects: [Object; N]) -> Self {
        objects.sort();

        Self {
            objects: Arc::new(objects),
        }
    }

    #[must_use]
    pub fn as_slice(&self) -> &[Object] {
        &self.objects
    }
}

impl Debug for AnySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.objects.iter()).finish()
    }
}

impl PartialEq for AnySet {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.objects, &other.objects) || self.objects == other.objects
    }
}

impl Eq for AnySet {}

impl PartialOrd for AnySet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AnySet {
    fn cmp(&self, other: &Self) -> Ordering {
        if Arc::ptr_eq(&self.objects, &other.objects) {
            Ordering::Equal
        } else {
            self.objects.cmp(&other.objects)
        }
    }
}

impl IntoIterator for AnySet {
    type Item = Object;
    type IntoIter = AnySetValues;

    fn into_iter(self) -> Self::IntoIter {
        AnySetValues {
            set: self,
            next_index: 0,
        }
    }
}

#[derive(Clone)]
pub struct AnySetValues {
    set: AnySet,
    next_index: usize,
}

impl Iterator for AnySetValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.set
            .as_slice()
            .get(self.next_index)
            .cloned()
            .inspect(|_| self.next_index += 1)
    }
}
