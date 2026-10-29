use num_bigint::BigInt;

use crate::model::{Atom, CharacterSet, Construction, Object, Set};

use super::*;

fn construction(constructor: Object, value: Object) -> Object {
    Construction::new(constructor, value).into()
}

fn character_object(character: char) -> Object {
    Object::Set(Set::Character(CharacterSet {
        representing: character,
    }))
}

fn text_object(text: &str) -> Object {
    Object::Set(Set::Text(TextSet::new(text.into()).unwrap()))
}

/// Returns the two values of the text set representing `text`.
fn text_values(text: &str) -> [Object; 2] {
    let mut characters = text.chars();
    let item = characters.next().unwrap();
    let tail = characters.as_str();

    [
        construction(Atom::LIST_ITEM.into(), character_object(item)),
        construction(
            Atom::LIST_TAIL.into(),
            if tail.is_empty() {
                Set::empty().into()
            } else {
                text_object(tail)
            },
        ),
    ]
}

#[test]
fn specializes_text_sets() {
    let values = text_values("hello");
    let set = Set::from(values.clone());

    assert_eq!(set, Set::Text(TextSet::new("hello".into()).unwrap()));
    assert_eq!(set.size(), 2);
    assert!(set.has(&values[0]));
    assert!(set.has(&values[1]));

    // Re-specializing the values must give back the same set. `add_object`
    // and `remove_object` rely on this.
    let mut rebuilt_values: Vec<_> = set.clone().values().collect();
    assert_eq!(Set::from(rebuilt_values.as_mut_slice()), set);
}

#[test]
fn specializes_single_character_text_sets() {
    let values = text_values("x");
    let set = Set::from(values.clone());

    assert_eq!(set, Set::Text(TextSet::new("x".into()).unwrap()));
    assert_eq!(set.size(), 2);
    assert!(set.has(&values[0]));
    assert!(set.has(&values[1]));
}

#[test]
fn does_not_specialize_ill_formed_text_sets() {
    let item = construction(Atom::LIST_ITEM.into(), character_object('a'));

    // The tail is neither text nor the empty set.
    let non_text_tail = construction(
        Atom::LIST_TAIL.into(),
        Set::from([Atom::ZERO.into()]).into(),
    );
    assert!(matches!(
        Set::from([item.clone(), non_text_tail]),
        Set::Any(_)
    ));

    // The item is not a character.
    let non_character_item = construction(Atom::LIST_ITEM.into(), Atom::ZERO.into());
    let empty_tail = construction(Atom::LIST_TAIL.into(), Set::empty().into());
    assert!(matches!(
        Set::from([non_character_item, empty_tail.clone()]),
        Set::Any(_)
    ));

    // The item is a character, but not in its specialized form.
    let unspecialized_item = construction(
        Atom::LIST_ITEM.into(),
        construction(
            Atom::CODE_POINT.into(),
            Object::new_integer(BigInt::from(u32::from('a'))),
        ),
    );
    assert!(matches!(
        Set::from([unspecialized_item, empty_tail.clone()]),
        Set::Any(_)
    ));

    // Both slots are `list_item`; there is no `list_tail`.
    let second_item = construction(Atom::LIST_ITEM.into(), character_object('b'));
    assert!(matches!(Set::from([item, second_item]), Set::Any(_)));

    // A set with more than two values is never text.
    assert!(matches!(
        Set::from([empty_tail, Atom::ZERO.into(), Atom::LIST_ITEM.into()]),
        Set::Any(_)
    ));
}
