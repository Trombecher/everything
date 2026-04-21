use std::{
    assert_matches,
    cmp::Ordering,
    hash::{DefaultHasher, Hash, Hasher},
    mem::MaybeUninit,
    num::NonZeroU128,
    sync::{Arc, LazyLock},
};

use dashmap::DashMap;

use crate::{AnyStructure, BlobStructure, Object, Property, Structure};

enum Specialization {
    Empty,
    NaturalNumber(NonZeroU128),
    // TODO: do these
}

#[derive(Default)]
struct StructureMetaInfo {
    hasher: DefaultHasher,
    prop_count: usize,
    current_natural_number: Option<NonZeroU128>,
    // TODO: more
}

impl StructureMetaInfo {
    pub fn add_property(&mut self, property: &Property) {
        if property.tag == Object::SUCCESSOR_OF
            && let Some(number_property_is_successor_of) = property.value.exact_natural_number()
        {
            // FIXME: this will only be useful when the prop count is at 0.

            self.current_natural_number =
                NonZeroU128::new(number_property_is_successor_of.checked_add(1).unwrap())
        }

        property.hash(&mut self.hasher);
        self.prop_count += 1;
    }

    #[must_use]
    pub fn final_hash(&self) -> u64 {
        self.hasher.finish()
    }

    #[must_use]
    pub fn specialization(&self) -> Option<Specialization> {
        match self {
            Self { prop_count: 0, .. } => Some(Specialization::Empty),
            Self {
                prop_count: 1,
                current_natural_number: Some(n),
                ..
            } => Some(Specialization::NaturalNumber(*n)),
            // TODO: more
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GlobalRegistry;

static GLOBAL_PROPERTIES: LazyLock<DashMap<u64, Arc<[Property]>>> = LazyLock::new(DashMap::new);

static GLOBAL_BLOBS: LazyLock<DashMap<u64, Arc<[u8]>>> = LazyLock::new(DashMap::new);

pub fn remove(s: &AnyStructure) {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();

    GLOBAL_PROPERTIES.remove(&hash);
}

pub fn resolve(
    base: &Structure,
    remove_properties: &mut [Property],
    mut add_properties: &mut [Property],
) -> Structure {
    // Prepare properties
    remove_properties.sort();

    add_properties = add_properties.partition_dedup().0;
    add_properties.sort();

    let info = structure_meta_info(base, remove_properties, add_properties);

    match info.specialization() {
        Some(Specialization::Empty) => return Structure::Empty,
        Some(Specialization::NaturalNumber(n)) => return Structure::NaturalNumber(n),
        None => {
            // We have no specialization, so we just
            // allocate that.
        }
    }

    let hash = info.final_hash();

    if let Some(x) = GLOBAL_PROPERTIES.get(&hash) {
        // Because this structure was already registered in
        // the global properties, it has to be an "any"
        // and not a specialization. Therefore it is safe to
        // return this:

        return Structure::Any(AnyStructure {
            properties: Arc::clone(&x),
        });
    }

    // The structure does not exist yet,
    // so we have to create it.

    let new_properties =
        allocate_new_structure(base, remove_properties, add_properties, info.prop_count);
    GLOBAL_PROPERTIES.insert(hash, Arc::clone(&new_properties));

    Structure::Any(AnyStructure {
        properties: new_properties,
    })
}

fn resolve_blob(slice: &[u8]) -> BlobStructure {
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

fn remove_blob(blob: BlobStructure) {
    if blob.as_ref().len() == 0 {
        return;
    }

    let mut hasher = DefaultHasher::new();
    blob.hash(&mut hasher);
    let hash_of_blob = hasher.finish();

    GLOBAL_BLOBS.remove(&hash_of_blob);
}

/// Calculates meta information about the structure with all
/// specified properties removed and then all specified properties added.
fn structure_meta_info(
    base: &Structure,
    remove_properties: &[Property],
    add_properties: &[Property],
) -> StructureMetaInfo {
    let mut info = StructureMetaInfo::default();

    let mut base_properties = base.properties().peekable();
    let mut add_iter = add_properties.iter().peekable();

    // Calculate hash
    loop {
        match (base_properties.peek(), add_iter.peek().cloned()) {
            (None, None) => break,
            (None, Some(property_to_add)) => {
                // There are no base props so
                // we just add this change.

                info.add_property(property_to_add);

                // Consume property.
                add_iter.next();
            }
            (Some(base_property), None) => {
                let ignore_property = remove_properties.binary_search(base_property).is_ok();

                if !ignore_property {
                    info.add_property(base_property);
                }

                base_properties.next();
            }
            (Some(base_property), Some(property_to_add)) => {
                match base_property.as_ref().cmp(property_to_add) {
                    Ordering::Less => {
                        // The base prop comes first. That's
                        // why we just add it.

                        let ignore_property =
                            remove_properties.binary_search(base_property).is_ok();

                        if !ignore_property {
                            info.add_property(base_property);
                        }

                        base_properties.next();
                    }
                    Ordering::Equal => {
                        // Both properties are equal

                        info.add_property(base_property);

                        add_iter.next();
                        base_properties.next();
                    }
                    Ordering::Greater => {
                        // We should choose the property to add.
                        info.add_property(property_to_add);

                        add_iter.next();
                    }
                }
            }
        }
    }

    info
}

fn allocate_new_structure(
    base: &Structure,
    remove_properties: &[Property],
    add_properties: &[Property],
    prop_count: usize,
) -> Arc<[Property]> {
    let mut new_props: Arc<[MaybeUninit<Property>]> = Arc::new_uninit_slice(prop_count);
    let mut new_props_iter = Arc::get_mut(&mut new_props).unwrap().iter_mut();

    let mut base_props = base.properties().peekable();
    let mut add_iter = add_properties.iter().peekable();

    // Fill new props
    loop {
        let base_prop = base_props.peek();
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
                    new_props_iter
                        .next()
                        .unwrap()
                        .write(base_prop.clone().into_owned());
                }

                base_props.next();
            }
            (Some(base_prop), Some(add_prop)) => match base_prop.as_ref().cmp(add_prop) {
                Ordering::Less => {
                    // The base prop comes first.

                    let ignore_property = remove_properties
                        .binary_search_by(|probe| probe.cmp(base_prop))
                        .is_ok();

                    if !ignore_property {
                        new_props_iter
                            .next()
                            .unwrap()
                            .write(base_prop.clone().into_owned());
                    }

                    base_props.next();
                }
                Ordering::Equal => {
                    new_props_iter
                        .next()
                        .unwrap()
                        .write(base_prop.clone().into_owned());

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
