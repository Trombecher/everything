use num_bigint::BigInt;

use crate::{
    base::BASE,
    model::{Atom, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn filter() {
    let node = Node::Filter {
        set: Node::Literal(integer_set().into()).into(),
        filter_function: Node::function(Node::Equal {
            first: Node::Parameter { depth: 0 }.into(),
            second: Node::Literal(Object::new_integer(BigInt::from(2))).into(),
        })
        .into(),
    };

    let values = evaluate(&node, &BASE).iter(&BASE).collect::<Vec<_>>();

    assert_eq!(values, vec![Object::new_integer(BigInt::from(2))]);
}

#[test]
fn filter_requires_sets() {
    let atom = Node::Literal(Object::Atom(Atom(42)));
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());
    let identity = identity_function();

    assert_eq!(
        evaluate(
            &Node::Filter {
                set: atom.into(),
                filter_function: identity.clone().into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );
    assert_eq!(
        evaluate(
            &Node::Filter {
                set: construction.into(),
                filter_function: identity.into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}
