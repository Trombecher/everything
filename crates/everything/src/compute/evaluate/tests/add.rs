use std::ops::RangeInclusive;

use num_bigint::BigInt;

use crate::{
    base::BASE,
    model::{Atom, Composite, Object},
};

use super::common::*;
use crate::compute::*;

#[test]
fn add() {
    const RANGE: RangeInclusive<i128> = -10..=20;

    for (a, b) in product_iterator(RANGE, RANGE) {
        assert_eq!(
            evaluate(
                &Node::Add {
                    first: Node::Literal(BigInt::from(a).into()).into(),
                    second: Node::Literal(BigInt::from(b).into()).into(),
                },
                &BASE,
            )
            .into_object(&BASE),
            BigInt::from(a + b).into()
        );
    }
}

#[test]
fn add_rejects_composites() {
    let union = Node::SetUnion {
        first: Node::Literal(Object::Composite(Composite::empty())).into(),
        second: Node::Literal(Object::Composite(Composite::empty())).into(),
    };

    assert_eq!(
        evaluate(
            &Node::Add {
                first: union.into(),
                second: Node::Literal(BigInt::from(1).into()).into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );

    assert_eq!(
        evaluate(
            &Node::Add {
                first: identity_function().into(),
                second: Node::Literal(BigInt::from(1).into()).into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}
