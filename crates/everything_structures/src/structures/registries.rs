use std::{
    assert_matches,
    borrow::Cow,
    cmp::Ordering,
    hash::{DefaultHasher, Hash, Hasher},
    mem::MaybeUninit,
    sync::{Arc, LazyLock},
};

use dashmap::DashMap;

use crate::{AnyStructure, BlobStructure, Property, Structure};

#[derive(Clone, Debug)]
pub struct GlobalRegistry;

static GLOBAL_PROPERTIES: LazyLock<DashMap<u64, Arc<[Property]>>> = LazyLock::new(DashMap::new);

static GLOBAL_BLOBS: LazyLock<DashMap<u64, Arc<[u8]>>> = LazyLock::new(DashMap::new);

impl GlobalRegistry {
    pub(crate) fn remove(&self, s: &AnyStructure) {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let hash = hasher.finish();

        GLOBAL_PROPERTIES.remove(&hash);
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
            return Structure::EMPTY;
        }

        if let Some(x) = GLOBAL_PROPERTIES.get(&hash) {
            // Because this structure was already registered in
            // the global properties, it has to be an "any"
            // and not a specialization. Therefore it is safe to
            // return this:

            return Structure::Any(AnyStructure {
                properties: Some(Arc::clone(&x)),
            });
        }

        // The structure does not exist yet,
        // so we have to create it.

        let new_props = allocate_new_structure(base, remove_properties, add_properties, prop_count);
        GLOBAL_PROPERTIES.insert(hash, Arc::clone(&new_props));

        AnyStructure {
            properties: Some(new_props),
        }
    }

    fn resolve_blob(&self, slice: &[u8]) -> BlobStructure {
        if slice.len() == 0 {
            return BlobStructure { data: None };
        }

        let mut hasher = DefaultHasher::new();
        slice.hash(&mut hasher);
        let hash_of_slice = hasher.finish();

        if let Some(blob) = GLOBAL_BLOBS.get(&hash_of_slice) {
            BlobStructure {
                data: Some(Arc::clone(&blob)),
            }
        } else {
            // Entry does not yet exist.
            let data = Arc::clone_from_ref(slice);

            GLOBAL_BLOBS.insert(hash_of_slice, Arc::clone(&data));

            BlobStructure { data: Some(data) }
        }
    }

    fn remove_blob(&self, blob: BlobStructure) {
        if blob.as_ref().len() == 0 {
            return;
        }

        let mut hasher = DefaultHasher::new();
        blob.hash(&mut hasher);
        let hash_of_blob = hasher.finish();

        GLOBAL_BLOBS.remove(&hash_of_blob);
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

    let mut base_props = base.properties().peekable();
    let mut add_iter = add_properties.iter().peekable();

    // Calculate hash
    loop {
        match (base_props.peek(), add_iter.peek()) {
            (None, None) => break,
            (None, Some(property_to_add)) => {
                // There are no base props so
                // we just add this change.

                property_to_add.hash(&mut hasher);
                prop_count += 1;

                // Consume property.
                add_iter.next();
            }
            (Some(base_property), None) => {
                let ignore_property = remove_properties.binary_search(base_property).is_ok();

                if !ignore_property {
                    base_property.hash(&mut hasher);
                    prop_count += 1;
                }

                base_props.next();
            }
            (Some(base_property), Some(property_to_add)) => {
                match base_property.as_ref().cmp(property_to_add) {
                    Ordering::Less => {
                        // The base prop comes first. That's
                        // why we just add it.

                        let ignore_property =
                            remove_properties.binary_search(base_property).is_ok();

                        if !ignore_property {
                            base_property.hash(&mut hasher);
                            prop_count += 1;
                        }

                        base_props.next();
                    }
                    Ordering::Equal => {
                        // Both properties are equal

                        base_property.hash(&mut hasher);
                        prop_count += 1;

                        add_iter.next();
                        base_props.next();
                    }
                    Ordering::Greater => {
                        // We should choose the property to add.
                        property_to_add.hash(&mut hasher);
                        prop_count += 1;

                        add_iter.next();
                    }
                }
            }
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
