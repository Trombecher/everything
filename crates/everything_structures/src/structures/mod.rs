#[cfg(test)]
mod tests;

use std::{
    assert_matches,
    cmp::Ordering,
    fmt,
    hash::{DefaultHasher, Hash, Hasher},
    mem::MaybeUninit,
    slice,
    sync::{Arc, LazyLock},
};

use dashmap::DashMap;

use crate::{Object, properties::Property};

#[derive(Clone, Eq, Hash)]
pub struct Structure {
    propeties: Option<Arc<[Property]>>,
}

impl Structure {
    /// The empty structure. Contains no properties.
    pub const EMPTY: Self = Self { propeties: None };

    /// Checks if the structure has this property.
    #[must_use]
    pub fn has(&self, property: &Property) -> bool {
        match &self.propeties {
            None => false,
            Some(properties) => properties.binary_search(property).is_ok(),
        }
    }

    #[must_use]
    pub fn has_by_ref(&self, tag: &Object, value: &Object) -> bool {
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

    /// Creates a new structure from the given properties
    /// by adding them to the empty structure.
    pub fn new(properties: &mut [Property]) -> Self {
        Self::EMPTY.change(&mut [], properties)
    }

    /// Modifies this structure by adding and removing properties.
    /// Returns the modified structure.
    ///
    /// The properties need to be mutable because this method needs to
    /// reorder and dedup changes in-place to avoid unneccessary
    /// allocations.
    ///
    /// Note that first all indicated properties are removed from
    /// the structure and then all indicated properties are added.
    #[must_use]
    pub fn change(
        &self,
        remove_properties: &mut [Property],
        add_properties: &mut [Property],
    ) -> Structure {
        GLOBAL_REGISTRY.resolve(self, remove_properties, add_properties)
    }

    /// Returns an iterator over all values that this tag has
    /// in this structure.
    #[must_use]
    pub fn values<'props, 'tag>(&'props self, tag: &'tag Object) -> ValuesIter<'props, 'tag> {
        let properties = self.as_ref();
        let start = properties.partition_point(|property| &property.tag < tag);

        ValuesIter {
            props: properties[start..].iter(),
            tag,
            done: false,
        }
    }

    /// Returns an iterator over all tags that this value has
    /// in this structure.
    #[must_use]
    pub fn tags(&self, value: &Object) -> impl Iterator<Item = &Object> {
        self.as_ref()
            .iter()
            .filter_map(move |property| (&property.value == value).then_some(&property.tag))
    }
}

#[derive(Clone)]
pub struct ValuesIter<'props, 'tag> {
    props: slice::Iter<'props, Property>,
    tag: &'tag Object,
    done: bool,
}

impl<'props, 'tag> Iterator for ValuesIter<'props, 'tag> {
    type Item = &'props Object;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else if let Some(property) = self.props.next() {
            if &property.tag == self.tag {
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

impl fmt::Debug for Structure {
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
            // that have a ref. When removing this structure
            // from the registry, `self` will be the
            // only reference and thus will deallocate
            // after drop.

            GLOBAL_REGISTRY.remove(self);
        }
    }
}

static GLOBAL_REGISTRY: LazyLock<Registry> = LazyLock::new(Registry::empty);

struct Registry {
    /// A map of hashes to structures.
    entries: DashMap<u64, Arc<[Property]>>,
}

impl Registry {
    #[must_use]
    pub(crate) fn empty() -> Self {
        Self {
            entries: DashMap::new(),
        }
    }

    pub(crate) fn remove(&self, s: &Structure) {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let hash = hasher.finish();

        self.entries.remove(&hash);
    }

    pub(crate) fn resolve(
        &self,
        base: &Structure,
        remove_properties: &mut [Property],
        mut add_properties: &mut [Property],
    ) -> Structure {
        // Prepare properties
        remove_properties.sort();

        add_properties = add_properties.partition_dedup().0;
        add_properties.sort();

        let (hash, prop_count) = hash_of_new_structure(base, remove_properties, add_properties);

        if hash_of_nothing() == hash {
            // Empty structure.
            return Structure { propeties: None };
        }

        if let Some(x) = self.entries.get(&hash) {
            return Structure {
                propeties: Some(Arc::clone(&x)),
            };
        }

        // The structure does not exist yet,
        // so we have to create it.

        let new_props = allocate_new_structure(base, remove_properties, add_properties, prop_count);
        self.entries.insert(hash, Arc::clone(&new_props));

        Structure {
            propeties: Some(new_props),
        }
    }
}

/// Hashes the given structure will changes applied
/// and returns `(hash, prop_count)`
fn hash_of_new_structure(
    base: &Structure,
    remove_properties: &[Property],
    add_properties: &[Property],
) -> (u64, usize) {
    let mut hasher = DefaultHasher::new();
    let mut prop_count = 0_usize;

    let mut base_props = base.as_ref().iter().peekable();
    let mut add_iter = add_properties.iter().peekable();

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
                prop_count += 1;

                // Consume change.
                add_iter.next();
            }
            (Some(base_prop), None) => {
                let ignore_property = remove_properties.binary_search(base_prop).is_ok();

                if !ignore_property {
                    base_prop.hash(&mut hasher);
                    prop_count += 1;
                }

                base_props.next();
            }
            (Some(base_prop), Some(add_property)) => match base_prop.cmp(add_property) {
                Ordering::Less => {
                    // The base prop comes first.

                    let ignore_property = remove_properties
                        .binary_search_by(|probe| probe.cmp(base_prop))
                        .is_ok();

                    if !ignore_property {
                        base_prop.hash(&mut hasher);
                        prop_count += 1;
                    }

                    base_props.next();
                }
                Ordering::Equal => {
                    base_prop.hash(&mut hasher);
                    prop_count += 1;

                    add_iter.next();
                    base_props.next();
                }
                Ordering::Greater => {
                    // We should choose the property to add.
                    add_property.hash(&mut hasher);
                    prop_count += 1;

                    add_iter.next();
                }
            },
        }
    }

    (hasher.finish(), prop_count)
}

#[inline(always)]
fn hash_of_nothing() -> u64 {
    let hasher = DefaultHasher::new();
    hasher.finish()
}

fn allocate_new_structure(
    base: &Structure,
    remove_properties: &[Property],
    add_properties: &[Property],
    prop_count: usize,
) -> Arc<[Property]> {
    let mut new_props: Arc<[MaybeUninit<Property>]> = Arc::new_uninit_slice(prop_count);
    let mut new_props_iter = Arc::get_mut(&mut new_props).unwrap().iter_mut();

    let mut base_props = base.as_ref().iter().peekable();
    let mut add_iter = add_properties.iter().peekable();

    // Fill new props
    loop {
        let base_prop = base_props.peek().copied();
        let change = add_iter.peek().copied();

        match (base_prop, change) {
            (None, None) => break,
            (None, Some(add_property)) => {
                // There are no base props so
                // we just add this change.

                new_props_iter.next().unwrap().write(add_property.clone());

                // Consume change.
                add_iter.next();
            }
            (Some(base_prop), None) => {
                let ignore_property = remove_properties.binary_search(base_prop).is_ok();

                if !ignore_property {
                    new_props_iter.next().unwrap().write(base_prop.clone());
                }

                base_props.next();
            }
            (Some(base_prop), Some(add_prop)) => match base_prop.cmp(add_prop) {
                Ordering::Less => {
                    // The base prop comes first.

                    let ignore_property = remove_properties
                        .binary_search_by(|probe| probe.cmp(base_prop))
                        .is_ok();

                    if !ignore_property {
                        new_props_iter.next().unwrap().write(base_prop.clone());
                    }

                    base_props.next();
                }
                Ordering::Equal => {
                    new_props_iter.next().unwrap().write(base_prop.clone());

                    add_iter.next();
                    base_props.next();
                }
                Ordering::Greater => {
                    // We should choose the property to add.
                    new_props_iter.next().unwrap().write(add_prop.clone());

                    add_iter.next();
                }
            },
        }
    }

    assert_matches!(new_props_iter.next(), None);

    unsafe { Arc::<[_]>::assume_init(new_props) }
}
