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

/// An arbitrary structure that is NOT exactly a natural number,
/// an array of characters, or binary data.
#[derive(Clone)]
pub struct AnyStructure {
    pub(super) properties: Arc<[Property]>,
}

impl AnyStructure {
    /// Checks if the AnyStructure has this property.
    #[must_use]
    pub fn has(&self, property: &Property) -> bool {
        self.properties.binary_search(property).is_ok()
    }

    #[must_use]
    pub fn has_by_ref(&self, tag: &Object, value: &Object) -> bool {
        self.properties
            .binary_search_by(|property| match property.tag.cmp(tag) {
                Ordering::Equal => property.value.cmp(value),
                ordering => ordering,
            })
            .is_ok()
    }

    /// Returns an iterator over all values that this tag has
    /// in this AnyStructure.
    #[must_use]
    pub fn values<'props>(&'props self, tag: Object) -> ValuesIter<'props> {
        let properties = self.as_ref();
        let start = properties.partition_point(|property| property.tag < tag);

        ValuesIter {
            props: properties[start..].iter(),
            tag,
            done: false,
        }
    }

    /// Returns an iterator over all tags that this value has
    /// in this AnyStructure.
    pub fn tags(&self, value: &Object) -> impl Iterator<Item = &Object> {
        self.as_ref()
            .iter()
            .filter_map(move |property| (&property.value == value).then_some(&property.tag))
    }

    /// Determines if `self` is a subset of `other` by checking
    /// if `other` has every property of `self`.
    #[must_use]
    pub fn is_subset_of(&self, other: &Self) -> bool {
        self.as_ref().iter().all(|property| other.has(property))
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

#[derive(Clone)]
pub struct ValuesIter<'props> {
    props: slice::Iter<'props, Property>,
    tag: Object,
    done: bool,
}

impl<'props> Iterator for ValuesIter<'props> {
    type Item = &'props Object;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else if let Some(property) = self.props.next() {
            if property.tag == self.tag {
                Some(&property.value)
            } else {
                self.done = true;

                None
            }
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
