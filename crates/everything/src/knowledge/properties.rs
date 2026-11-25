use imbl::{OrdSet, ordset::ConsumingIter, shared_ptr::DefaultSharedPtr};

use crate::model::{Composite, CompositeProperties, Object};

/// The properties of a subject, i.e. the objects that the subject has.
///
/// Unlike [`Composite`], this is not a model object: it is not canonicalized or
/// specialized and it is not itself a property. It is the working container for
/// the statements about a subject, so it can be inserted into and removed from
/// in logarithmic time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Properties {
    objects: OrdSet<Object>,
}

impl Default for Properties {
    fn default() -> Self {
        Self::empty()
    }
}

impl Properties {
    /// Returns the empty properties.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            objects: OrdSet::new(),
        }
    }

    /// Adds the given object to these properties.
    ///
    /// Adding an object that is already present does nothing.
    pub fn insert(&mut self, object: Object) {
        self.objects.insert(object);
    }

    /// Removes the given object from these properties.
    ///
    /// Removing an object that is not present does nothing.
    pub fn remove(&mut self, object: &Object) {
        self.objects.remove(object);
    }

    /// Returns `true` iff the given object is one of these properties.
    #[must_use]
    pub fn has(&self, object: &Object) -> bool {
        self.objects.contains(object)
    }

    /// Returns `true` iff there are no properties.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// The number of properties.
    #[must_use]
    pub fn size(&self) -> u64 {
        u64::try_from(self.objects.len()).unwrap_or(u64::MAX)
    }
}

impl FromIterator<Object> for Properties {
    fn from_iter<T: IntoIterator<Item = Object>>(iter: T) -> Self {
        Self {
            objects: iter.into_iter().collect(),
        }
    }
}

impl<const N: usize> From<[Object; N]> for Properties {
    fn from(objects: [Object; N]) -> Self {
        objects.into_iter().collect()
    }
}

impl IntoIterator for Properties {
    type Item = Object;
    type IntoIter = PropertiesIter;

    fn into_iter(self) -> Self::IntoIter {
        PropertiesIter(self.objects.into_iter())
    }
}

/// An iterator over a [`Properties`].
#[derive(Clone)]
pub struct PropertiesIter(ConsumingIter<Object, DefaultSharedPtr>);

impl Iterator for PropertiesIter {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// Iterates over the properties of an object.
///
/// Whether the properties come from the statements about an atom or from the
/// members of a composite is an implementation detail: callers only observe the
/// properties, as both kinds of object are queried identically at runtime.
#[derive(Clone)]
pub struct ObjectProperties(ObjectPropertiesSource);

#[derive(Clone)]
enum ObjectPropertiesSource {
    Properties(PropertiesIter),
    Members(CompositeProperties),
}

impl ObjectProperties {
    pub(crate) fn properties(properties: Properties) -> Self {
        Self(ObjectPropertiesSource::Properties(properties.into_iter()))
    }

    pub(crate) fn members(composite: Composite) -> Self {
        Self(ObjectPropertiesSource::Members(composite.into_iter()))
    }
}

impl Iterator for ObjectProperties {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            ObjectPropertiesSource::Properties(properties) => properties.next(),
            ObjectPropertiesSource::Members(properties) => properties.next(),
        }
    }
}
