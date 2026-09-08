use everything_objects::{Abstract, Composite, Object, Property};

use crate::{
    base::{BASE, IS_INTEGER},
    ctx::EvaluationContext,
    ext::{AbstractExt, CompositeExt, ObjectExt},
};
use std::assert_matches;

#[test]
fn base_is_knowledge() {
    assert_matches!(BASE.is_knowledge(), Ok(()));
}

#[test]
fn integers() {
    let knowledge = &BASE;

    // Positive cases

    for i in -10..10_i128 {
        assert!(
            IS_INTEGER
                .call(
                    knowledge,
                    &[Object::new_integer(i).into()],
                    &mut EvaluationContext::default(),
                )
                .is_truthy(knowledge),
            "{i} is apparently not an integer"
        );
    }

    // Negative cases

    let not_integers = [
        Object::Abstract(Abstract(18_743_875_983_473)),
        Composite::Empty.into(),
        Composite::new_bool(true).into(),
        Composite::new(&mut [Property {
            tag: Abstract::SUCCESSOR_OF.into(),
            value: Composite::Empty.into(),
        }])
        .into(),
        Composite::new(&mut [
            Property {
                tag: Abstract::SUCCESSOR_OF.into(),
                value: Object::new_integer(0),
            },
            Property {
                tag: Abstract::SUCCESSOR_OF.into(),
                value: Object::new_integer(1),
            },
        ])
        .into(),
        Composite::new(&mut [Property {
            tag: Abstract::SUCCESSOR_OF.into(),
            value: Abstract(18_743_875_983_473).into(),
        }])
        .into(),
        Composite::new(&mut [Property {
            tag: Abstract::PREDECESSOR_OF.into(),
            value: Composite::Empty.into(),
        }])
        .into(),
        Composite::new(&mut [
            Property {
                tag: Abstract::PREDECESSOR_OF.into(),
                value: Object::new_integer(0),
            },
            Property {
                tag: Abstract::PREDECESSOR_OF.into(),
                value: Object::new_integer(-1),
            },
        ])
        .into(),
        Composite::new(&mut [
            Property {
                tag: Abstract::PREDECESSOR_OF.into(),
                value: Abstract::ZERO.into(),
            },
            Property {
                tag: Abstract::SUCCESSOR_OF.into(),
                value: Abstract::ZERO.into(),
            },
        ])
        .into(),
        Composite::new(&mut [
            Property {
                tag: Abstract::SUCCESSOR_OF.into(),
                value: Abstract::ZERO.into(),
            },
            Property {
                tag: Abstract::CONTAINS.into(),
                value: Composite::Empty.into(),
            },
        ])
        .into(),
    ];

    for not_integer in not_integers {
        assert!(
            !IS_INTEGER
                .call(
                    knowledge,
                    &[not_integer.into()],
                    &mut EvaluationContext::default()
                )
                .is_truthy(knowledge)
        );
    }
}
