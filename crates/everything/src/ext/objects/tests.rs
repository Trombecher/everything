use everything_structures::{Object, Property, Structure};

use crate::ext::ObjectExt;

#[test]
fn is_only_natural_number() {
    // Abstracts
    assert!(!Object::CONTAINS.is_only_natural_number());
    assert!(Object::ZERO.is_only_natural_number());

    let one: Object = Structure::new(&mut [Property {
        tag: Object::SUCCESSOR_OF,
        value: Object::ZERO,
    }])
    .into();

    assert!(one.is_only_natural_number());

    let one_with_stuff: Object = Structure::new(&mut [
        Property {
            tag: Object::SUCCESSOR_OF,
            value: Object::ZERO,
        },
        Property {
            tag: Object::CONTAINS,
            value: Object::ZERO,
        },
    ])
    .into();

    assert!(!one_with_stuff.is_only_natural_number())
}

#[test]
fn natural_number() {
    assert_eq!(Object::natural_number(0), Object::ZERO);

    assert_eq!(
        Object::natural_number(2),
        Structure::new(&mut [Property {
            tag: Object::SUCCESSOR_OF,
            value: Structure::new(&mut [Property {
                tag: Object::SUCCESSOR_OF,
                value: Object::ZERO
            }])
            .into()
        }])
        .into()
    )
}

// TODO: more tests
