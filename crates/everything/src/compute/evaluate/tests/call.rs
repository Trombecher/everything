use num_bigint::BigInt;

use crate::{
    base::BASE,
    model::{Atom, Object, Set},
};

use super::super::*;
use super::common::*;

#[test]
fn curried_add_function() {
    let first_call = Node::Call {
        callee: add_folder().into(),
        with: Node::Literal(integer(0)).into(),
    };
    let second_call = Node::Call {
        callee: first_call.into(),
        with: Node::Literal(integer(5)).into(),
    };

    assert_eq!(evaluate(&second_call, &BASE).into_object(&BASE), integer(5));
}

#[test]
fn call_parses_deferred_callee_as_a_node() {
    let function = identity_function().into_object();
    let empty_set = Node::Literal(Object::Set(Set::empty()));
    let callee = Node::Union {
        first: Node::Literal(function).into(),
        second: empty_set.into(),
    };

    assert_eq!(
        evaluate(
            &Node::Call {
                callee: callee.into(),
                with: Node::Literal(integer(5)).into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        integer(5)
    );
}

#[test]
fn call_with_undefined_callee_is_undefined() {
    assert_eq!(
        evaluate(
            &Node::Call {
                callee: Node::Literal(Atom::UNDEFINED.into()).into(),
                with: Node::Literal(integer(1)).into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn call_rejects_sets() {
    let union = Node::Union {
        first: Node::Literal(Object::Set(Set::empty())).into(),
        second: Node::Literal(Object::Set(Set::empty())).into(),
    };

    assert_eq!(
        evaluate(
            &Node::Call {
                callee: union.into(),
                with: Node::Literal(BigInt::from(1).into()).into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}
