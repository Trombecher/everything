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

use crate::{GlobalRegistry, Object, Property, structures::registries::Registry};

#[derive(Clone)]
pub struct AnyStructure<R: Registry = GlobalRegistry> {
    pub(super) propeties: Option<Arc<[Property<R>]>>,
    pub(super) registry: R,
}

impl AnyStructure<GlobalRegistry> {
    pub const EMPTY: Self = Self {
        propeties: None,
        registry: GlobalRegistry,
    };

    #[must_use]
    #[inline]
    pub fn new(properties: &mut [Property]) -> Self {
        Self::EMPTY.add(properties)
    }
}

impl<R: Registry> AnyStructure<R> {
    #[inline]
    #[must_use]
    pub fn empty(registry: R) -> Self {
        Self {
            propeties: None,
            registry,
        }
    }

    /// Checks if the AnyStructure has this property.
    #[must_use]
    pub fn has(&self, property: &Property<R>) -> bool {
        match &self.propeties {
            None => false,
            Some(properties) => properties.binary_search(property).is_ok(),
        }
    }

    #[must_use]
    pub fn has_by_ref(&self, tag: &Object<R>, value: &Object<R>) -> bool {
        match &self.propeties {
            None => false,
            Some(properties) => properties
                .binary_search_by(|property| match property.tag.cmp(tag) {
                    Ordering::Equal => property.value.cmp(value),
                    ordering => ordering,
                })
                .is_ok(),
        }
    }

    /// Creates a new AnyStructure from the given properties
    /// by adding them to the empty AnyStructure.
    #[must_use]
    #[inline]
    pub fn new_in(properties: &mut [Property<R>], registry: R) -> Self {
        Self::empty(registry).add(properties)
    }

    /// Adds the given properties to this AnyStructure.
    #[inline]
    #[must_use]
    pub fn add(&self, properties: &mut [Property<R>]) -> Self {
        self.change(&mut [], properties)
    }

    /// Removes the given properties from this AnyStructure.
    #[inline]
    #[must_use]
    pub fn remove(&self, properties: &mut [Property<R>]) -> Self {
        self.change(properties, &mut [])
    }

    /// Modifies this AnyStructure by adding and removing properties.
    /// Returns the modified AnyStructure.
    ///
    /// The properties need to be mutable because this method needs to
    /// reorder and dedup changes in-place to avoid unneccessary
    /// allocations.
    ///
    /// Note that first all indicated properties are removed from
    /// the AnyStructure and then all indicated properties are added.
    #[must_use]
    pub fn change(
        &self,
        remove_properties: &mut [Property<R>],
        add_properties: &mut [Property<R>],
    ) -> AnyStructure<R> {
        self.registry
            .resolve(self, remove_properties, add_properties)
    }

    /// Returns an iterator over all values that this tag has
    /// in this AnyStructure.
    #[must_use]
    pub fn values<'props>(&'props self, tag: Object<R>) -> ValuesIter<'props, R> {
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
    pub fn tags(&self, value: &Object<R>) -> impl Iterator<Item = &Object<R>> {
        self.as_ref()
            .iter()
            .filter_map(move |property| (&property.value == value).then_some(&property.tag))
    }

    /// Merges the properties of `self` and `other` into a new AnyStructure.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self
    where
        R: Clone,
    {
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
pub struct ValuesIter<'props, R: Registry = GlobalRegistry> {
    props: slice::Iter<'props, Property<R>>,
    tag: Object<R>,
    done: bool,
}

impl<'props, R: Registry> Iterator for ValuesIter<'props, R> {
    type Item = &'props Object<R>;

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

impl<R: Registry> fmt::Debug for AnyStructure<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_set();

        if let Some(props) = &self.propeties {
            for prop in props.iter() {
                debug.entry(prop);
            }
        }

        debug.finish()
    }
}

impl<R: Registry> AsRef<[Property<R>]> for AnyStructure<R> {
    fn as_ref(&self) -> &[Property<R>] {
        match &self.propeties {
            None => &[],
            Some(x) => x,
        }
    }
}

impl<R: Registry> Hash for AnyStructure<R> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.propeties.as_ref().map(Arc::as_ptr).hash(state);
    }
}

impl<R: Registry> PartialEq for AnyStructure<R> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.propeties, &other.propeties) {
            (None, None) => true,
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl<R: Registry> Eq for AnyStructure<R> {}

impl<R: Registry> PartialOrd for AnyStructure<R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<R: Registry> Ord for AnyStructure<R> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.propeties, &other.propeties) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(a), Some(b)) => Arc::as_ptr(a)
                .cast::<()>()
                .cmp(&Arc::as_ptr(b).cast::<()>()),
        }
    }
}

impl<R: Registry> Drop for AnyStructure<R> {
    fn drop(&mut self) {
        if let Some(props) = &self.propeties
            && Arc::strong_count(props) == 2
        {
            // We and the registry are the only ones
            // that have a ref. When removing this AnyStructure
            // from the registry, `self` will be the
            // only reference and thus will deallocate
            // after drop.

            self.registry.remove(self);
        }
    }
}
