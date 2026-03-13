use everything_structures::{Object, Property, Structure};

use crate::objects::{self, ObjectExt};

#[test]
fn is_only_natural_number() {
    // Abstracts
    assert!(!objects::CONTAINS.is_only_natural_number());
    assert!(objects::ZERO.is_only_natural_number());

    let one: Object = Structure::new(&mut [Property {
        tag: objects::SUCESSOR_OF,
        value: objects::ZERO,
    }])
    .into();

    assert!(one.is_only_natural_number());

    let one_with_stuff: Object = Structure::new(&mut [
        Property {
            tag: objects::SUCESSOR_OF,
            value: objects::ZERO,
        },
        Property {
            tag: objects::CONTAINS,
            value: objects::ZERO,
        },
    ])
    .into();

    assert!(!one_with_stuff.is_only_natural_number())
}

#[test]
fn natural_number() {
    assert_eq!(Object::natural_number(0), objects::ZERO);

    assert_eq!(
        Object::natural_number(2),
        Structure::new(&mut [Property {
            tag: objects::SUCESSOR_OF,
            value: Structure::new(&mut [Property {
                tag: objects::SUCESSOR_OF,
                value: objects::ZERO
            }])
            .into()
        }])
        .into()
    )
}

// TODO: more tests
