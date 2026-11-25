use num_bigint::BigInt;

use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn set_filter() {
    let node = Node::SetFilter {
        set: Node::Literal(integer_set().into()).into(),
        filter_function: Node::function(Node::Equal {
            first: Node::Parameter { depth: 0 }.into(),
            second: Node::Literal(Object::new_integer(BigInt::from(2))).into(),
        })
        .into(),
    };

    let result = evaluate(&node, &BASE);
    let values = set(&result).iter(&BASE).collect::<Vec<_>>();

    assert_eq!(values, vec![Object::new_integer(BigInt::from(2))]);
}

#[test]
fn set_filter_requires_a_set() {
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());
    let identity = identity_function();

    assert_eq!(
        evaluate(
            &Node::SetFilter {
                set: construction.into(),
                filter_function: identity.clone().into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );

    // An atom and a function object are sets with no items.
    for set in [Node::Literal(Object::Atom(Atom(42))), identity_function()] {
        assert_eq!(
            evaluate(
                &Node::SetFilter {
                    set: set.into(),
                    filter_function: identity.clone().into(),
                },
                &BASE,
            )
            .into_object(&BASE),
            Object::Composite(Composite::empty())
        );
    }
}
