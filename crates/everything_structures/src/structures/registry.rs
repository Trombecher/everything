//! The _registry_ is a global arena of structures. It serves
//! as a de-duplicator, such that no equal structure is allocated
//! twice.

use std::{
    assert_matches,
    cmp::Ordering,
    hash::{DefaultHasher, Hash, Hasher},
    mem::MaybeUninit,
    num::NonZeroU128,
    sync::{Arc, LazyLock},
};

use dashmap::DashMap;

use crate::{
    Abstract, AnyStructure, Bit, BitSlot, BytesStructure, Object, Property, Structure,
    TextStructure,
};

enum Specialization<'info> {
    Empty,
    NaturalNumber(NonZeroU128),
    Character(char),
    Binary {
        head: u8,
        tail: &'info BytesStructure,
    },
    Text {
        head: char,
        tail: &'info TextStructure,
    },
}

#[derive(Default)]
struct StructureMetaInfo {
    hasher: DefaultHasher,
    prop_count: usize,
    last_successor_of: Option<u128>,
    last_code_point: Option<char>,
    last_list_item: Option<Object>,
    last_list_tail: Option<Object>,
    last_bit_slots: [Option<Bit>; 8],
}

impl StructureMetaInfo {
    pub fn add_property(&mut self, property: &Property) {
        match property.tag {
            Object::Abstract(Abstract::SUCCESSOR_OF) => {
                if let Some(predecessor) = property.value.exact_natural_number()
                    && self.prop_count == 0
                {
                    self.last_successor_of = Some(predecessor)
                }
            }
            Object::Abstract(Abstract::CODE_POINT) => {
                if self.prop_count == 0
                    && let Some(maybe_char) = property.value.exact_natural_number()
                    && maybe_char <= u32::MAX as u128
                    && let Ok(c) = char::try_from(maybe_char as u32)
                {
                    self.last_code_point = Some(c)
                }
            }
            Object::Abstract(Abstract::LIST_ITEM) if self.prop_count < 2 => {
                self.last_list_item = Some(property.value.clone());
            }
            Object::Abstract(Abstract::LIST_TAIL) if self.prop_count < 2 => {
                self.last_list_tail = Some(property.value.clone());
            }
            Object::Abstract(maybe_slot)
                if let Ok(slot) = BitSlot::try_from(maybe_slot)
                    && self.prop_count < 8
                    && let Object::Abstract(maybe_bit) = property.value
                    && let Ok(bit) = Bit::try_from(maybe_bit) =>
            {
                // TODO: outsource this
                self.last_bit_slots[slot as u8 as usize] = Some(bit);
            }
            _ => {
                // Do nothing special.
            }
        }

        property.hash(&mut self.hasher);
        self.prop_count += 1;
    }

    #[must_use]
    pub fn final_hash(&self) -> u64 {
        self.hasher.finish()
    }

    /// Returns the specialization type.
    #[must_use]
    pub fn specialization<'info>(&'info self) -> Option<Specialization<'info>> {
        match self {
            Self { prop_count: 0, .. } => Some(Specialization::Empty),
            Self {
                last_successor_of: Some(n),
                ..
            } => Some(Specialization::NaturalNumber(
                NonZeroU128::new(n.checked_add(1).unwrap()).unwrap(),
            )),
            Self {
                last_code_point: Some(c),
                ..
            } => Some(Specialization::Character(*c)),
            Self {
                prop_count: 2,
                last_list_item: Some(item),
                last_list_tail: Some(tail),
                ..
            } if let Some(head) = item.exact_natural_number()
                && head <= 255
                && let Object::Structure(Structure::Bytes(binary)) = tail =>
            {
                Some(Specialization::Binary {
                    head: head as u8,
                    tail: binary,
                })
            }
            Self {
                prop_count: 2,
                last_list_item: Some(item),
                last_list_tail: Some(tail),
                ..
            } if let Object::Structure(Structure::Character(head)) = item
                && let Object::Structure(Structure::Text(tail)) = tail =>
            {
                Some(Specialization::Text { head: *head, tail })
            }
            // TODO: more
            _ => None,
        }
    }
}

static GLOBAL_PROPERTIES: LazyLock<DashMap<u64, Arc<[Property]>>> = LazyLock::new(DashMap::new);

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
        Some(Specialization::Character(c)) => return Structure::Character(c),
        Some(Specialization::Text { head, tail }) => {
            return Structure::Text(TextStructure::from_parts(head, tail.as_ref()));
        }
        Some(Specialization::Binary { head, tail }) => {
            // This unwrap is safe because [head].len() > 0.
            return Structure::Bytes(BytesStructure::from_parts(&[head], tail.as_ref()).unwrap());
        }
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
                match base_property.cmp(property_to_add) {
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
