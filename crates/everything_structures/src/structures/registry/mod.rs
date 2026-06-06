//! The _registry_ is a global arena of structures. It serves
//! as a de-duplicator, such that no equal structure is allocated
//! twice.

#[cfg(test)]
mod tests;

use std::{
    cmp::Ordering,
    hash::{DefaultHasher, Hash, Hasher},
    mem::MaybeUninit,
    num::NonZeroI128,
    sync::{Arc, LazyLock},
};

use dashmap::DashMap;

use crate::{
    Abstract, AnyStructure, Bit, BitSlot, Byte, BytesStructure, MaybeEmptyBytesStructure,
    MaybeEmptyTextStructure, Object, Property, Structure, TextStructure,
};

enum Specialization {
    Empty,
    Integer(NonZeroI128),
    Byte(Byte),
    Character(char),
    Bytes {
        item: Byte,
        tail: MaybeEmptyBytesStructure,
    },
    Text {
        item: char,
        tail: MaybeEmptyTextStructure,
    },
}

#[derive(Default)]
struct StructureMetaInfo {
    hasher: DefaultHasher,
    prop_count: usize,
    last_successor_of: Option<i128>,
    last_predecessor_of: Option<i128>,
    last_code_point: Option<char>,
    last_list_item: Option<Object>,
    last_list_tail: Option<Object>,
    last_bit_slots: [Option<Bit>; 8],
}

impl StructureMetaInfo {
    pub fn add_property(&mut self, property: &Property) {
        match property.tag {
            Object::Abstract(Abstract::SUCCESSOR_OF) => {
                if let Some(predecessor) = property.value.exact_integer()
                    && self.prop_count == 0
                {
                    self.last_successor_of = Some(predecessor)
                }
            }
            Object::Abstract(Abstract::PREDECESSOR_OF) => {
                if let Some(successor) = property.value.exact_integer()
                    && self.prop_count == 0
                {
                    self.last_predecessor_of = Some(successor)
                }
            }
            Object::Abstract(Abstract::CODE_POINT) => {
                if self.prop_count == 0
                    && let Some(maybe_char) = property.value.exact_integer()
                    && (0..=u32::MAX as i128).contains(&maybe_char)
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
    pub fn final_hash(&mut self) -> u64 {
        // Write property count to prevent prefix collisions.
        self.hasher.write_usize(self.prop_count);
        self.hasher.finish()
    }

    /// Returns the specialization type.
    #[must_use]
    pub fn specialization(&self) -> Option<Specialization> {
        match self {
            Self { prop_count: 0, .. } => Some(Specialization::Empty),
            Self {
                last_successor_of: Some(predecessor),
                prop_count: 1,
                ..
            } => Some(Specialization::Integer(
                NonZeroI128::new(
                    predecessor
                        .checked_add(1)
                        .expect("yo integer too big for ts"),
                )
                .unwrap(),
            )),
            Self {
                last_predecessor_of: Some(successor),
                prop_count: 1,
                ..
            } => Some(Specialization::Integer(
                NonZeroI128::new(
                    successor
                        .checked_sub(1)
                        .expect("yo integer too small for ts"),
                )
                .unwrap(),
            )),
            Self {
                last_code_point: Some(c),
                prop_count: 1,
                ..
            } => Some(Specialization::Character(*c)),
            Self {
                prop_count: 2,
                last_list_item: Some(item),
                last_list_tail: Some(tail),
                ..
            } if let Object::Structure(Structure::Byte(item)) = item
                && let Object::Structure(tail) = tail
                && let Some(tail) = tail.exact_bytes() =>
            {
                Some(Specialization::Bytes { item: *item, tail })
            }
            Self {
                prop_count: 2,
                last_list_item: Some(item),
                last_list_tail: Some(tail),
                ..
            } if let Object::Structure(Structure::Character(item)) = item
                && let Object::Structure(tail) = tail
                && let Some(tail) = tail.exact_text() =>
            {
                Some(Specialization::Text { item: *item, tail })
            }
            Self {
                prop_count: 8,
                last_bit_slots:
                    slots @ [
                        Some(_),
                        Some(_),
                        Some(_),
                        Some(_),
                        Some(_),
                        Some(_),
                        Some(_),
                        Some(_),
                    ],
                ..
            } => Some(Specialization::Byte(Byte::from_bits(
                slots.map(Option::unwrap),
            ))),
            _ => None,
        }
    }
}

pub(super) static GLOBAL_PROPERTIES: LazyLock<DashMap<u64, Arc<[Property]>>> =
    LazyLock::new(DashMap::new);

pub fn remove(s: &AnyStructure) {
    GLOBAL_PROPERTIES.remove_if(&s.registry_hash, |_, arc| Arc::strong_count(arc) == 2);
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

    let mut info = structure_meta_info(base, remove_properties, add_properties);

    match info.specialization() {
        Some(Specialization::Empty) => return Structure::Empty,
        Some(Specialization::Integer(n)) => return Structure::Integer(n),
        Some(Specialization::Character(c)) => return Structure::Character(c),
        Some(Specialization::Text { item, tail }) => {
            return Structure::Text(TextStructure::from_parts(item, tail.as_ref()));
        }
        Some(Specialization::Bytes { item, tail }) => {
            // This unwrap is safe because [head].len() > 0.
            return Structure::Bytes(BytesStructure::from_parts(&[item.0], tail.as_ref()).unwrap());
        }
        Some(Specialization::Byte(byte)) => return Structure::Byte(byte),
        None => {
            // We have no specialization, so we just
            // allocate that.
        }
    }

    let hash = info.final_hash();

    match GLOBAL_PROPERTIES.entry(hash) {
        dashmap::Entry::Occupied(occupied_entry) => Structure::Any(AnyStructure {
            properties: Arc::clone(occupied_entry.get()),
            registry_hash: hash,
        }),
        dashmap::Entry::Vacant(vacant_entry) => {
            // The structure does not exist yet,
            // so we have to create it.

            let new_properties =
                allocate_new_structure(base, remove_properties, add_properties, info.prop_count);

            vacant_entry.insert(Arc::clone(&new_properties));

            Structure::Any(AnyStructure {
                properties: new_properties,
                registry_hash: hash,
            })
        }
    }
}

/// Calculates meta information about the structure with all
/// specified properties removed and then all specified properties added.
///
/// Both `remove_properties` and `add_properties` must be sorted;
/// `add_properties` must be deduped.
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
                // There are no (more) base props so
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
                        // Both properties are equal,
                        // so we just add one (no duplicates!).

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

/// Allocates a new atomic slice of properties.
///
/// `add_properties` and `remove_properties` must both be sorted;
/// additionally, `add_properties` must be deduped.
fn allocate_new_structure(
    base: &Structure,
    remove_properties: &[Property],
    add_properties: &[Property],
    prop_count: usize,
) -> Arc<[Property]> {
    let mut new_properties: Arc<[MaybeUninit<Property>]> = Arc::new_uninit_slice(prop_count);
    let mut new_properties_iter = Arc::get_mut(&mut new_properties).unwrap().iter_mut();

    let mut base_properties_iter = base.properties().peekable();
    let mut add_properties_iter = add_properties.iter().peekable();

    // Fill new props
    loop {
        match (
            base_properties_iter.peek(),
            add_properties_iter.peek().cloned(),
        ) {
            (None, None) => break,
            (None, Some(add_property)) => {
                // There are no base props so
                // we just add this change.

                new_properties_iter
                    .next()
                    .unwrap()
                    .write(add_property.clone());

                // Consume change.
                add_properties_iter.next();
            }
            (Some(base_property), None) => {
                let ignore_property = remove_properties.binary_search(base_property).is_ok();

                if !ignore_property {
                    new_properties_iter
                        .next()
                        .unwrap()
                        .write(base_property.clone());
                }

                base_properties_iter.next();
            }
            (Some(base_property), Some(add_property)) => match base_property.cmp(add_property) {
                Ordering::Less => {
                    // The base prop comes first.

                    let ignore_property = remove_properties
                        .binary_search_by(|probe| probe.cmp(base_property))
                        .is_ok();

                    if !ignore_property {
                        new_properties_iter
                            .next()
                            .unwrap()
                            .write(base_property.clone());
                    }

                    base_properties_iter.next();
                }
                Ordering::Equal => {
                    new_properties_iter
                        .next()
                        .unwrap()
                        .write(base_property.clone());

                    add_properties_iter.next();
                    base_properties_iter.next();
                }
                Ordering::Greater => {
                    // We should choose the property to add.
                    new_properties_iter
                        .next()
                        .unwrap()
                        .write(add_property.clone());

                    add_properties_iter.next();
                }
            },
        }
    }

    assert!(new_properties_iter.next().is_none());

    unsafe { Arc::<[_]>::assume_init(new_properties) }
}
