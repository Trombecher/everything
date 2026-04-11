use std::num::NonZeroU128;

use crate::{AnyStructure, Object, Property, Structure};

#[test]
fn equality() {
    assert_eq!(
        Structure::NaturalNumber(NonZeroU128::new(1).unwrap()),
        Structure::Any(AnyStructure::new(&mut [Property {
            tag: Object::SUCCESSOR_OF,
            value: Object::ZERO
        }]))
    )
}
