use crate::model::{Atom, Composite, Construction, Object};

#[test]
fn known_atoms_have_names() {
    assert_eq!(Atom::ZERO.known_name(), Some("zero"));
    assert_eq!(Atom::NODE_FUNCTION.known_name(), Some("node_function"));
    assert_eq!(
        Atom::NODE_CONSTRUCTION_VALUES_PROPERTY_CONSTRUCTOR.known_name(),
        Some("node_construction_values_property_constructor")
    );
}

#[test]
fn unknown_atoms_have_no_name() {
    assert_eq!(Atom(0).known_name(), None);
    assert_eq!(Atom(42).known_name(), None);
    assert_eq!(Atom(u128::MAX).known_name(), None);
}

#[test]
fn debug_prints_known_names_and_falls_back_to_ids() {
    assert_eq!(format!("{:?}", Atom::ZERO), "zero");
    assert_eq!(
        format!("{:?}", Atom::NODE_CONSTRUCTION_VALUES_PROPERTY_CONSTRUCTOR),
        "node_construction_values_property_constructor"
    );
    assert_eq!(format!("{:?}", Atom(42)), "@42");
}

#[test]
fn debug_names_propagate_through_objects() {
    let construction = Construction::new(Atom::CONTAINS.into(), Atom(10).into());

    assert_eq!(format!("{construction:?}"), "contains ·· @10");

    let composite = Composite::from([Object::Atom(Atom(20)), Object::Construction(construction)]);

    assert_eq!(format!("{composite:?}"), "⟨@20, contains ·· @10⟩");
}
