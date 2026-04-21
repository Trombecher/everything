#[cfg(test)]
mod tests;

use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    num::NonZeroU128,
    slice,
    sync::Arc,
};

use crate::{Object, Property, structures::registry};

/// An arbitrary structure with no specialization.
#[derive(Clone)]
pub struct AnyStructure {
    pub(super) properties: Arc<[Property]>,
}

impl AnyStructure {
    /// Returns an iterator over all tags that this value has
    /// in this AnyStructure.
    pub fn tags(&self, value: &Object) -> impl Iterator<Item = &Object> {
        self.as_ref()
            .iter()
            .filter_map(move |property| (&property.value == value).then_some(&property.tag))
    }

    /// Returns the natural number representation of
    /// this structure if it has no additional props.
    pub fn exact_natural_number(&self) -> Option<NonZeroU128> {
        if let [Property { tag, value }] = self.as_ref()
            && tag == &Object::SUCCESSOR_OF
        {
            value
                .exact_natural_number()
                .and_then(|n| NonZeroU128::new(n.checked_add(1).unwrap()))
        } else {
            None
        }
    }
}

impl fmt::Debug for AnyStructure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.properties.fmt(f)
    }
}

impl AsRef<[Property]> for AnyStructure {
    fn as_ref(&self) -> &[Property] {
        &self.properties
    }
}

impl Hash for AnyStructure {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.properties).hash(state);
    }
}

impl PartialEq for AnyStructure {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.properties, &other.properties)
    }
}

impl Eq for AnyStructure {}

impl PartialOrd for AnyStructure {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AnyStructure {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reasons why we can do this:
        //
        // * all property allocations are non-zero in size.
        // * all property allocations are unique (via registry)
        //
        // Therefore, the pointers are unique.

        Arc::as_ptr(&self.properties)
            .cast::<()>()
            .cmp(&Arc::as_ptr(&other.properties).cast::<()>())
    }
}

impl Drop for AnyStructure {
    fn drop(&mut self) {
        if Arc::strong_count(&self.properties) == 2 {
            // We and the registry are the only ones
            // that have a ref. When removing this AnyStructure
            // from the registry, `self` will be the
            // only reference and thus will deallocate
            // after drop.

            registry::remove(self);
        }
    }
}
