use everything_structures::{Abstract, Object};

use crate::{
    base::{AXIOMATIC_AXIOMATIC_CONSTRAINT, BASE},
    ext::AbstractExt,
    query::values,
};

#[test]
fn basic() {
    let qr = values::values(
        &BASE,
        &Object::Abstract(Abstract::AXIOMATIC),
        Abstract::AXIOMATIC.into(),
        &mut Default::default(),
    );

    let mut values = qr.values();
    assert_eq!(values.next(), Some(AXIOMATIC_AXIOMATIC_CONSTRAINT.clone()));
    assert_eq!(values.next(), None);
    assert_eq!(values.next(), None);
    assert_eq!(values.next(), None);
}
