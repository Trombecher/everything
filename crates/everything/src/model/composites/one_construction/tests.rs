use crate::model::{Atom, CompositeApi, Construction, Object};

use super::*;

fn construction(constructor: Object, value: Object) -> Construction {
    Construction::new(constructor, value)
}

fn object(constructor: Object, value: Object) -> Object {
    construction(constructor, value).into()
}

#[test]
fn new_wraps_the_given_construction() {
    let construction = construction(Atom(10).into(), Atom(20).into());
    let composite = OneConstructionComposite::new(construction.clone());

    assert_eq!(composite.size(), 1);
    assert_eq!(
        composite.properties().collect::<Vec<_>>(),
        [Object::Construction(construction)]
    );
}

#[test]
fn has_matches_only_the_contained_construction() {
    let composite = OneConstructionComposite::new(construction(Atom(10).into(), Atom(20).into()));

    assert!(composite.has(&object(Atom(10).into(), Atom(20).into())));
    assert!(!composite.has(&object(Atom(10).into(), Atom(21).into())));
    assert!(!composite.has(&object(Atom(11).into(), Atom(20).into())));
    assert!(!composite.has(&Object::Atom(Atom(10))));
}

#[test]
fn iteration_yields_exactly_the_construction() {
    let object = object(Atom(10).into(), Atom(20).into());
    let mut properties = OneConstructionComposite::new(
        object
            .clone()
            .destructure_construction()
            .expect("the object is a construction"),
    )
    .into_iter();

    assert_eq!(properties.next(), Some(object));
    assert_eq!(properties.next(), None);
    assert_eq!(properties.next(), None);
}

#[test]
fn specialize_from_moving_accepts_exactly_one_construction() {
    let construction = construction(Atom(10).into(), Atom(20).into());
    let mut objects = [Object::Construction(construction.clone())];

    let composite = OneConstructionComposite::specialize_from_moving(&mut objects)
        .expect("one construction must specialize");

    assert_eq!(composite, OneConstructionComposite::new(construction));

    // The construction was moved out of the slice.
    assert_eq!(objects, [Object::default()]);
}

#[test]
fn specialize_from_moving_rejects_other_shapes() {
    let mut empty: [Object; 0] = [];
    assert_eq!(
        OneConstructionComposite::specialize_from_moving(&mut empty),
        None
    );

    let mut atom = [Object::Atom(Atom(10))];
    assert_eq!(
        OneConstructionComposite::specialize_from_moving(&mut atom),
        None
    );

    let mut two_constructions = [
        object(Atom(10).into(), Atom(20).into()),
        object(Atom(30).into(), Atom(40).into()),
    ];
    assert_eq!(
        OneConstructionComposite::specialize_from_moving(&mut two_constructions),
        None
    );

    let mut construction_and_atom = [
        object(Atom(10).into(), Atom(20).into()),
        Object::Atom(Atom(30)),
    ];
    assert_eq!(
        OneConstructionComposite::specialize_from_moving(&mut construction_and_atom),
        None
    );
}

#[test]
fn debug_fmt_renders_a_set_with_one_entry() {
    let construction = construction(Atom(10).into(), Atom(20).into());
    let composite = OneConstructionComposite::new(construction.clone());

    assert_eq!(format!("{composite:?}"), format!("{{{construction:?}}}"));
}
