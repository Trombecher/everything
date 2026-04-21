use std::num::NonZeroU128;

use crate::{Object, Property, Structure};

#[test]
fn equality() {
    assert_eq!(
        Structure::NaturalNumber(NonZeroU128::new(1).unwrap()),
        Structure::new(&mut [Property {
            tag: Object::SUCCESSOR_OF,
            value: Object::ZERO
        }])
    )
}
