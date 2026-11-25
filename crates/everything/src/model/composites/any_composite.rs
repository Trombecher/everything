use std::{cmp::Ordering, fmt::Debug, sync::Arc};

use crate::model::{CompositeApi, Object};

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
        f.debug_set().entries(self.objects.iter()).finish()
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
