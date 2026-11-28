use crate::model::{Atom, AtomOrComposite};

use super::super::*;
use super::common::*;

#[test]
fn object_has_item_distinguishes_contained_and_absent_items() {
    let atom = Atom(42);
    let contained = integer(1);
    let not_contained = integer(2);

    let mut knowledge = Knowledge::empty();
    knowledge.add_statement(Statement {
        atom,
        object: contains(contained.clone()),
    });

    let atom = AtomOrComposite::Atom(atom);

    assert!(knowledge.object_has_item(&atom, &contained));
    assert!(!knowledge.object_has_item(&atom, &not_contained));
}
