use crate::model::{Atom, AtomOrComposite};

use super::super::*;
use super::common::*;

#[test]
fn size_of_object_counts_the_properties_of_an_atom() {
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: integer(1),
    });
    knowledge.add_statement(Statement {
        atom,
        object: integer(2),
    });

    assert_eq!(knowledge.size_of_object(&AtomOrComposite::Atom(atom)), 2);
}
