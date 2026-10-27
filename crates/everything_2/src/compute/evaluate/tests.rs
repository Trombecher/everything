use std::ops::RangeInclusive;

use num_bigint::BigInt;

use crate::{base::BASE, model::Set};

use super::*;

fn product_iterator<A: Clone, B>(
    a: impl Iterator<Item = A>,
    b: impl Iterator<Item = B> + Clone,
) -> impl Iterator<Item = (A, B)> {
    a.flat_map(move |a_item| b.clone().map(move |b_item| (a_item.clone(), b_item)))
}

#[test]
fn less() {
    const RANGE: RangeInclusive<i128> = -10..=20;

    for (a, b) in product_iterator(RANGE, RANGE) {
        assert_eq!(
            evaluate(
                &Node::Less {
                    first: Node::Literal(BigInt::from(a).into()).into(),
                    second: Node::Literal(BigInt::from(b).into()).into(),
                },
                &BASE,
                &mut EvaluationContext::default(),
            )
            .into_object(),
            (a < b).into()
        );
    }
}

#[test]
fn and() {
    let cases = [
        Object::Set(Set::empty()),
        Atom::TRUE.into(),
        Atom::FALSE.into(),
    ];

    for (first, second) in product_iterator(cases.iter().cloned(), cases.iter().cloned()) {
        assert_eq!(
            evaluate(
                &Node::And {
                    first: Node::Literal(first.clone()).into(),
                    second: Node::Literal(second.clone()).into()
                },
                &BASE,
                &mut EvaluationContext::default()
            )
            .into_object(),
            match first {
                Object::Atom(Atom::TRUE) => {
                    MaybeUndefined::from(second.truthy()).into()
                }
                Object::Atom(Atom::FALSE) => Atom::FALSE.into(),
                _ => Atom::UNDEFINED.into(),
            },
            "case {first:?} {second:?}"
        );
    }
}

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
                &mut EvaluationContext::default(),
            )
            .into_object(),
            BigInt::from(a + b).into()
        );
    }
}

#[test]
fn unwrap() {}
