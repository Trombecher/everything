use imbl::OrdMap;
use num_bigint::BigInt;

use super::super::RawNode;
use crate::{
    knowledge::Knowledge,
    model::{Atom, Composite, Object},
};

#[test]
fn raw_node_constructors_round_trip() {
    let knowledge = Knowledge { map: OrdMap::new() };

    let nodes = [
        RawNode::function(RawNode::literal(Object::new_integer(BigInt::from(1)))),
        RawNode::and(
            RawNode::literal(Object::new_integer(BigInt::from(1))),
            RawNode::literal(Object::new_integer(BigInt::from(2))),
        ),
        RawNode::parameter(1),
        RawNode::function_self(0),
        RawNode::if_then_else(
            RawNode::literal(true),
            RawNode::literal(false),
            RawNode::literal(Atom::UNDEFINED),
        ),
        RawNode::set_fold(
            RawNode::literal(Object::Composite(Composite::empty())),
            RawNode::function(RawNode::parameter(0)),
            RawNode::literal(BigInt::ZERO),
        ),
        RawNode::new_set([
            RawNode::literal(Object::new_integer(BigInt::from(1))),
            RawNode::literal(Object::new_integer(BigInt::from(2))),
        ]),
        RawNode::new_composite([RawNode::literal(Object::new_integer(BigInt::from(1)))]),
        RawNode::composite_from_set(RawNode::literal(Object::Composite(Composite::empty()))),
        RawNode::is_node(RawNode::literal(Object::new_integer(BigInt::from(1)))),
    ];

    for raw in nodes {
        let object = raw.into_object();
        let parsed = RawNode::new(&object, &knowledge).unwrap();

        assert_eq!(parsed.into_object(), object);
    }
}
