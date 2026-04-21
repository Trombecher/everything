use std::borrow::Cow;

use everything_structures::Object;

use crate::{
    base::{AXIOMATIC_AXIOMATIC_CONSTRAINT, BASE},
    ext::ObjectExt,
    query::values,
};

#[test]
fn basic() {
    let ax = Object::AXIOMATIC;

    let qr = values::values(&BASE, &ax, Object::AXIOMATIC, &mut Default::default());

    let res: Vec<_> = qr.iter().collect();

    assert_eq!(res, [Cow::Borrowed(&*AXIOMATIC_AXIOMATIC_CONSTRAINT)]);
}
