use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::common::*;
use crate::compute::*;

#[test]
fn set_intersection() {
    let result = evaluate(
        &Node::SetIntersection {
            first: Node::Literal(Object::Composite(integer_set())).into(),
            second: Node::Literal(set_of([integer(2), integer(3), integer(4)])).into(),
        },
        &BASE,
    );

    assert!(result.is_composite());
    assert_eq!(set(&result).item_count(&BASE), 2);
    assert_eq!(
        set(&result).iter(&BASE).collect::<Vec<_>>(),
        vec![integer(2), integer(3)]
    );
    assert!(set(&result).has_item(&integer(2), &BASE));
    assert!(!set(&result).has_item(&integer(4), &BASE));
}

#[test]
fn set_intersection_of_deferred_sets() {
    let union = Node::SetUnion {
        first: Node::Literal(set_of([integer(1), integer(2)])).into(),
        second: Node::Literal(set_of([integer(3)])).into(),
    };

    let result = evaluate(
        &Node::SetIntersection {
            first: union.into(),
            second: Node::Literal(set_of([integer(2), integer(3)])).into(),
        },
        &BASE,
    );

    assert_eq!(set(&result).item_count(&BASE), 2);
    assert_eq!(
        set(&result).iter(&BASE).collect::<Vec<_>>(),
        vec![integer(2), integer(3)]
    );
}

#[test]
fn set_intersection_of_disjoint_sets_is_empty() {
    let result = evaluate(
        &Node::SetIntersection {
            first: Node::Literal(set_of([integer(1)])).into(),
            second: Node::Literal(set_of([integer(2)])).into(),
        },
        &BASE,
    )
    .into_object(&BASE);

    assert_eq!(result, Object::Composite(Composite::empty()));
}

#[test]
fn set_intersection_requires_a_set() {
    let empty_set = Node::Literal(Object::Composite(Composite::empty()));
    let invalid = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());

    assert_eq!(
        evaluate(
            &Node::SetIntersection {
                first: invalid.into(),
                second: empty_set.clone().into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );

    // An atom is a set with no items.
    assert_eq!(
        evaluate(
            &Node::SetIntersection {
                first: Node::Literal(Object::Atom(Atom(42))).into(),
                second: empty_set.into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Object::Composite(Composite::empty())
    );
}
