use crate::{
    base::BASE,
    model::{Atom, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn function_values_are_sets() {
    assert_eq!(
        evaluate(&Node::IsSet(identity_function().into()), &BASE).into_object(&BASE),
        Atom::TRUE.into()
    );
    assert_eq!(
        evaluate(
            &Node::IsSet(Node::Literal(Object::Atom(Atom(42))).into()),
            &BASE
        )
        .into_object(&BASE),
        Atom::FALSE.into()
    );
    assert_eq!(
        evaluate(
            &Node::IsSet(
                Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()).into()
            ),
            &BASE
        )
        .into_object(&BASE),
        Atom::FALSE.into()
    );
}
