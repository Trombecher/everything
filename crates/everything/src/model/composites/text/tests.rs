use num_bigint::BigInt;

use crate::model::{Atom, CharacterComposite, Composite, Construction, Object};

use super::*;

fn construction(constructor: Object, value: Object) -> Object {
    Construction::new(constructor, value).into()
}

fn character_object(character: char) -> Object {
    Object::Composite(Composite::Character(CharacterComposite {
        representing: character,
    }))
}

fn text_object(text: &str) -> Object {
    Object::Composite(Composite::Text(TextComposite::new(text.into()).unwrap()))
}

/// Returns the two properties of the text composite representing `text`.
fn text_properties(text: &str) -> [Object; 2] {
    let mut characters = text.chars();
    let item = characters.next().unwrap();
    let tail = characters.as_str();

    [
        construction(Atom::LIST_ITEM.into(), character_object(item)),
        construction(
            Atom::LIST_TAIL.into(),
            if tail.is_empty() {
                Composite::empty().into()
            } else {
                text_object(tail)
            },
        ),
    ]
}

#[test]
fn specializes_text_composites() {
    let properties = text_properties("hello");
    let composite = Composite::from(properties.clone());

    assert_eq!(
        composite,
        Composite::Text(TextComposite::new("hello".into()).unwrap())
    );
    assert_eq!(composite.size(), 2);
    assert!(composite.has(&properties[0]));
    assert!(composite.has(&properties[1]));

    // Re-specializing the properties must give back the same composite. `add_object`
    // and `remove_object` rely on this.
    let mut rebuilt_properties: Vec<_> = composite.clone().properties().collect();
    assert_eq!(
        Composite::from(rebuilt_properties.as_mut_slice()),
        composite
    );
}

#[test]
fn specializes_single_character_text_composites() {
    let properties = text_properties("x");
    let composite = Composite::from(properties.clone());

    assert_eq!(
        composite,
        Composite::Text(TextComposite::new("x".into()).unwrap())
    );
    assert_eq!(composite.size(), 2);
    assert!(composite.has(&properties[0]));
    assert!(composite.has(&properties[1]));
}

#[test]
fn does_not_specialize_ill_formed_text_composites() {
    let item = construction(Atom::LIST_ITEM.into(), character_object('a'));

    // The tail is neither text nor the empty composite.
    let non_text_tail = construction(
        Atom::LIST_TAIL.into(),
        Composite::from([Atom::ZERO.into()]).into(),
    );
    assert!(matches!(
        Composite::from([item.clone(), non_text_tail]),
        Composite::Any(_)
    ));

    // The item is not a character.
    let non_character_item = construction(Atom::LIST_ITEM.into(), Atom::ZERO.into());
    let empty_tail = construction(Atom::LIST_TAIL.into(), Composite::empty().into());
    assert!(matches!(
        Composite::from([non_character_item, empty_tail.clone()]),
        Composite::Any(_)
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
        Composite::from([unspecialized_item, empty_tail.clone()]),
        Composite::Any(_)
    ));

    // Both slots are `list_item`; there is no `list_tail`.
    let second_item = construction(Atom::LIST_ITEM.into(), character_object('b'));
    assert!(matches!(
        Composite::from([item, second_item]),
        Composite::Any(_)
    ));

    // A composite with more than two properties is never text.
    assert!(matches!(
        Composite::from([empty_tail, Atom::ZERO.into(), Atom::LIST_ITEM.into()]),
        Composite::Any(_)
    ));
}
