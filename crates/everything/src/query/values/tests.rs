use everything_structures::{Abstract, Object};

use crate::{
    base::{AXIOMATIC_AXIOMATIC_CONSTRAINT, BASE},
    ext::AbstractExt,
    query::{self, QueryValues},
};

#[test]
fn basic() {
    let values = query::values(
        &BASE,
        Object::Abstract(Abstract::AXIOMATIC),
        Abstract::AXIOMATIC.into(),
    );

    let mut values = match values {
        QueryValues::Axiomatically(values) => values,
        QueryValues::Call { .. } => unreachable!(),
    };

    assert_eq!(values.next(), Some(AXIOMATIC_AXIOMATIC_CONSTRAINT.clone()));
    assert_eq!(values.next(), None);
    assert_eq!(values.next(), None);
    assert_eq!(values.next(), None);
}
