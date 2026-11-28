use num_bigint::BigInt;

use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::common::*;
use crate::compute::*;

#[test]
fn set_map() {
    let node = Node::SetMap {
        set: Node::Literal(integer_set().into()).into(),
        mapper_function: identity_function().into(),
    };

    let result = evaluate(&node, &BASE);
    let values = set(&result).iter(&BASE).collect::<Vec<_>>();

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
fn set_map_deduplicates() {
    let node = Node::SetMap {
        set: Node::Literal(integer_set().into()).into(),
        // Maps every item to `1`.
        mapper_function: constant_predicate(integer(1)).into(),
    };

    let result = evaluate(&node, &BASE);

    assert_eq!(
        set(&result).iter(&BASE).collect::<Vec<_>>(),
        vec![integer(1)]
    );
    assert_eq!(set(&result).item_count(&BASE), 1);
}

#[test]
fn set_map_requires_a_set() {
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());
    let identity = identity_function();

    assert_eq!(
        evaluate(
            &Node::SetMap {
                set: construction.into(),
                mapper_function: identity.clone().into(),
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
                &Node::SetMap {
                    set: set.into(),
                    mapper_function: identity.clone().into(),
                },
                &BASE,
            )
            .into_object(&BASE),
            Object::Composite(Composite::empty())
        );
    }
}
