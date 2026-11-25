use num_bigint::BigInt;

use crate::{
    base::BASE,
    model::{Atom, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn map() {
    let node = Node::Map {
        composite: Node::Literal(integer_composite().into()).into(),
        mapper_function: identity_function().into(),
    };

    let values = evaluate(&node, &BASE).iter(&BASE).collect::<Vec<_>>();

    assert_eq!(
        values,
        [
            Object::new_integer(BigInt::from(1)),
            Object::new_integer(BigInt::from(2)),
            Object::new_integer(BigInt::from(3)),
        ]
    );
}

#[test]
fn map_deduplicates() {
    let node = Node::Map {
        composite: Node::Literal(integer_composite().into()).into(),
        // Maps every element to `1`.
        mapper_function: constant_predicate(integer(1)).into(),
    };

    let result = evaluate(&node, &BASE);

    assert_eq!(result.iter(&BASE).collect::<Vec<_>>(), vec![integer(1)]);
    assert_eq!(result.size(&BASE), 1);
}

#[test]
fn map_requires_composites() {
    let atom = Node::Literal(Object::Atom(Atom(42)));
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());
    let identity = identity_function();

    assert_eq!(
        evaluate(
            &Node::Map {
                composite: atom.into(),
                mapper_function: identity.clone().into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );
    assert_eq!(
        evaluate(
            &Node::Map {
                composite: construction.into(),
                mapper_function: identity.clone().into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );

    // A function object is a composite, so mapping over it yields its property.
    let values = evaluate(
        &Node::Map {
            composite: identity_function().into(),
            mapper_function: identity_function().into(),
        },
        &BASE,
    )
    .iter(&BASE)
    .collect::<Vec<_>>();

    assert_eq!(
        values,
        vec![
            Construction::new(
                Atom::NODE_FUNCTION.into(),
                Node::Parameter { depth: 0 }.into_object()
            )
            .into()
        ]
    );
}
