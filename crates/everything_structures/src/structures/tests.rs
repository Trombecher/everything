use std::num::NonZeroU128;

use crate::{Property, Structure};

#[test]
fn specialization() {
    assert_eq!(
        Structure::NaturalNumber(NonZeroU128::new(1).unwrap()),
        Structure::new(&mut [Property::successor_of(0)])
    )
}
