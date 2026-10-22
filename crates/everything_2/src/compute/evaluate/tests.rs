use std::ops::RangeInclusive;

use crate::base::BASE;

use super::*;

#[test]
fn less() {
    const RANGE: RangeInclusive<i128> = -10..=20;

    for (a, b) in RANGE.flat_map(|a| RANGE.clone().map(move |b| (a, b))) {
        assert_eq!(
            evaluate(
                &Object::new_node(Node::Less {
                    first: a.into(),
                    second: b.into(),
                }),
                &BASE,
                &mut EvaluationContext::default(),
            )
            .into_object(),
            (a < b).into()
        );
    }
}
