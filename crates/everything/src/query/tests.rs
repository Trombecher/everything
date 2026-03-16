use everything_structures::Object;

use crate::{
    base::{AXIOMATIC_AXIOMATIC_CONSTRAINT, BASE},
    ext::ObjectExt,
    query::query_values,
};

#[test]
fn basic() {
    let ax = Object::AXIOMATIC;

    let qr = query_values(&BASE, &ax, Object::AXIOMATIC, &mut Default::default());

    let res: Vec<&Object> = qr.iter().collect();

    assert_eq!(res, [&*AXIOMATIC_AXIOMATIC_CONSTRAINT]);
}
