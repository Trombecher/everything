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

use crate::{GlobalRegistry, Object, Property};

/// An arbitrary structure that is NOT exactly a natural number,
/// an array of characters, or binary data.
#[derive(Clone)]
pub struct AnyStructure {
    pub(super) properties: Option<Arc<[Property]>>,
}

impl AnyStructure {
    #[must_use]
    pub const unsafe fn new_unchecked(properties: Option<Arc<[Property]>>) -> Self {
        Self { properties }
    }

    #[must_use]
    #[inline]
    pub fn new(properties: &mut [Property]) -> Self {
        Self::EMPTY.add(properties)
    }

    /// Checks if the AnyStructure has this property.
    #[must_use]
    pub fn has(&self, property: &Property) -> bool {
        match &self.properties {
            None => false,
            Some(properties) => properties.binary_search(property).is_ok(),
        }
    }

    #[must_use]
    pub fn has_by_ref(&self, tag: &Object, value: &Object) -> bool {
        match &self.properties {
            None => false,
            Some(properties) => properties
                .binary_search_by(|property| match property.tag.cmp(tag) {
                    Ordering::Equal => property.value.cmp(value),
                    ordering => ordering,
                })
                .is_ok(),
        }
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

    /// Merges the properties of `self` and `other` into a new AnyStructure.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut add_properties = Box::clone_from_ref(other.as_ref());

        self.add(&mut add_properties)
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
        let mut debug = f.debug_set();

        if let Some(props) = &self.properties {
            for prop in props.iter() {
                debug.entry(prop);
            }
        }

        debug.finish()
    }
}

impl AsRef<[Property]> for AnyStructure {
    fn as_ref(&self) -> &[Property] {
        match &self.properties {
            None => &[],
            Some(x) => x,
        }
    }
}

impl Hash for AnyStructure {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.properties.as_ref().map(Arc::as_ptr).hash(state);
    }
}

impl PartialEq for AnyStructure {
    fn eq(&self, other: &Self) -> bool {
        match (&self.properties, &other.properties) {
            (None, None) => true,
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
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
        match (&self.properties, &other.properties) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(a), Some(b)) => Arc::as_ptr(a)
                .cast::<()>()
                .cmp(&Arc::as_ptr(b).cast::<()>()),
        }
    }
}

impl Drop for AnyStructure {
    fn drop(&mut self) {
        if let Some(props) = &self.properties
            && Arc::strong_count(props) == 2
        {
            // We and the registry are the only ones
            // that have a ref. When removing this AnyStructure
            // from the registry, `self` will be the
            // only reference and thus will deallocate
            // after drop.

            GlobalRegistry.remove(self);
        }
    }
}
