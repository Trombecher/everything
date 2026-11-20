use num_bigint::BigInt;

use crate::{
    base::BASE,
    model::{Atom, Construction, Object, Set},
};

use super::super::*;
use super::common::*;

#[test]
fn count_requires_sets() {
    let atom = Atom(42);
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));
    let set = Object::Set(Set::from([Object::new_integer(BigInt::from(1))]));

    let count = |object: Object| {
        evaluate(&Node::Count(Node::Literal(object).into()), &BASE).into_object(&BASE)
    };

    assert_eq!(count(Object::Atom(atom)), Atom::UNDEFINED.into());
    assert_eq!(count(construction), Atom::UNDEFINED.into());
    assert_eq!(count(Atom::UNDEFINED.into()), Atom::UNDEFINED.into());
    assert_eq!(count(set), Object::new_integer(BigInt::from(1)));

    // A function value is a set: its node representation has one property.
    assert_eq!(
        evaluate(&Node::Count(identity_function().into()), &BASE).into_object(&BASE),
        Object::new_integer(BigInt::from(1))
    );
}
