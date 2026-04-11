use std::{fmt, hash::Hash};

use crate::{GlobalRegistry, Registry, objects::Object};

/// A property has a tag and a value.
#[derive(Clone)]
pub struct Property<R: Registry = GlobalRegistry> {
    pub tag: Object<R>,
    pub value: Object<R>,
}

impl<R: Registry> fmt::Debug for Property<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.tag)
            .field(&self.value)
            .finish()
    }
}

impl<R: Registry> PartialEq for Property<R> {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag && self.value == other.value
    }
}

impl<R: Registry> Eq for Property<R> {}

impl<R: Registry> PartialOrd for Property<R> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<R: Registry> Ord for Property<R> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.tag.cmp(&other.tag).then(self.value.cmp(&other.value))
    }
}

impl<R: Registry> Hash for Property<R> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
        self.value.hash(state);
    }
}
