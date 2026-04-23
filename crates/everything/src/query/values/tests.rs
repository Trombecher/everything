use core::slice;

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

    let res: Vec<_> = qr.values().collect();

    assert_eq!(res, slice::from_ref(&*AXIOMATIC_AXIOMATIC_CONSTRAINT));
}
