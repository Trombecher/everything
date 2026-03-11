use everything_structures::{Object, Property, Structure};

use crate::objects;

#[test]
fn is_only_natural_number() {
    // Abstracts
    assert!(!objects::is_only_natural_number(&objects::CONTAINS));
    assert!(objects::is_only_natural_number(&objects::ZERO));

    let one: Object = Structure::new(&mut [Property {
        tag: objects::SUCESSOR_OF,
        value: objects::ZERO,
    }])
    .into();

    assert!(objects::is_only_natural_number(&one));

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

    assert!(!objects::is_only_natural_number(&one_with_stuff))
}

#[test]
fn natural_number() {
    assert_eq!(objects::natural_number(0), objects::ZERO);

    assert_eq!(
        objects::natural_number(2),
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
