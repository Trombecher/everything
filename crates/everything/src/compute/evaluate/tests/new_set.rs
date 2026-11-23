use crate::{
    base::BASE,
    model::{Atom, Object, Set},
};

use super::super::*;
use super::common::*;

fn new_set(values: impl IntoIterator<Item = Node>) -> Node {
    Node::NewSet {
        values: values.into_iter().map(Into::into).collect(),
    }
}

fn evaluate_new_set(values: impl IntoIterator<Item = Node>) -> Object {
    evaluate(&new_set(values), &BASE).into_object(&BASE)
}

#[test]
fn new_set_constructs_a_set() {
    assert_eq!(
        evaluate_new_set([Node::Literal(integer(1)), Node::Literal(integer(2))]),
        Object::Set(Set::from([integer(1), integer(2)]))
    );
}

#[test]
fn new_set_with_a_single_value_is_still_a_set() {
    assert_eq!(
        evaluate_new_set([Node::Literal(integer(1))]),
        Object::Set(Set::from([integer(1)]))
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
        evaluate_new_set(values),
        Object::Set(Set::from([integer(3), integer(6)]))
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
        evaluate_new_set(values),
        Object::Set(Set::from([integer(1)]))
    );
}

#[test]
fn nested_sets_are_values() {
    let inner = Object::Set(Set::from([integer(1), integer(2)]));

    assert_eq!(
        evaluate_new_set([Node::Literal(inner.clone()), Node::Literal(integer(3))]),
        Object::Set(Set::from([inner, integer(3)]))
    );
}

#[test]
fn deferred_children_are_materialized() {
    let union = Node::Union {
        first: Node::Literal(Object::Set(Set::from([integer(1)]))).into(),
        second: Node::Literal(Object::Set(Set::from([integer(2)]))).into(),
    };

    assert_eq!(
        evaluate_new_set([union, Node::Literal(integer(3))]),
        Object::Set(Set::from([
            Object::Set(Set::from([integer(1), integer(2)])),
            integer(3),
        ]))
    );
}

#[test]
fn new_set_includes_undefined_values() {
    let failing = Node::Add {
        first: Node::Literal(Atom(42).into()).into(),
        second: Node::Literal(Atom(43).into()).into(),
    };

    assert_eq!(
        evaluate_new_set([Node::Literal(integer(1)), failing]),
        Object::Set(Set::from([integer(1), Atom::UNDEFINED.into()]))
    );
}

#[test]
fn new_set_evaluates_children_in_the_evaluation_context() {
    let body = new_set([
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
        Object::Set(Set::from([integer(5), integer(6)]))
    );
}

#[test]
fn new_set_captures_the_evaluation_context() {
    // `x |-> { y |-> x }`; the inner function escapes the call and must
    // capture the parameter of the outer function.
    let escaped_function = Node::function(Node::Parameter { depth: 1 });

    let outer_function = Node::function(new_set([escaped_function]));

    let node = Node::Call {
        callee: outer_function.into(),
        with: Node::Literal(integer(5)).into(),
    };

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        Object::Set(Set::from([
            Node::function(Node::Literal(integer(5))).into_object()
        ]))
    );
}
