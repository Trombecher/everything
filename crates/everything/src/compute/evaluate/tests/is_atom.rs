use crate::{
    base::BASE,
    model::{Atom, Object},
};

use super::super::*;

#[test]
fn undefined_is_an_atom() {
    assert_eq!(
        evaluate(
            &Node::IsAtom(Node::Literal(Object::Atom(Atom::UNDEFINED)).into()),
            &BASE
        )
        .into_object(&BASE),
        Atom::TRUE.into()
    );
}
