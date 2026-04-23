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

    assert_eq!(res, [AXIOMATIC_AXIOMATIC_CONSTRAINT.clone()]);
}
