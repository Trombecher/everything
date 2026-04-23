use std::num::NonZeroU128;

use super::*;

#[test]
fn specialization() {
    assert_eq!(
        Structure::NaturalNumber(NonZeroU128::new(1).unwrap()),
        Structure::new(&mut [Property::new_successor_of(0)])
    )
}
