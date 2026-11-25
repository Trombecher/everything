use crate::model::{Atom, Object};

use super::*;

#[test]
fn debug_fmt_uses_the_documented_construction_syntax() {
    let construction = Construction::new(Atom(10).into(), Atom(20).into());

    assert_eq!(format!("{construction:?}"), "@10 ·· @20");
}

#[test]
fn debug_fmt_nests_constructions_to_the_right() {
    let construction = Construction::new(
        Atom(10).into(),
        Construction::new(Atom(20).into(), Atom(30).into()).into(),
    );

    assert_eq!(format!("{construction:?}"), "@10 ·· @20 ·· @30");
}

#[test]
fn debug_fmt_renders_composite_members_with_composite_syntax() {
    let construction = Construction::new(
        Atom(10).into(),
        Object::Composite(crate::model::Composite::from([
            Atom(20).into(),
            Atom(30).into(),
        ])),
    );

    assert_eq!(format!("{construction:?}"), "@10 ·· ⟨@20, @30⟩");
}
