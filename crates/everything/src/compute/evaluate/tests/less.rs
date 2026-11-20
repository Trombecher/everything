use std::ops::RangeInclusive;

use num_bigint::BigInt;

use crate::base::BASE;

use super::super::*;
use super::common::*;

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
            )
            .into_object(&BASE),
            (a < b).into()
        );
    }
}
