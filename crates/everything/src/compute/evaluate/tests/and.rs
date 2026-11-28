use crate::{
    base::BASE,
    model::{Atom, Composite, Object},
};

use super::common::*;
use crate::compute::*;

#[test]
fn and() {
    let cases = [
        Object::Composite(Composite::empty()),
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
            )
            .into_object(&BASE),
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
