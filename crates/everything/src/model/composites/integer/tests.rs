use num_bigint::BigInt;

use crate::model::{
    Atom, Composite, CompositeApi, CompositeSpecialization, Construction, NonZeroBigInt, Object,
};

use super::*;

fn construction(constructor: Object, value: Object) -> Object {
    Construction::new(constructor, value).into()
}

fn successor(value: impl Into<BigInt>) -> Object {
    construction(Atom::SUCCESSOR_OF.into(), Object::new_integer(value.into()))
}

fn predecessor(value: impl Into<BigInt>) -> Object {
    construction(
        Atom::PREDECESSOR_OF.into(),
        Object::new_integer(value.into()),
    )
}

fn non_zero(value: impl Into<BigInt>) -> NonZeroBigInt {
    NonZeroBigInt::new(value.into()).expect("the value must be non-zero")
}

fn representing(value: impl Into<BigInt>) -> IntegerComposite {
    IntegerComposite {
        representing: non_zero(value),
    }
}

#[test]
fn parses_successor_representations() {
    assert_eq!(
        IntegerComposite::parse_representation_successor(&successor(0)),
        Some(non_zero(1))
    );
    assert_eq!(
        IntegerComposite::parse_representation_successor(&successor(41)),
        Some(non_zero(42))
    );
    assert_eq!(
        IntegerComposite::parse_representation_successor(&successor(BigInt::from(2).pow(200))),
        Some(non_zero(BigInt::from(2).pow(200) + 1))
    );
}

#[test]
fn rejects_invalid_successor_representations() {
    assert_eq!(
        IntegerComposite::parse_representation_successor(&successor(-1)),
        None
    );
    assert_eq!(
        IntegerComposite::parse_representation_successor(&construction(
            Atom::SUCCESSOR_OF.into(),
            Object::Atom(Atom(1)),
        )),
        None
    );
    assert_eq!(
        IntegerComposite::parse_representation_successor(&predecessor(0)),
        None
    );
    assert_eq!(
        IntegerComposite::parse_representation_successor(&Object::Atom(Atom::ZERO)),
        None
    );
}

#[test]
fn parses_predecessor_representations() {
    assert_eq!(
        IntegerComposite::parse_representation_predecessor(&predecessor(0)),
        Some(non_zero(-1))
    );
    assert_eq!(
        IntegerComposite::parse_representation_predecessor(&predecessor(-41)),
        Some(non_zero(-42))
    );
    assert_eq!(
        IntegerComposite::parse_representation_predecessor(&predecessor(-BigInt::from(2).pow(200))),
        Some(non_zero(-BigInt::from(2).pow(200) - 1))
    );
}

#[test]
fn rejects_invalid_predecessor_representations() {
    assert_eq!(
        IntegerComposite::parse_representation_predecessor(&predecessor(1)),
        None
    );
    assert_eq!(
        IntegerComposite::parse_representation_predecessor(&construction(
            Atom::PREDECESSOR_OF.into(),
            Object::Atom(Atom(1)),
        )),
        None
    );
    assert_eq!(
        IntegerComposite::parse_representation_predecessor(&successor(0)),
        None
    );
    assert_eq!(
        IntegerComposite::parse_representation_predecessor(&Object::Atom(Atom::ZERO)),
        None
    );
}

#[test]
fn specializes_single_successor_and_predecessor_representations() {
    assert_eq!(
        IntegerComposite::specialize_from(&[successor(0)]),
        Some(representing(1))
    );
    assert_eq!(
        IntegerComposite::specialize_from(&[predecessor(0)]),
        Some(representing(-1))
    );
}

#[test]
fn does_not_specialize_other_object_lists() {
    assert_eq!(IntegerComposite::specialize_from(&[]), None);
    assert_eq!(
        IntegerComposite::specialize_from(&[Object::Atom(Atom::ZERO)]),
        None
    );
    assert_eq!(
        IntegerComposite::specialize_from(&[successor(0), predecessor(0)]),
        None
    );
    assert_eq!(IntegerComposite::specialize_from(&[successor(-1)]), None);
    assert_eq!(
        IntegerComposite::specialize_from(&[construction(
            Atom::SUCCESSOR_OF.into(),
            Object::Atom(Atom(1)),
        )]),
        None
    );
}

#[test]
fn size_is_always_one() {
    assert_eq!(representing(1).size(), 1);
    assert_eq!(representing(-1).size(), 1);
    assert_eq!(representing(42).size(), 1);
}

#[test]
fn has_matches_the_representing_construction() {
    let five = representing(5);

    assert!(five.has(&successor(4)));
    assert!(!five.has(&successor(5)));
    assert!(!five.has(&predecessor(-6)));
    assert!(!five.has(&Object::Atom(Atom::ZERO)));

    let minus_five = representing(-5);

    assert!(minus_five.has(&predecessor(-4)));
    assert!(!minus_five.has(&predecessor(-5)));
    assert!(!minus_five.has(&successor(4)));
}

#[test]
fn iteration_reconstructs_the_construction() {
    assert_eq!(
        representing(1).properties().collect::<Vec<_>>(),
        [successor(0)]
    );
    assert_eq!(
        representing(42).properties().collect::<Vec<_>>(),
        [successor(41)]
    );
    assert_eq!(
        representing(-1).properties().collect::<Vec<_>>(),
        [predecessor(0)]
    );
    assert_eq!(
        representing(-42).properties().collect::<Vec<_>>(),
        [predecessor(-41)]
    );
}

#[test]
fn iteration_is_fused() {
    let mut properties = representing(3).into_iter();

    assert_eq!(properties.next(), Some(successor(2)));
    assert_eq!(properties.next(), None);
    assert_eq!(properties.next(), None);
}

#[test]
fn iteration_supports_large_integers() {
    let value = BigInt::from(2).pow(200);

    assert_eq!(
        representing(value.clone() + 1)
            .properties()
            .collect::<Vec<_>>(),
        [successor(value)]
    );
}

#[test]
fn debug_fmt_shows_the_represented_integer() {
    assert_eq!(format!("{:?}", representing(42)), "42");
    assert_eq!(format!("{:?}", representing(-42)), "-42");
}

#[test]
fn composite_specialization_round_trips() {
    for value in [0, 1, 41, -1, -41] {
        let representation = if value >= 0 {
            successor(value)
        } else {
            predecessor(value)
        };
        let expected = if value >= 0 {
            representing(value + 1)
        } else {
            representing(value - 1)
        };

        let composite = Composite::from([representation.clone()]);

        assert_eq!(composite, Composite::Integer(expected));
        assert_eq!(composite.properties().collect::<Vec<_>>(), [representation]);
    }
}
