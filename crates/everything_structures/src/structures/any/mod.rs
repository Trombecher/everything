#[cfg(test)]
mod tests;

use core::slice;
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    sync::Arc,
};

use crate::{Object, Property, structures::registry};

/// An arbitrary structure with no specialization.
#[derive(Clone)]
pub struct AnyStructure {
    pub(super) properties: Arc<[Property]>,
}

impl AnyStructure {
    /// Returns an iterator over all values that this tag has
    /// in `self`.
    pub fn values<'properties>(&'properties self, tag: Object) -> AnyStructureValues<'properties> {
        let start = self
            .properties
            .partition_point(|property| property.tag < tag);

        AnyStructureValues {
            props: self.properties[start..].iter(),
            tag,
            done: false,
        }
    }

    /// Returns an iterator of all tags that this value has in `self`.
    pub fn tags<'properties>(&'properties self, value: Object) -> AnyStructureTags<'properties> {
        AnyStructureTags {
            properties: self.properties.iter(),
            value,
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
        let ref_count = Arc::strong_count(&self.properties);

        if ref_count == 2 {
            // We and the registry are the only ones
            // that have a ref. When removing this AnyStructure
            // from the registry, `self` will be the
            // only reference and thus will deallocate
            // after drop.

            registry::remove(self);
        } else if ref_count == 1 {
            unreachable!("the registry does not know of us!")
        }
    }
}

/// Iterator over values for a tag in an [AnyStructure].
#[derive(Clone)]
pub struct AnyStructureValues<'props> {
    props: slice::Iter<'props, Property>,
    tag: Object,
    done: bool,
}

impl<'props> Iterator for AnyStructureValues<'props> {
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

#[derive(Clone)]
pub struct AnyStructureTags<'properties> {
    properties: slice::Iter<'properties, Property>,
    value: Object,
}

impl<'properties> Iterator for AnyStructureTags<'properties> {
    type Item = &'properties Object;

    fn next(&mut self) -> Option<Self::Item> {
        for next in &mut self.properties {
            if next.value != self.value {
                continue;
            }

            return Some(&next.tag);
        }

        None
    }
}
