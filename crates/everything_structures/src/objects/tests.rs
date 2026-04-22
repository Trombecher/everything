use std::num::NonZeroU128;

use crate::{Abstract, Object, Structure};

#[test]
fn new_natural_number() {
    assert_eq!(
        Object::new_natural_number(0),
        Object::Abstract(Abstract::ZERO)
    );

    for i in (1..200_u128).map(|x| x * 7) {
        assert_eq!(
            Object::new_natural_number(i),
            Object::Structure(Structure::NaturalNumber(NonZeroU128::new(i).unwrap()))
        );
    }
}
