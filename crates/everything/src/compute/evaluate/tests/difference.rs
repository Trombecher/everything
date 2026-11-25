use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn difference() {
    let result = evaluate(
        &Node::Difference {
            first: Node::Literal(Object::Composite(integer_composite())).into(),
            second: Node::Literal(Object::Composite(Composite::from([integer(2)]))).into(),
        },
        &BASE,
    );

    assert!(result.is_composite());
    assert_eq!(result.size(&BASE), 2);
    assert_eq!(
        result.iter(&BASE).collect::<Vec<_>>(),
        vec![integer(1), integer(3)]
    );
    assert!(!result.has(&integer(2), &BASE));
}

#[test]
fn difference_of_a_composite_with_itself_is_empty() {
    let composite = Node::Literal(Object::Composite(integer_composite()));

    let result = evaluate(
        &Node::Difference {
            first: composite.clone().into(),
            second: composite.into(),
        },
        &BASE,
    )
    .into_object(&BASE);

    assert_eq!(result, Object::Composite(Composite::empty()));
}

#[test]
fn difference_does_not_evaluate_the_second_child_when_the_first_is_empty() {
    let result = evaluate(
        &Node::Difference {
            first: Node::Literal(Object::Composite(Composite::empty())).into(),
            // A non-composite would make the result undefined if it were evaluated.
            second: Node::Literal(Object::Atom(Atom(42))).into(),
        },
        &BASE,
    );

    assert_eq!(
        result.into_object(&BASE),
        Object::Composite(Composite::empty())
    );
}

#[test]
fn difference_short_circuits_for_an_empty_deferred_first_composite() {
    let empty_filter = Node::Filter {
        composite: Node::Literal(Object::Composite(integer_composite())).into(),
        filter_function: always_false_predicate().into(),
    };

    let result = evaluate(
        &Node::Difference {
            first: empty_filter.into(),
            second: Node::Literal(Object::Atom(Atom(42))).into(),
        },
        &BASE,
    );

    assert_eq!(
        result.into_object(&BASE),
        Object::Composite(Composite::empty())
    );
}

#[test]
fn difference_of_deferred_composites() {
    let union = Node::Union {
        first: Node::Literal(Object::Composite(Composite::from([integer(1), integer(2)]))).into(),
        second: Node::Literal(Object::Composite(Composite::from([integer(3)]))).into(),
    };

    let result = evaluate(
        &Node::Difference {
            first: union.into(),
            second: Node::Literal(Object::Composite(Composite::from([integer(2)]))).into(),
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
        composite: Node::Literal(Object::Composite(Composite::from([integer(1), integer(2)])))
            .into(),
        // Maps both elements to `1`.
        mapper_function: constant_predicate(integer(1)).into(),
    };

    let result = evaluate(
        &Node::Difference {
            first: map.into(),
            second: Node::Literal(Object::Composite(Composite::empty())).into(),
        },
        &BASE,
    );

    assert_eq!(result.iter(&BASE).collect::<Vec<_>>(), vec![integer(1)]);
    assert_eq!(result.size(&BASE), 1);
}

#[test]
fn difference_requires_composites() {
    let empty_composite = Node::Literal(Object::Composite(Composite::empty()));
    let non_composite = Node::Literal(Object::Atom(Atom(42)));
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());

    // The first child is not a composite.
    for first in [non_composite.clone(), construction] {
        assert_eq!(
            evaluate(
                &Node::Difference {
                    first: first.into(),
                    second: empty_composite.clone().into(),
                },
                &BASE,
            )
            .into_object(&BASE),
            Atom::UNDEFINED.into()
        );
    }

    // The first child is non-empty and the second child is not a composite.
    assert_eq!(
        evaluate(
            &Node::Difference {
                first: Node::Literal(Object::Composite(Composite::from([integer(1)]))).into(),
                second: non_composite.into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}
