use std::num::NonZeroI128;

use crate::model::{Atom, Construction};

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
fn empty_set_is_canonical() {
    assert_eq!(Set::empty(), Set::new(&[]));
    assert_eq!(Set::empty(), Set::new_move([]));
    assert_eq!(Set::empty().size(), 0);
}

#[test]
fn duplicate_values_are_removed() {
    let atom = Object::Atom(Atom(0xDEAD_BEEF));

    let from_slice = Set::new(&[atom.clone(), atom.clone(), atom.clone()]);
    let from_array = Set::new_move([atom.clone(), atom]);

    assert_eq!(from_slice, from_array);
    assert_eq!(from_slice.size(), 1);
}

#[test]
fn duplicate_values_do_not_select_invalid_specialization() {
    let bit = construction(Atom::BIT_SLOT_0.into(), Atom::BIT_ON.into());
    let objects: [Object; 8] = std::array::from_fn(|_| bit.clone());

    let set = Set::new(&objects);

    assert!(matches!(&set, Set::OneConstruction(_)));
    assert_eq!(set.size(), 1);
}

#[test]
fn slice_and_array_construction_are_canonical() {
    let a = Object::Atom(Atom(10));
    let b = Object::Atom(Atom(20));
    let c = Object::Atom(Atom(30));

    let from_slice = Set::new(&[c.clone(), a.clone(), b.clone(), a.clone()]);
    let from_array = Set::new_move([c, b, a]);

    assert_eq!(from_slice, from_array);
}

#[test]
fn specializes_integer_sets() {
    let set = Set::new(&[successor_of_zero()]);

    assert_eq!(set.size(), 1);
    assert_eq!(
        set,
        Set::Integer(IntegerSet {
            representing: NonZeroI128::new(1).unwrap(),
        })
    );
    assert_eq!(Object::Set(set), Object::new_integer(1));
}

#[test]
fn specializes_character_sets() {
    let set = Set::new_move([construction(
        Atom::CODE_POINT.into(),
        Object::new_integer(i128::from(u32::from('A'))),
    )]);

    assert_eq!(set.size(), 1);
    assert_eq!(set, Set::Character(CharacterSet { representing: 'A' }));
}

#[test]
fn specializes_byte_sets() {
    let objects = byte_objects();

    let from_slice = Set::new(&objects);
    let from_array = Set::new_move(objects);

    assert_eq!(from_slice, from_array);
    assert_eq!(from_slice, Set::Byte(ByteSet(0xA5)));
    assert_eq!(from_slice.size(), 8);
}

#[test]
fn specializes_one_construction_sets() {
    let object = construction(Atom(0x1234).into(), Atom(0x5678).into());

    let set = Set::new(std::slice::from_ref(&object));

    assert!(matches!(&set, Set::OneConstruction(_)));
    assert_eq!(set, Set::new_move([object]));
}

#[test]
fn arbitrary_sets_are_sorted_and_deduplicated() {
    let a = Object::Atom(Atom(10));
    let b = Object::Atom(Atom(20));
    let c = Object::Atom(Atom(30));

    let set = Set::new(&[c.clone(), a.clone(), b.clone(), a.clone()]);

    assert!(matches!(&set, Set::Any(_)));
    assert_eq!(set.size(), 3);

    let values: Vec<_> = set.values().collect();
    assert_eq!(values.as_slice(), [a, b, c].as_slice());
}
