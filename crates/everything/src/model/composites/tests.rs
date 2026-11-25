use num_bigint::BigInt;

use crate::model::{Atom, Construction, NonZeroBigInt};

use super::*;

fn construction(constructor: Object, value: Object) -> Object {
    Construction::new(constructor, value).into()
}

fn successor_of_zero() -> Object {
    construction(Atom::SUCCESSOR_OF.into(), Atom::ZERO.into())
}

fn byte_objects() -> [Object; 8] {
    [
        construction(Atom::BIT_SLOT_0.into(), Atom::BIT_ON.into()),
        construction(Atom::BIT_SLOT_1.into(), Atom::BIT_OFF.into()),
        construction(Atom::BIT_SLOT_2.into(), Atom::BIT_ON.into()),
        construction(Atom::BIT_SLOT_3.into(), Atom::BIT_OFF.into()),
        construction(Atom::BIT_SLOT_4.into(), Atom::BIT_OFF.into()),
        construction(Atom::BIT_SLOT_5.into(), Atom::BIT_ON.into()),
        construction(Atom::BIT_SLOT_6.into(), Atom::BIT_OFF.into()),
        construction(Atom::BIT_SLOT_7.into(), Atom::BIT_ON.into()),
    ]
}

#[test]
fn empty_composite_is_canonical() {
    assert_eq!(Composite::empty(), Composite::from([]));
    assert_eq!(Composite::empty().size(), 0);
}

#[test]
fn duplicate_properties_are_removed() {
    let atom = Object::Atom(Atom(0xDEAD_BEEF));

    let from_slice = Composite::from([atom.clone(), atom.clone(), atom.clone()]);
    let from_array = Composite::from([atom.clone(), atom]);

    assert_eq!(from_slice, from_array);
    assert_eq!(from_slice.size(), 1);
}

#[test]
fn duplicate_properties_do_not_select_invalid_specialization() {
    let bit = construction(Atom::BIT_SLOT_0.into(), Atom::BIT_ON.into());
    let objects: [Object; 8] = std::array::from_fn(|_| bit.clone());

    let composite = Composite::from(objects);

    assert!(matches!(&composite, Composite::OneConstruction(_)));
    assert_eq!(composite.size(), 1);
}

#[test]
fn slice_and_array_construction_are_canonical() {
    let a = Object::Atom(Atom(10));
    let b = Object::Atom(Atom(20));
    let c = Object::Atom(Atom(30));

    let from_slice = Composite::from([c.clone(), a.clone(), b.clone(), a.clone()]);
    let from_array = Composite::from([c, b, a]);

    assert_eq!(from_slice, from_array);
}

#[test]
fn specializes_integer_composites() {
    let composite = Composite::from([successor_of_zero()]);

    assert_eq!(composite.size(), 1);
    assert_eq!(
        composite,
        Composite::Integer(IntegerComposite {
            representing: NonZeroBigInt::new(BigInt::from(1)).unwrap(),
        })
    );
    assert_eq!(
        Object::Composite(composite),
        Object::new_integer(BigInt::from(1))
    );
}

#[test]
fn specializes_character_composites() {
    let composite = Composite::from([construction(
        Atom::CODE_POINT.into(),
        Object::new_integer(BigInt::from(u32::from('A'))),
    )]);

    assert_eq!(composite.size(), 1);
    assert_eq!(
        composite,
        Composite::Character(CharacterComposite { representing: 'A' })
    );
}

#[test]
fn specializes_byte_composites() {
    let objects = byte_objects();

    let from_slice = Composite::from(objects.clone());
    let from_array = Composite::from(objects);

    assert_eq!(from_slice, from_array);
    assert_eq!(from_slice, Composite::Byte(ByteComposite(0xA5)));
    assert_eq!(from_slice.size(), 8);
}

#[test]
fn specializes_one_construction_composites() {
    let object = construction(Atom(0x1234).into(), Atom(0x5678).into());

    let composite = Composite::from([object.clone()]);

    assert!(matches!(&composite, Composite::OneConstruction(_)));
    assert_eq!(composite, Composite::from([object]));
}

#[test]
fn arbitrary_composites_are_sorted_and_deduplicated() {
    let a = Object::Atom(Atom(10));
    let b = Object::Atom(Atom(20));
    let c = Object::Atom(Atom(30));

    let composite = Composite::from([c.clone(), a.clone(), b.clone(), a.clone()]);

    assert!(matches!(&composite, Composite::Any(_)));
    assert_eq!(composite.size(), 3);

    let properties: Vec<_> = composite.properties().collect();
    assert_eq!(properties.as_slice(), [a, b, c].as_slice());
}

#[test]
fn every_variant_iterates_in_ascending_order() {
    let composites = [
        Composite::empty(),
        Composite::from([successor_of_zero()]),
        Composite::from([construction(
            Atom::CODE_POINT.into(),
            Object::new_integer(BigInt::from(u32::from('A'))),
        )]),
        Composite::from(byte_objects()),
        Composite::from([construction(Atom(0x1234).into(), Atom(0x5678).into())]),
        Composite::from([
            Object::Atom(Atom(30)),
            Object::Atom(Atom(10)),
            Object::Atom(Atom(20)),
        ]),
        Composite::from([
            construction(Atom::CONTAINS.into(), Object::Atom(Atom(10))),
            Object::Atom(Atom(20)),
        ]),
    ];

    for composite in composites {
        let properties: Vec<_> = composite.properties().collect();

        assert!(
            properties.windows(2).all(|pair| pair[0] < pair[1]),
            "properties are not ascending: {properties:?}"
        );
    }
}

#[test]
fn set_items_become_contains_constructions() {
    let a = Object::Atom(Atom(10));
    let b = Object::Atom(Atom(20));

    let expected = Composite::from([
        construction(Atom::CONTAINS.into(), a.clone()),
        construction(Atom::CONTAINS.into(), b.clone()),
    ]);

    assert_eq!(Composite::from_set_items(&[a.clone(), b.clone()]), expected);
    assert_eq!(Composite::from_set_items_array([a, b]), expected);
}

#[test]
fn set_items_are_canonical_regardless_of_order_and_duplicates() {
    let a = Object::Atom(Atom(10));
    let b = Object::Atom(Atom(20));

    let expected = Composite::from_set_items(&[a.clone(), b.clone()]);

    assert_eq!(
        Composite::from_set_items(&[b.clone(), a.clone(), b.clone(), a.clone()]),
        expected
    );
    assert_eq!(
        Composite::from_set_items_array([b, a.clone(), a.clone(), a]),
        expected
    );
}

#[test]
fn empty_set_items_are_the_empty_set() {
    let from_slice = Composite::from_set_items(&[]);
    let from_array = Composite::from_set_items_array([]);

    assert_eq!(from_slice, Composite::empty());
    assert_eq!(from_array, Composite::empty());
}

#[test]
fn set_items_specialize_without_creating_constructions_first() {
    let a = Object::Atom(Atom(10));
    let b = Object::Atom(Atom(20));

    assert!(matches!(Composite::from_set_items(&[]), Composite::Set(_)));
    assert!(matches!(
        Composite::from_set_items(std::slice::from_ref(&a)),
        Composite::OneConstruction(_)
    ));
    assert!(matches!(
        Composite::from_set_items(&[a.clone(), b.clone()]),
        Composite::Set(_)
    ));
    assert!(matches!(
        Composite::from_set_items_array([a.clone(), a.clone()]),
        Composite::OneConstruction(_)
    ));
    assert!(matches!(
        Composite::from_set_items_array([b.clone(), a, b]),
        Composite::Set(_)
    ));
}

#[test]
fn a_single_set_item_is_still_a_set() {
    let item = Object::Atom(Atom(10));
    let composite = Composite::from_set_items_array([item.clone()]);

    assert!(composite.has(&construction(Atom::CONTAINS.into(), item)));
    assert_eq!(composite.size(), 1);
}

#[test]
fn add_object_keeps_the_canonical_form() {
    let composite = Composite::empty().add_object(successor_of_zero());

    assert_eq!(composite, Composite::from([successor_of_zero()]));
    assert_eq!(composite.size(), 1);
}

#[test]
fn adding_a_present_object_does_not_change_the_composite() {
    let composite = Composite::from([successor_of_zero()]);

    assert_eq!(composite.clone().add_object(successor_of_zero()), composite);
}

#[test]
fn remove_object_keeps_the_canonical_form() {
    let object = construction(Atom(0x1234).into(), Atom(0x5678).into());
    let composite = Composite::from([object.clone(), successor_of_zero()]);

    assert_eq!(
        composite.remove_object(&object),
        Composite::from([successor_of_zero()])
    );
}

#[test]
fn removing_an_absent_object_does_not_change_the_composite() {
    let composite = Composite::from([successor_of_zero()]);

    assert_eq!(
        composite.clone().remove_object(&Atom(0x1234).into()),
        composite
    );
}
