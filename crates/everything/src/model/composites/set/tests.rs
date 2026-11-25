use crate::model::{Atom, Composite, CompositeApi, Construction, Object};

fn construction(constructor: Object, value: Object) -> Object {
    Construction::new(constructor, value).into()
}

fn contains(item: Object) -> Object {
    construction(Atom::CONTAINS.into(), item)
}

#[test]
fn empty_composite_is_the_empty_set() {
    let composite = Composite::from([]);

    let Composite::Set(set) = &composite else {
        panic!("an empty composite must specialize to the empty set");
    };

    assert_eq!(set.items(), []);
    assert_eq!(set.rest(), []);
    assert_eq!(composite.size(), 0);
}

#[test]
fn specializes_contains_composites() {
    let one = Object::Atom(Atom(10));
    let two = Object::Atom(Atom(20));

    let composite = Composite::from([contains(two.clone()), contains(one.clone())]);

    let Composite::Set(set) = &composite else {
        panic!("a composite of `contains` constructions must specialize to a set");
    };

    assert_eq!(set.items(), [one, two].as_slice());
    assert_eq!(set.rest(), []);
    assert_eq!(composite.size(), 2);
}

#[test]
fn open_sets_preserve_other_members_as_rest() {
    let item = construction(Atom(10).into(), Atom(20).into());
    let other = construction(Atom(30).into(), Atom(40).into());
    let atom = Object::Atom(Atom(50));

    let composite = Composite::from([contains(item.clone()), other.clone(), atom.clone()]);

    let Composite::Set(set) = &composite else {
        panic!("unrelated members must not disqualify a set");
    };

    assert_eq!(set.items(), [item].as_slice());
    assert_eq!(set.rest(), [atom, other].as_slice());
    assert_eq!(composite.size(), 3);
}

#[test]
fn composites_without_contains_members_are_not_sets() {
    let composite = Composite::from([Object::Atom(Atom(10)), Object::Atom(Atom(20))]);

    assert!(matches!(composite, Composite::Any(_)));
}

#[test]
fn single_contains_member_is_a_one_construction_composite() {
    let composite = Composite::from([contains(Object::Atom(Atom(10)))]);

    // A single-member composite is represented more optimally by the
    // one-construction specialization, even when the member is a set item.
    assert!(matches!(composite, Composite::OneConstruction(_)));
}

#[test]
fn multiple_contains_members_form_a_set() {
    let composite = Composite::from([
        contains(Object::Atom(Atom(10))),
        contains(Object::Atom(Atom(20))),
    ]);

    assert!(matches!(composite, Composite::Set(_)));
}

#[test]
fn duplicate_contains_members_are_deduplicated() {
    let item = Object::Atom(Atom(10));
    let other = Object::Atom(Atom(20));

    let composite = Composite::from([
        contains(item.clone()),
        contains(item.clone()),
        contains(other.clone()),
    ]);

    let Composite::Set(set) = &composite else {
        panic!("a composite of `contains` constructions must specialize to a set");
    };

    assert_eq!(set.items(), [item, other].as_slice());
    assert_eq!(composite.size(), 2);
}

#[test]
fn has_distinguishes_items_from_contains_constructions() {
    let item = Object::Atom(Atom(10));
    let other = Object::Atom(Atom(20));

    let composite = Composite::from([contains(item.clone()), other.clone()]);

    let Composite::Set(set) = &composite else {
        panic!("a composite of `contains` constructions must specialize to a set");
    };

    assert!(set.has(&contains(item.clone())));
    assert!(!set.has(&item));
    assert!(set.has(&other));
    assert!(!set.has(&Atom(30).into()));
}

#[test]
fn members_are_iterated_in_ascending_order() {
    // The rest contains constructions with constructors below and above
    // `contains`, so the item constructions must be merged with the rest
    // instead of being yielded in separate blocks.
    let lower = construction(Atom::ZERO.into(), Object::Atom(Atom(10)));
    let upper = construction(Atom(u128::MAX).into(), Object::Atom(Atom(20)));
    let atom = Object::Atom(Atom(30));

    let composite = Composite::from([contains(Object::Atom(Atom(40))), lower, upper, atom]);

    let properties: Vec<_> = composite.properties().collect();

    assert!(
        properties.windows(2).all(|pair| pair[0] < pair[1]),
        "properties are not ascending: {properties:?}"
    );
}

#[test]
fn input_order_does_not_change_the_set() {
    let item = Object::Atom(Atom(10));
    let other = Object::Atom(Atom(20));

    assert_eq!(
        Composite::from([contains(item.clone()), other.clone()]),
        Composite::from([other, contains(item)])
    );
}

#[test]
fn adding_and_removing_objects_keeps_the_canonical_set() {
    let item = Object::Atom(Atom(10));
    let other = Object::Atom(Atom(20));

    let composite = Composite::empty().add_object(contains(item.clone()));
    assert_eq!(composite, Composite::from([contains(item.clone())]));

    let composite = composite.add_object(other.clone());
    assert_eq!(
        composite,
        Composite::from([contains(item.clone()), other.clone()])
    );

    let composite = composite.remove_object(&other);
    assert_eq!(composite, Composite::from([contains(item)]));
}
