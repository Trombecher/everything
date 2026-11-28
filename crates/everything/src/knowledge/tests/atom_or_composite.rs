use crate::model::{Atom, AtomOrComposite};

use super::common::*;

#[test]
fn construction_is_not_an_atom_or_composite() {
    assert!(AtomOrComposite::try_from(construction_value(Atom(1), 1)).is_err());
}
