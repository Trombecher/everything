use crate::{
    base::BASE,
    model::{Atom, Composite, Object},
};

use super::super::*;
use super::common::*;

fn new_composite(values: impl IntoIterator<Item = Node>) -> Node {
    Node::NewComposite {
        values: values.into_iter().map(Into::into).collect(),
    }
}

fn evaluate_new_composite(values: impl IntoIterator<Item = Node>) -> Object {
    evaluate(&new_composite(values), &BASE).into_object(&BASE)
}

#[test]
fn new_composite_constructs_a_composite() {
    assert_eq!(
        evaluate_new_composite([Node::Literal(integer(1)), Node::Literal(integer(2))]),
        Object::Composite(Composite::from([integer(1), integer(2)]))
    );
}

#[test]
fn new_composite_with_a_single_value_is_still_a_composite() {
    assert_eq!(
        evaluate_new_composite([Node::Literal(integer(1))]),
        Object::Composite(Composite::from([integer(1)]))
    );
}

#[test]
fn children_are_evaluated() {
    let values = [
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
        evaluate_new_composite(values),
        Object::Composite(Composite::from([integer(3), integer(6)]))
    );
}

#[test]
fn equal_results_are_merged() {
    let values = [
        Node::Literal(integer(1)),
        Node::Add {
            first: Node::Literal(integer(0)).into(),
            second: Node::Literal(integer(1)).into(),
        },
    ];

    assert_eq!(
        evaluate_new_composite(values),
        Object::Composite(Composite::from([integer(1)]))
    );
}

#[test]
fn nested_composites_are_values() {
    let inner = Object::Composite(Composite::from([integer(1), integer(2)]));

    assert_eq!(
        evaluate_new_composite([Node::Literal(inner.clone()), Node::Literal(integer(3))]),
        Object::Composite(Composite::from([inner, integer(3)]))
    );
}

#[test]
fn deferred_children_are_materialized() {
    let union = Node::Union {
        first: Node::Literal(Object::Composite(Composite::from([integer(1)]))).into(),
        second: Node::Literal(Object::Composite(Composite::from([integer(2)]))).into(),
    };

    assert_eq!(
        evaluate_new_composite([union, Node::Literal(integer(3))]),
        Object::Composite(Composite::from([
            Object::Composite(Composite::from([integer(1), integer(2)])),
            integer(3),
        ]))
    );
}

#[test]
fn new_composite_includes_undefined_values() {
    let failing = Node::Add {
        first: Node::Literal(Atom(42).into()).into(),
        second: Node::Literal(Atom(43).into()).into(),
    };

    assert_eq!(
        evaluate_new_composite([Node::Literal(integer(1)), failing]),
        Object::Composite(Composite::from([integer(1), Atom::UNDEFINED.into()]))
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
        Object::Composite(Composite::from([integer(5), integer(6)]))
    );
}

#[test]
fn new_composite_captures_the_evaluation_context() {
    // `x |-> { y |-> x }`; the inner function escapes the call and must
    // capture the parameter of the outer function.
    let escaped_function = Node::function(Node::Parameter { depth: 1 });

    let outer_function = Node::function(new_composite([escaped_function]));

    let node = Node::Call {
        callee: outer_function.into(),
        with: Node::Literal(integer(5)).into(),
    };

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        Object::Composite(Composite::from([
            Node::function(Node::Literal(integer(5))).into_object()
        ]))
    );
}
