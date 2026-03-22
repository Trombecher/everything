use everything_structures::{Object, Property, Structure};

use crate::{base::BASE, ext::ObjectExt, query::values::values_axiomatically};

#[test]
fn structure_props() {
    let structure: Object = Structure::new(&mut [
        Property {
            tag: Object::CONTAINS,
            value: Object::Abstract(4242),
        },
        Property {
            tag: Object::CONTAINS,
            value: Object::Abstract(6969),
        },
        Property {
            tag: Object::SUCCESSOR_OF,
            value: Object::ZERO,
        },
    ])
    .into();

    let mut values = values_axiomatically(&BASE, &structure, Object::CONTAINS);

    assert_eq!(values.next(), Some(&Object::Abstract(4242)));
    assert_eq!(values.next(), Some(&Object::Abstract(6969)));
    assert_eq!(values.next(), None);
    assert_eq!(values.next(), None);
}
