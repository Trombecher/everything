use num_bigint::BigInt;

use crate::model::{Atom, Composite, CompositeApi, CompositeSpecialization, Construction, Object};

use super::*;

fn construction(constructor: Object, value: Object) -> Object {
    Construction::new(constructor, value).into()
}

fn code_point(value: impl Into<BigInt>) -> Object {
    construction(Atom::CODE_POINT.into(), Object::new_integer(value.into()))
}

fn character(representing: char) -> CharacterComposite {
    CharacterComposite { representing }
}

#[test]
fn parses_code_point_representations() {
    assert_eq!(CharacterComposite::parse_char(&code_point(65)), Some('A'));
    assert_eq!(CharacterComposite::parse_char(&code_point(0)), Some('\0'));
    assert_eq!(
        CharacterComposite::parse_char(&code_point(0x10_FFFF)),
        Some(char::MAX)
    );
}

#[test]
fn rejects_invalid_code_point_representations() {
    // Surrogates are not valid `char`s.
    assert_eq!(CharacterComposite::parse_char(&code_point(0xD800)), None);
    assert_eq!(CharacterComposite::parse_char(&code_point(0x11_0000)), None);
    assert_eq!(CharacterComposite::parse_char(&code_point(-1)), None);
    assert_eq!(
        CharacterComposite::parse_char(&construction(
            Atom::CODE_POINT.into(),
            Object::Atom(Atom(65)),
        )),
        None
    );
    assert_eq!(
        CharacterComposite::parse_char(&construction(
            Atom::SUCCESSOR_OF.into(),
            Object::new_integer(BigInt::from(65)),
        )),
        None
    );
    assert_eq!(
        CharacterComposite::parse_char(&Object::Atom(Atom::ZERO)),
        None
    );
}

#[test]
fn specializes_single_code_point_representations() {
    assert_eq!(
        CharacterComposite::specialize_from(&[code_point(65)]),
        Some(character('A'))
    );
    assert_eq!(
        CharacterComposite::specialize_from(&[code_point(0)]),
        Some(character('\0'))
    );
}

#[test]
fn does_not_specialize_other_object_lists() {
    assert_eq!(CharacterComposite::specialize_from(&[]), None);
    assert_eq!(
        CharacterComposite::specialize_from(&[code_point(65), code_point(66)]),
        None
    );
    assert_eq!(
        CharacterComposite::specialize_from(&[code_point(0xD800)]),
        None
    );
    assert_eq!(
        CharacterComposite::specialize_from(&[Object::Atom(Atom::ZERO)]),
        None
    );
}

#[test]
fn size_is_always_one() {
    assert_eq!(character('A').size(), 1);
    assert_eq!(character('\0').size(), 1);
    assert_eq!(character(char::MAX).size(), 1);
}

#[test]
fn has_matches_the_representing_character() {
    let composite = character('A');

    assert!(composite.has(&code_point(65)));
    assert!(!composite.has(&code_point(66)));
    assert!(!composite.has(&code_point(0xD800)));
    assert!(!composite.has(&Object::Atom(Atom::ZERO)));
    assert!(!composite.has(&construction(
        Atom::SUCCESSOR_OF.into(),
        Object::new_integer(BigInt::from(65)),
    )));
}

#[test]
fn iteration_reconstructs_the_code_point() {
    for representing in ['\0', 'A', 'ß', '😀', char::MAX] {
        let mut properties = character(representing).into_iter();

        assert_eq!(properties.next(), Some(code_point(u32::from(representing))));
        assert_eq!(properties.next(), None);
        assert_eq!(properties.next(), None);
    }
}

#[test]
fn debug_fmt_shows_the_represented_character() {
    assert_eq!(format!("{:?}", character('A')), "'A'");
    assert_eq!(format!("{:?}", character('ß')), "'ß'");
    assert_eq!(format!("{:?}", character('😀')), "'😀'");
}

#[test]
fn composite_specialization_round_trips() {
    for representing in ['\0', 'A', 'ß', '😀', char::MAX] {
        let representation = code_point(u32::from(representing));
        let composite = Composite::from([representation.clone()]);

        assert_eq!(composite, Composite::Character(character(representing)));
        assert_eq!(composite.properties().collect::<Vec<_>>(), [representation]);
    }
}
