use crate::model::{Atom, AtomOrComposite};

use super::super::*;
use super::common::*;

#[test]
fn item_count_of_an_atom_counts_its_contains_values() {
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: contains(integer(1)),
    });
    knowledge.add_statement(Statement {
        atom,
        object: contains(integer(2)),
    });

    assert_eq!(knowledge.item_count_of(&AtomOrComposite::Atom(atom)), 2);
}
