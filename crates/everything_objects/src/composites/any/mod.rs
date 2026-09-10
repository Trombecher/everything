#[cfg(test)]
mod tests;

use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    sync::Arc,
};

use crate::{Object, Property, composites::registry};

/// An arbitrary composite object with no specialization.
#[derive(Clone)]
pub struct AnyComposite {
    pub(super) properties: Arc<[Property]>,

    /// This is not the hash returned by [`AnyComposite::hash`] but
    /// a hash of all properties + the number of properties.
    ///
    /// This hash is used by the registry.
    pub(super) registry_hash: u64,
}

impl AnyComposite {
    /// Returns an iterator over all values that this tag has
    /// in `self`.
    pub fn values(&self, tag: Object) -> AnyCompositeValues {
        AnyCompositeValues::new(self.clone(), tag)
    }

    /// Returns an iterator of all tags that this value has in `self`.
    pub fn tags(&self, value: Object) -> AnyCompositeTags {
        AnyCompositeTags {
            properties: self.properties(),
            value,
        }
    }

    pub fn properties(&self) -> AnyCompositeProperties {
        AnyCompositeProperties::new(self.clone())
    }

    #[must_use]
    pub fn has(&self, tag: &Object, value: &Object) -> bool {
        self.properties
            .binary_search_by(|property| {
                property
                    .tag
                    .cmp(tag)
                    .then_with(|| property.value.cmp(value))
            })
            .is_ok()
    }
}

impl fmt::Debug for AnyComposite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.properties.iter()).finish()
    }
}

impl AsRef<[Property]> for AnyComposite {
    fn as_ref(&self) -> &[Property] {
        &self.properties
    }
}

impl Hash for AnyComposite {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.properties).hash(state);
    }
}

impl PartialEq for AnyComposite {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.properties, &other.properties)
    }
}

impl Eq for AnyComposite {}

impl PartialOrd for AnyComposite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AnyComposite {
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

impl Drop for AnyComposite {
    fn drop(&mut self) {
        let ref_count = Arc::strong_count(&self.properties);

        if ref_count == 2 {
            // We and the registry are the only ones
            // that have a ref. When removing this AnyComposite
            // from the registry, `self` will be the
            // only reference and thus will deallocate
            // after drop.

            registry::remove(self);
        } else if ref_count == 1 {
            unreachable!("the registry does not know of us!")
        }
    }
}

/// Iterator over values for a tag in an [AnyComposite].
/// It is a `FilterMap` over [`AnyCompositeProperties`].
#[derive(Clone)]
pub struct AnyCompositeValues {
    properties: AnyCompositeProperties,
    tag: Object,
}

impl AnyCompositeValues {
    /// Creates a new iterator over values for a `tag` in the given
    /// `subject`.
    pub fn new(subject: AnyComposite, tag: Object) -> Self {
        Self {
            properties: AnyCompositeProperties::new_starting_from_tag(subject, tag.clone()),
            tag,
        }
    }
}

impl Iterator for AnyCompositeValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.properties
            .find_map(|property| (property.tag == self.tag).then_some(property.value))
    }
}

/// An iterator over all tags a value has on an [`AnyComposite`].
#[derive(Clone)]
pub struct AnyCompositeTags {
    pub properties: AnyCompositeProperties,
    pub value: Object,
}

impl Iterator for AnyCompositeTags {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.properties
            .find_map(|property| (property.value == self.value).then_some(property.tag))
    }
}

/// An iterator over all properties of an [`AnyComposite`].
#[derive(Clone)]
pub struct AnyCompositeProperties {
    subject: AnyComposite,
    index: usize,
}

impl AnyCompositeProperties {
    pub const fn new(subject: AnyComposite) -> Self {
        Self { subject, index: 0 }
    }

    /// Creates a new properties iterator which starts at the first occurance
    /// of the tag in the subject.
    ///
    /// You can use this function to implement efficient value iteration for
    /// a given tag by exploiting the fact that properties in an [`AnyComposite`]
    /// are lexicographically sorted.
    pub fn new_starting_from_tag(subject: AnyComposite, tag: Object) -> Self {
        let start = subject
            .properties
            .partition_point(|property| property.tag < tag);

        Self {
            subject,
            index: start,
        }
    }
}

impl Iterator for AnyCompositeProperties {
    type Item = Property;

    fn next(&mut self) -> Option<Self::Item> {
        self.subject.properties.get(self.index).map(|property| {
            self.index += 1;
            property.clone()
        })
    }
}
