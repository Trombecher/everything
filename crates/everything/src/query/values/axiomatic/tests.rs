use everything_structures::{Abstract, Object, Property, Structure};

use crate::{
    base::BASE,
    ext::{AbstractExt, PropertyExt},
    query,
};

#[test]
fn structure_props() {
    let structure: Object = Structure::new(&mut [
        Property::new_contains(Abstract(4242).into()),
        Property::new_contains(Abstract(6969).into()),
        Property::new_successor_of(Abstract::ZERO.into()),
    ])
    .into();

    let mut values = query::values_axiomatically(&BASE, structure, Abstract::CONTAINS.into());

    assert_eq!(values.next(), Some(Object::Abstract(Abstract(4242))));
    assert_eq!(values.next(), Some(Object::Abstract(Abstract(6969))));
    assert_eq!(values.next(), None);
    assert_eq!(values.next(), None);
}
