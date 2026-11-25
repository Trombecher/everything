use std::{cmp::Ordering, fmt::Debug, sync::Arc};

use crate::model::{Atom, CompositeApi, Construction, Object};

use super::fmt_composite;

#[cfg(test)]
mod tests;

/// A composite that is no specialization.
#[derive(Clone)]
pub struct AnyComposite {
    objects: Arc<[Object]>,
}

impl CompositeApi for AnyComposite {
    fn size(&self) -> u64 {
        self.as_slice().len() as u64
    }

    fn has(&self, object: &Object) -> bool {
        self.objects.binary_search(object).is_ok()
    }

    fn has_item(&self, item: &Object) -> bool {
        self.objects
            .binary_search_by(|object| compare_to_contains(object, item))
            .is_ok()
    }

    fn item_count(&self) -> u64 {
        u64::try_from(
            self.objects
                .iter()
                .filter(|object| is_contains(object))
                .count(),
        )
        .unwrap_or(u64::MAX)
    }
}

/// Compares `object` with the virtual construction `contains ·· item`.
fn compare_to_contains(object: &Object, item: &Object) -> std::cmp::Ordering {
    match object {
        Object::Construction(construction) => construction
            .constructor
            .as_ref()
            .cmp(&Object::Atom(Atom::CONTAINS))
            .then_with(|| construction.value.as_ref().cmp(item)),
        Object::Atom(_) => std::cmp::Ordering::Less,
        Object::Composite(_) => std::cmp::Ordering::Greater,
    }
}

fn is_contains(object: &Object) -> bool {
    matches!(
        object,
        Object::Construction(Construction { constructor, .. })
            if matches!(&**constructor, Object::Atom(Atom::CONTAINS))
    )
}

impl AnyComposite {
    /// Creates an [`AnyComposite`] from objects that are already sorted and
    /// deduplicated.
    ///
    /// DO NOT EXPOSE THIS!
    #[must_use]
    pub(super) fn from_sorted(objects: impl IntoIterator<Item = Object>) -> Self {
        let objects = objects.into_iter().collect::<Arc<[Object]>>();

        debug_assert!(objects.windows(2).all(|pair| pair[0] < pair[1]));

        Self { objects }
    }

    #[must_use]
    pub fn as_slice(&self) -> &[Object] {
        &self.objects
    }
}

impl Debug for AnyComposite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_composite(f, self.objects.iter())
    }
}

impl PartialEq for AnyComposite {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.objects, &other.objects) || self.objects == other.objects
    }
}

impl Eq for AnyComposite {}

impl PartialOrd for AnyComposite {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AnyComposite {
    fn cmp(&self, other: &Self) -> Ordering {
        if Arc::ptr_eq(&self.objects, &other.objects) {
            Ordering::Equal
        } else {
            self.objects.cmp(&other.objects)
        }
    }
}

impl IntoIterator for AnyComposite {
    type Item = Object;
    type IntoIter = AnyCompositeProperties;

    fn into_iter(self) -> Self::IntoIter {
        AnyCompositeProperties {
            composite: self,
            next_index: 0,
        }
    }
}

#[derive(Clone)]
pub struct AnyCompositeProperties {
    composite: AnyComposite,
    next_index: usize,
}

impl Iterator for AnyCompositeProperties {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.composite
            .as_slice()
            .get(self.next_index)
            .cloned()
            .inspect(|_| self.next_index += 1)
    }
}
