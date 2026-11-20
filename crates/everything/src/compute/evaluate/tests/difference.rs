use crate::{
    base::BASE,
    model::{Atom, Construction, Object, Set},
};

use super::super::*;
use super::common::*;

#[test]
fn difference() {
    let result = evaluate(
        &Node::Difference {
            first: Node::Literal(Object::Set(integer_set())).into(),
            second: Node::Literal(Object::Set(Set::from([integer(2)]))).into(),
        },
        &BASE,
    );

    assert!(result.is_set());
    assert_eq!(result.size(&BASE), 2);
    assert_eq!(
        result.iter(&BASE).collect::<Vec<_>>(),
        vec![integer(1), integer(3)]
    );
    assert!(!result.has(&integer(2), &BASE));
}

#[test]
fn difference_of_a_set_with_itself_is_empty() {
    let set = Node::Literal(Object::Set(integer_set()));

    let result = evaluate(
        &Node::Difference {
            first: set.clone().into(),
            second: set.into(),
        },
        &BASE,
    )
    .into_object(&BASE);

    assert_eq!(result, Object::Set(Set::empty()));
}

#[test]
fn difference_does_not_evaluate_the_second_child_when_the_first_is_empty() {
    let result = evaluate(
        &Node::Difference {
            first: Node::Literal(Object::Set(Set::empty())).into(),
            // A non-set would make the result undefined if it were evaluated.
            second: Node::Literal(Object::Atom(Atom(42))).into(),
        },
        &BASE,
    );

    assert_eq!(result.into_object(&BASE), Object::Set(Set::empty()));
}

#[test]
fn difference_short_circuits_for_an_empty_deferred_first_set() {
    let empty_filter = Node::Filter {
        set: Node::Literal(Object::Set(integer_set())).into(),
        filter_function: always_false_predicate().into(),
    };

    let result = evaluate(
        &Node::Difference {
            first: empty_filter.into(),
            second: Node::Literal(Object::Atom(Atom(42))).into(),
        },
        &BASE,
    );

    assert_eq!(result.into_object(&BASE), Object::Set(Set::empty()));
}

#[test]
fn difference_of_deferred_sets() {
    let union = Node::Union {
        first: Node::Literal(Object::Set(Set::from([integer(1), integer(2)]))).into(),
        second: Node::Literal(Object::Set(Set::from([integer(3)]))).into(),
    };

    let result = evaluate(
        &Node::Difference {
            first: union.into(),
            second: Node::Literal(Object::Set(Set::from([integer(2)]))).into(),
        },
        &BASE,
    );

    assert_eq!(
        result.iter(&BASE).collect::<Vec<_>>(),
        vec![integer(1), integer(3)]
    );
}

#[test]
fn difference_of_a_map_deduplicates() {
    let map = Node::Map {
        set: Node::Literal(Object::Set(Set::from([integer(1), integer(2)]))).into(),
        // Maps both elements to `1`.
        mapper_function: constant_predicate(integer(1)).into(),
    };

    let result = evaluate(
        &Node::Difference {
            first: map.into(),
            second: Node::Literal(Object::Set(Set::empty())).into(),
        },
        &BASE,
    );

    assert_eq!(result.iter(&BASE).collect::<Vec<_>>(), vec![integer(1)]);
    assert_eq!(result.size(&BASE), 1);
}

#[test]
fn difference_requires_sets() {
    let empty_set = Node::Literal(Object::Set(Set::empty()));
    let non_set = Node::Literal(Object::Atom(Atom(42)));
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());

    // The first child is not a set.
    for first in [non_set.clone(), construction] {
        assert_eq!(
            evaluate(
                &Node::Difference {
                    first: first.into(),
                    second: empty_set.clone().into(),
                },
                &BASE,
            )
            .into_object(&BASE),
            Atom::UNDEFINED.into()
        );
    }

    // The first child is non-empty and the second child is not a set.
    assert_eq!(
        evaluate(
            &Node::Difference {
                first: Node::Literal(Object::Set(Set::from([integer(1)]))).into(),
                second: non_set.into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}
