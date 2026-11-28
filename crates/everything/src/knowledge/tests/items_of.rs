use crate::model::{Atom, AtomOrComposite};

use super::super::*;
use super::common::*;

#[test]
fn items_of_an_atom_yields_its_contains_values() {
    let atom = Atom(42);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: contains(integer(1)),
    });
    knowledge.add_statement(Statement {
        atom,
        object: integer(2),
    });

    assert_eq!(
        knowledge
            .items_of(&AtomOrComposite::Atom(atom))
            .collect::<Vec<_>>(),
        vec![integer(1)]
    );
}
