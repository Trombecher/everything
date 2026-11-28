use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

fn is_node(inner: Node) -> Object {
    evaluate(&Node::IsNode(inner.into()), &BASE).into_object(&BASE)
}

#[test]
fn node_objects_are_nodes() {
    let node_object = Node::Literal(integer(1)).into_object();

    assert_eq!(is_node(Node::Literal(node_object)), Atom::TRUE.into());
}

#[test]
fn function_values_are_nodes() {
    assert_eq!(is_node(identity_function()), Atom::TRUE.into());
}

#[test]
fn plain_data_is_not_a_node() {
    assert_eq!(is_node(Node::Literal(integer(1))), Atom::FALSE.into());
}

#[test]
fn undefined_is_not_a_node() {
    assert_eq!(
        is_node(Node::Literal(Object::Atom(Atom::UNDEFINED))),
        Atom::FALSE.into()
    );
}

#[test]
fn constructions_are_not_nodes() {
    assert_eq!(
        is_node(Node::Literal(
            Construction::new(Atom(1).into(), Atom(2).into()).into()
        )),
        Atom::FALSE.into()
    );
}

#[test]
fn nodes_with_invalid_children_are_not_nodes() {
    let object =
        Composite::from([Construction::new(Atom::NODE_SET_COUNT.into(), integer(1)).into()]).into();

    assert_eq!(is_node(Node::Literal(object)), Atom::FALSE.into());
}

#[test]
fn deferred_sets_are_materialised_before_the_check() {
    let union = Node::SetUnion {
        first: Node::Literal(Object::Composite(Composite::empty())).into(),
        second: Node::Literal(Object::Composite(Composite::empty())).into(),
    };

    assert_eq!(is_node(union), Atom::FALSE.into());
}
