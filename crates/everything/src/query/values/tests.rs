use everything_objects::{Abstract, Composite, Object, Property};

use crate::{
    base::BASE,
    ext::{AbstractExt, PropertyExt},
    query::QueryValues,
};

#[test]
fn composite_props() {
    let composite: Object = Composite::new(&mut [
        Property::new_contains(Abstract(4242).into()),
        Property::new_contains(Abstract(6969).into()),
        Property::new_successor_of(Abstract::ZERO.into()),
    ])
    .into();

    let mut values = QueryValues::new(&BASE, composite, Abstract::CONTAINS.into());

    assert_eq!(values.next(), Some(Object::Abstract(Abstract(4242))));
    assert_eq!(values.next(), Some(Object::Abstract(Abstract(6969))));
    assert_eq!(values.next(), None);
    assert_eq!(values.next(), None);
}
