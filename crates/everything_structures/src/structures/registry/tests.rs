use std::{
    hash::{DefaultHasher, Hash, Hasher},
    num::NonZeroU128,
};

use crate::{Abstract, Bit, BitSlot, Object, Property, Structure};

#[test]
fn structure_meta_info_of_empty() {
    let base = Structure::Empty;
    let mut info = super::structure_meta_info(&base, &[], &[]);

    let empty_hash = {
        let mut hasher = DefaultHasher::new();
        hasher.write_usize(0);
        hasher.finish()
    };

    // Nothing should be hashed.
    assert_eq!(info.final_hash(), empty_hash);
    assert_eq!(info.last_bit_slots, [None; 8]);
    assert_eq!(info.last_code_point, None);
    assert_eq!(info.last_list_item, None);
    assert_eq!(info.last_list_tail, None);
    assert_eq!(info.last_successor_of, None);
    assert_eq!(info.prop_count, 0);
}

/// This removes `SUCCESSOR_OF` from a natural number and checks wheather
/// [Structure::Empty] is the result.
#[test]
fn structure_meta_info_of_empty_2() {
    let base = Structure::NaturalNumber(NonZeroU128::new(42).unwrap());
    let mut info = super::structure_meta_info(
        &base,
        &[Property {
            tag: Object::Abstract(Abstract::SUCCESSOR_OF),
            value: Object::Structure(Structure::NaturalNumber(NonZeroU128::new(41).unwrap())),
        }],
        &[],
    );

    let empty_hash = {
        let mut hasher = DefaultHasher::new();
        hasher.write_usize(0);
        hasher.finish()
    };

    // Nothing should be hashed.
    assert_eq!(info.final_hash(), empty_hash);
    assert_eq!(info.last_bit_slots, [None; 8]);
    assert_eq!(info.last_code_point, None);
    assert_eq!(info.last_list_item, None);
    assert_eq!(info.last_list_tail, None);
    assert_eq!(info.last_successor_of, None);
    assert_eq!(info.prop_count, 0);
}

#[test]
fn structure_meta_info_of_natural_number() {
    const PROP: Property = Property {
        tag: Object::Abstract(Abstract::SUCCESSOR_OF),
        value: Object::Structure(Structure::NaturalNumber(NonZeroU128::new(41).unwrap())),
    };

    let base = Structure::Empty;
    let mut info = super::structure_meta_info(&base, &[], &[PROP]);

    let hash_of_structure = {
        let mut hasher = DefaultHasher::new();
        PROP.hash(&mut hasher);
        hasher.write_usize(1);
        hasher.finish()
    };

    assert_eq!(info.final_hash(), hash_of_structure);
    assert_eq!(info.last_bit_slots, [None; 8]);
    assert_eq!(info.last_code_point, None);
    assert_eq!(info.last_list_item, None);
    assert_eq!(info.last_list_tail, None);
    assert_eq!(info.last_successor_of, Some(41));
    assert_eq!(info.prop_count, 1);
}

#[test]
fn structure_meta_info_of_byte() {
    for byte in 0..=255_u8 {
        let properties = [
            Property::new_bit_slot(BitSlot::Slot0, Bit::from(byte & 1 != 0)),
            Property::new_bit_slot(BitSlot::Slot1, Bit::from(byte & 2 != 0)),
            Property::new_bit_slot(BitSlot::Slot2, Bit::from(byte & 4 != 0)),
            Property::new_bit_slot(BitSlot::Slot3, Bit::from(byte & 8 != 0)),
            Property::new_bit_slot(BitSlot::Slot4, Bit::from(byte & 16 != 0)),
            Property::new_bit_slot(BitSlot::Slot5, Bit::from(byte & 32 != 0)),
            Property::new_bit_slot(BitSlot::Slot6, Bit::from(byte & 64 != 0)),
            Property::new_bit_slot(BitSlot::Slot7, Bit::from(byte & 128 != 0)),
        ];

        let info = super::structure_meta_info(&Structure::Empty, &[], &properties);

        assert_eq!(
            info.last_bit_slots,
            [
                Bit::from(byte & 1 != 0),
                Bit::from(byte & 2 != 0),
                Bit::from(byte & 4 != 0),
                Bit::from(byte & 8 != 0),
                Bit::from(byte & 16 != 0),
                Bit::from(byte & 32 != 0),
                Bit::from(byte & 64 != 0),
                Bit::from(byte & 128 != 0),
            ]
            .map(Some)
        );
        assert_eq!(info.last_code_point, None);
        assert_eq!(info.last_list_item, None);
        assert_eq!(info.last_list_tail, None);
        assert_eq!(info.last_successor_of, None);
        assert_eq!(info.prop_count, 8);
    }
}
