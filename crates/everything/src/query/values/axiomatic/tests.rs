use everything_structures::{Abstract, Object, Property, Structure};

use crate::{base::BASE, ext::ObjectExt, query};

#[test]
fn structure_props() {
    let structure: Object = Structure::new(&mut [
        Property {
            tag: Object::CONTAINS,
            value: Object::Abstract(Abstract(4242)),
        },
        Property {
            tag: Object::CONTAINS,
            value: Object::Abstract(Abstract(6969)),
        },
        Property {
            tag: Object::Abstract(Abstract::SUCCESSOR_OF),
            value: Object::Abstract(Abstract::ZERO),
        },
    ])
    .into();

    let mut values = query::values_axiomatically(&BASE, &structure, Object::CONTAINS);

    assert_eq!(values.next(), Some(Object::Abstract(Abstract(4242))));
    assert_eq!(values.next(), Some(Object::Abstract(Abstract(6969))));
    assert_eq!(values.next(), None);
    assert_eq!(values.next(), None);
}
