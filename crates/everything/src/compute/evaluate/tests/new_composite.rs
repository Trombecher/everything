use crate::{
    base::BASE,
    model::{Atom, Construction, Object},
};

use super::super::*;
use super::common::*;

fn new_composite(properties: impl IntoIterator<Item = Node>) -> Node {
    Node::NewComposite {
        properties: properties.into_iter().map(Into::into).collect(),
    }
}

fn evaluate_new_composite(properties: impl IntoIterator<Item = Node>) -> Object {
    evaluate(&new_composite(properties), &BASE).into_object(&BASE)
}

#[test]
fn new_composite_constructs_a_composite() {
    assert_eq!(
        evaluate_new_composite([Node::Literal(integer(1)), Node::Literal(integer(2))]),
        composite_of([integer(1), integer(2)])
    );
}

#[test]
fn new_composite_with_a_single_property_is_still_a_composite() {
    assert_eq!(
        evaluate_new_composite([Node::Literal(integer(1))]),
        composite_of([integer(1)])
    );
}

#[test]
fn children_are_evaluated() {
    let properties = [
        Node::Add {
            first: Node::Literal(integer(1)).into(),
            second: Node::Literal(integer(2)).into(),
        },
        Node::Multiply {
            first: Node::Literal(integer(2)).into(),
            second: Node::Literal(integer(3)).into(),
        },
    ];

    assert_eq!(
        evaluate_new_composite(properties),
        composite_of([integer(3), integer(6)])
    );
}

#[test]
fn equal_results_are_merged() {
    let properties = [
        Node::Literal(integer(1)),
        Node::Add {
            first: Node::Literal(integer(0)).into(),
            second: Node::Literal(integer(1)).into(),
        },
    ];

    assert_eq!(
        evaluate_new_composite(properties),
        composite_of([integer(1)])
    );
}

#[test]
fn nested_composites_are_properties() {
    let inner = composite_of([integer(1), integer(2)]);

    assert_eq!(
        evaluate_new_composite([Node::Literal(inner.clone()), Node::Literal(integer(3))]),
        composite_of([inner, integer(3)])
    );
}

#[test]
fn deferred_children_are_materialized() {
    let union = Node::SetUnion {
        first: Node::Literal(set_of([integer(1)])).into(),
        second: Node::Literal(set_of([integer(2)])).into(),
    };

    assert_eq!(
        evaluate_new_composite([union, Node::Literal(integer(3))]),
        composite_of([set_of([integer(1), integer(2)]), integer(3)])
    );
}

#[test]
fn new_composite_includes_undefined_properties() {
    let failing = Node::Add {
        first: Node::Literal(Atom(42).into()).into(),
        second: Node::Literal(Atom(43).into()).into(),
    };

    assert_eq!(
        evaluate_new_composite([Node::Literal(integer(1)), failing]),
        composite_of([integer(1), Atom::UNDEFINED.into()])
    );
}

#[test]
fn new_composite_does_not_wrap_children_as_set_items() {
    let result = evaluate(
        &new_composite([Node::Literal(integer(1)), Node::Literal(integer(2))]),
        &BASE,
    );

    assert_eq!(set(&result).item_count(&BASE), 0);
    assert_eq!(
        result.into_object(&BASE),
        composite_of([integer(1), integer(2)])
    );
}

#[test]
fn a_contains_child_becomes_a_set_item() {
    let contains = Construction::new(Atom::CONTAINS.into(), integer(1)).into();

    assert_eq!(
        evaluate_new_composite([Node::Literal(contains)]),
        set_of([integer(1)])
    );
}

#[test]
fn new_composite_evaluates_children_in_the_evaluation_context() {
    let body = new_composite([
        Node::Parameter { depth: 0 },
        Node::Add {
            first: Node::Parameter { depth: 0 }.into(),
            second: Node::Literal(integer(1)).into(),
        },
    ]);

    let node = Node::Call {
        callee: Node::function(body).into(),
        with: Node::Literal(integer(5)).into(),
    };

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        composite_of([integer(5), integer(6)])
    );
}

#[test]
fn new_composite_captures_the_evaluation_context() {
    // `x |-> ⟨y |-> x⟩`; the inner function escapes the call and must capture
    // the parameter of the outer function.
    let escaped_function = Node::function(Node::Parameter { depth: 1 });

    let outer_function = Node::function(new_composite([escaped_function]));

    let node = Node::Call {
        callee: outer_function.into(),
        with: Node::Literal(integer(5)).into(),
    };

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        composite_of([Node::function(Node::Literal(integer(5))).into_object()])
    );
}
