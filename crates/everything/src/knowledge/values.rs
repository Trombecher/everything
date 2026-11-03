use imbl::{OrdSet, ordset::ConsumingIter, shared_ptr::DefaultSharedPtr};

use crate::model::{Object, Set, SetValues};

/// The values of a subject, i.e. the objects that the subject has.
///
/// Unlike [`Set`], this is not a model object: it is not canonicalized or
/// specialized and it is not itself a value. It is the working container for
/// the statements about a subject, so it can be inserted into and removed from
/// in logarithmic time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Values {
    objects: OrdSet<Object>,
}

impl Default for Values {
    fn default() -> Self {
        Self::empty()
    }
}

impl Values {
    /// Returns the empty values.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            objects: OrdSet::new(),
        }
    }

    /// Adds the given object to these values.
    ///
    /// Adding an object that is already present does nothing.
    pub fn insert(&mut self, object: Object) {
        self.objects.insert(object);
    }

    /// Removes the given object from these values.
    ///
    /// Removing an object that is not present does nothing.
    pub fn remove(&mut self, object: &Object) {
        self.objects.remove(object);
    }

    /// Returns `true` iff the given object is one of these values.
    #[must_use]
    pub fn has(&self, object: &Object) -> bool {
        self.objects.contains(object)
    }

    /// Returns `true` iff there are no values.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// The number of values.
    #[must_use]
    pub fn size(&self) -> u64 {
        u64::try_from(self.objects.len()).unwrap_or(u64::MAX)
    }
}

impl FromIterator<Object> for Values {
    fn from_iter<T: IntoIterator<Item = Object>>(iter: T) -> Self {
        Self {
            objects: iter.into_iter().collect(),
        }
    }
}

impl<const N: usize> From<[Object; N]> for Values {
    fn from(objects: [Object; N]) -> Self {
        objects.into_iter().collect()
    }
}

impl IntoIterator for Values {
    type Item = Object;
    type IntoIter = ValuesValues;

    fn into_iter(self) -> Self::IntoIter {
        ValuesValues(self.objects.into_iter())
    }
}

/// An iterator over the values of a [`Values`].
#[derive(Clone)]
pub struct ValuesValues(ConsumingIter<Object, DefaultSharedPtr>);

impl Iterator for ValuesValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// Iterates over the values of an object.
///
/// Whether the values come from the statements about an atom or from the
/// members of a set is an implementation detail: callers only observe the
/// values, as both kinds of object are queried identically at runtime.
#[derive(Clone)]
pub struct ObjectValues(ObjectValuesSource);

#[derive(Clone)]
enum ObjectValuesSource {
    Properties(ValuesValues),
    Members(SetValues),
}

impl ObjectValues {
    pub(crate) fn properties(values: Values) -> Self {
        Self(ObjectValuesSource::Properties(values.into_iter()))
    }

    pub(crate) fn members(set: Set) -> Self {
        Self(ObjectValuesSource::Members(set.into_iter()))
    }

    pub(crate) fn empty() -> Self {
        Self::properties(Values::empty())
    }
}

impl Iterator for ObjectValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            ObjectValuesSource::Properties(values) => values.next(),
            ObjectValuesSource::Members(values) => values.next(),
        }
    }
}
