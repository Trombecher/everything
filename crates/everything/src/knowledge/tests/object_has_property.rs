use crate::model::{Atom, AtomOrComposite};

use super::super::*;
use super::common::*;

#[test]
fn object_has_property_distinguishes_known_and_unknown_properties() {
    let property = integer(1);
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: property.clone(),
    });

    assert!(knowledge.object_has_property(&AtomOrComposite::Atom(atom), &property));
    assert!(!knowledge.object_has_property(&AtomOrComposite::Atom(Atom(43)), &property));
}
