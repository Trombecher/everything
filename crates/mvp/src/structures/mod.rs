#[cfg(test)]
mod tests;

use std::{
    cmp::Ordering,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    sync::{Arc, LazyLock},
};

use dashmap::DashMap;

use crate::properties::Property;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Change {
    Add(Property),
    Remove(Property),
}

impl Change {
    pub fn property(&self) -> &Property {
        match self {
            Self::Add(property) => property,
            Self::Remove(property) => property,
        }
    }
}

#[derive(Clone, Eq, Hash)]
pub struct Structure {
    propeties: Option<Arc<[Property]>>,
}

impl Structure {
    pub const EMPTY: Self = Self { propeties: None };

    #[must_use]
    pub fn has(&self, property: &Property) -> bool {
        match &self.propeties {
            None => false,
            Some(properties) => properties.binary_search(property).is_ok(),
        }
    }

    #[must_use]
    pub fn change(&self, changes: &mut [Change]) -> Structure {
        GLOBAL_REGISTRY.resolve(self, changes)
    }
}

impl Debug for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_set();

        if let Some(props) = &self.propeties {
            for prop in props.iter() {
                debug.entry(prop);
            }
        }

        debug.finish()
    }
}

impl AsRef<[Property]> for Structure {
    fn as_ref(&self) -> &[Property] {
        match &self.propeties {
            None => &[],
            Some(x) => x,
        }
    }
}

impl PartialEq for Structure {
    fn eq(&self, other: &Self) -> bool {
        match (&self.propeties, &other.propeties) {
            (None, None) => true,
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl PartialOrd for Structure {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Structure {
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

impl Drop for Structure {
    fn drop(&mut self) {
        if let Some(props) = &self.propeties
            && Arc::strong_count(props) == 2
        {
            // We and the registry are the only ones
            // that have a ref.
        }
    }
}

static GLOBAL_REGISTRY: LazyLock<Registry> = LazyLock::new(Registry::empty);

struct Registry {
    entries: DashMap<u64, Arc<[Property]>>,
}

/*
fn merge_iters<I: Iterator>(a: I, b: I) -> I {
    let mut a = a.peekable();
    let mut b = b.peekable();

    iter::from_fn(move || {
        match (a.peek(), b.peek()) {
            (Some(a), Some(b)) => {
            }
            (None, None) => None,
            (None, Some(x)) => Some(x)
        }
    })
} */

impl Registry {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            entries: DashMap::new(),
        }
    }

    pub fn resolve(&self, base: &Structure, mut changes: &mut [Change]) -> Structure {
        // Remove duplicates
        changes = changes.partition_dedup().0;

        // Sort
        changes.sort();

        // Partition into add and remove.
        let first_remove_index = changes.partition_point(|a| matches!(a, Change::Add(_)));
        let props_to_add = &changes[..first_remove_index];
        let props_to_remove = &changes[first_remove_index..];

        let mut base_props = base.as_ref().iter().peekable();
        let mut add_iter = props_to_add.iter().peekable();

        let mut hasher = DefaultHasher::new();
        let empty_hash = hasher.finish();

        // Calculate hash
        loop {
            let base_prop = base_props.peek().copied();
            let change = add_iter.peek().copied();

            match (base_prop, change) {
                (None, None) => break,
                (None, Some(change)) => {
                    // There are no base props so
                    // we just add this change.

                    change.hash(&mut hasher);

                    // Consume change.
                    add_iter.next();
                }
                (Some(property), None) => {
                    let ignore_property = props_to_remove
                        .binary_search(&Change::Remove(property.clone()))
                        .is_ok();

                    if !ignore_property {
                        property.hash(&mut hasher);
                    }

                    base_props.next();
                }
                (Some(prop), Some(change)) => match prop.cmp(change.property()) {
                    Ordering::Less => {
                        // The base prop comes first.

                        let ignore_property = props_to_remove
                            .binary_search_by(|probe| probe.property().cmp(prop))
                            .is_ok();

                        if !ignore_property {
                            prop.hash(&mut hasher);
                        }

                        base_props.next();
                    }
                    Ordering::Equal => {
                        prop.hash(&mut hasher);

                        add_iter.next();
                        base_props.next();
                    }
                    Ordering::Greater => {
                        // We should choose the property to add.
                        change.property().hash(&mut hasher);

                        add_iter.next();
                    }
                },
            }
        }

        let hash = hasher.finish();

        if empty_hash == hash {
            // Empty structure.
            return Structure { propeties: None };
        }

        if let Some(x) = self.entries.get(&hash) {
            return Structure {
                propeties: Some(Arc::clone(&x)),
            };
        }

        todo!("create structure")
    }
}
