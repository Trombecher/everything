use crate::{
    base::BASE,
    model::{Atom, Object},
};

use super::super::*;
use super::common::*;

fn new_set(items: impl IntoIterator<Item = Node>) -> Node {
    Node::NewSet {
        items: items.into_iter().map(Into::into).collect(),
    }
}

fn evaluate_new_set(items: impl IntoIterator<Item = Node>) -> Object {
    evaluate(&new_set(items), &BASE).into_object(&BASE)
}

#[test]
fn new_set_constructs_a_set() {
    assert_eq!(
        evaluate_new_set([Node::Literal(integer(1)), Node::Literal(integer(2))]),
        set_of([integer(1), integer(2)])
    );
}

#[test]
fn new_set_with_a_single_item_is_still_a_set() {
    assert_eq!(
        evaluate_new_set([Node::Literal(integer(1))]),
        set_of([integer(1)])
    );
}

#[test]
fn children_are_evaluated() {
    let items = [
        Node::Add {
            first: Node::Literal(integer(1)).into(),
            second: Node::Literal(integer(2)).into(),
        },
        Node::Multiply {
            first: Node::Literal(integer(2)).into(),
            second: Node::Literal(integer(3)).into(),
        },
    ];

    assert_eq!(evaluate_new_set(items), set_of([integer(3), integer(6)]));
}

#[test]
fn equal_results_are_merged() {
    let items = [
        Node::Literal(integer(1)),
        Node::Add {
            first: Node::Literal(integer(0)).into(),
            second: Node::Literal(integer(1)).into(),
        },
    ];

    assert_eq!(evaluate_new_set(items), set_of([integer(1)]));
}

#[test]
fn nested_sets_are_items() {
    let inner = set_of([integer(1), integer(2)]);

    assert_eq!(
        evaluate_new_set([Node::Literal(inner.clone()), Node::Literal(integer(3))]),
        set_of([inner, integer(3)])
    );
}

#[test]
fn deferred_children_are_materialized() {
    let union = Node::SetUnion {
        first: Node::Literal(set_of([integer(1)])).into(),
        second: Node::Literal(set_of([integer(2)])).into(),
    };

    assert_eq!(
        evaluate_new_set([union, Node::Literal(integer(3))]),
        set_of([set_of([integer(1), integer(2)]), integer(3)])
    );
}

#[test]
fn new_set_includes_undefined_items() {
    let failing = Node::Add {
        first: Node::Literal(Atom(42).into()).into(),
        second: Node::Literal(Atom(43).into()).into(),
    };

    assert_eq!(
        evaluate_new_set([Node::Literal(integer(1)), failing]),
        set_of([integer(1), Atom::UNDEFINED.into()])
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
        set_of([integer(5), integer(6)])
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
        set_of([Node::function(Node::Literal(integer(5))).into_object()])
    );
}
