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
                &Object::new_node(Node::Less {
                    first: BigInt::from(a).into(),
                    second: BigInt::from(b).into(),
                }),
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
                &Object::new_node(Node::And {
                    first: Object::new_node(Node::Literal(first.clone())),
                    second: Object::new_node(Node::Literal(second.clone()))
                }),
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
