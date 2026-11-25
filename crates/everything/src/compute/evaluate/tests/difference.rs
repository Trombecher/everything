use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn set_difference() {
    let result = evaluate(
        &Node::SetDifference {
            first: Node::Literal(Object::Composite(integer_set())).into(),
            second: Node::Literal(set_of([integer(2)])).into(),
        },
        &BASE,
    );

    assert!(result.is_composite());
    assert_eq!(set(&result).item_count(&BASE), 2);
    assert_eq!(
        set(&result).iter(&BASE).collect::<Vec<_>>(),
        vec![integer(1), integer(3)]
    );
    assert!(!set(&result).has_item(&integer(2), &BASE));
}

#[test]
fn set_difference_of_a_set_with_itself_is_empty() {
    let set = Node::Literal(Object::Composite(integer_set()));

    let result = evaluate(
        &Node::SetDifference {
            first: set.clone().into(),
            second: set.into(),
        },
        &BASE,
    )
    .into_object(&BASE);

    assert_eq!(result, Object::Composite(Composite::empty()));
}

#[test]
fn set_difference_does_not_evaluate_the_second_child_when_the_first_is_empty() {
    let result = evaluate(
        &Node::SetDifference {
            first: Node::Literal(Object::Composite(Composite::empty())).into(),
            // A construction would make the result undefined if it were evaluated.
            second: Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()).into(),
        },
        &BASE,
    );

    assert_eq!(
        result.into_object(&BASE),
        Object::Composite(Composite::empty())
    );
}

#[test]
fn set_difference_short_circuits_for_an_empty_deferred_first_set() {
    let empty_filter = Node::SetFilter {
        set: Node::Literal(Object::Composite(integer_set())).into(),
        filter_function: always_false_predicate().into(),
    };

    let result = evaluate(
        &Node::SetDifference {
            first: empty_filter.into(),
            second: Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()).into(),
        },
        &BASE,
    );

    assert_eq!(
        result.into_object(&BASE),
        Object::Composite(Composite::empty())
    );
}

#[test]
fn set_difference_of_deferred_sets() {
    let union = Node::SetUnion {
        first: Node::Literal(set_of([integer(1), integer(2)])).into(),
        second: Node::Literal(set_of([integer(3)])).into(),
    };

    let result = evaluate(
        &Node::SetDifference {
            first: union.into(),
            second: Node::Literal(set_of([integer(2)])).into(),
        },
        &BASE,
    );

    assert_eq!(
        set(&result).iter(&BASE).collect::<Vec<_>>(),
        vec![integer(1), integer(3)]
    );
}

#[test]
fn set_difference_of_a_map_deduplicates() {
    let map = Node::SetMap {
        set: Node::Literal(set_of([integer(1), integer(2)])).into(),
        // Maps both items to `1`.
        mapper_function: constant_predicate(integer(1)).into(),
    };

    let result = evaluate(
        &Node::SetDifference {
            first: map.into(),
            second: Node::Literal(Object::Composite(Composite::empty())).into(),
        },
        &BASE,
    );

    assert_eq!(
        set(&result).iter(&BASE).collect::<Vec<_>>(),
        vec![integer(1)]
    );
    assert_eq!(set(&result).item_count(&BASE), 1);
}

#[test]
fn set_difference_requires_a_set() {
    let empty_set = Node::Literal(Object::Composite(Composite::empty()));
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());

    assert_eq!(
        evaluate(
            &Node::SetDifference {
                first: construction.into(),
                second: empty_set.clone().into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );

    // An atom is a set with no items, so the difference is empty.
    assert_eq!(
        evaluate(
            &Node::SetDifference {
                first: Node::Literal(Object::Atom(Atom(42))).into(),
                second: empty_set.into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Object::Composite(Composite::empty())
    );

    // The first set is non-empty and the second child is not a set.
    assert_eq!(
        evaluate(
            &Node::SetDifference {
                first: Node::Literal(set_of([integer(1)])).into(),
                second: Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into())
                    .into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}
