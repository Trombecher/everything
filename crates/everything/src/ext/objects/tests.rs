use everything_structures::{Object, Property, Structure};

use crate::{
    Knowledge,
    base::BASE,
    ext::{NodeType, ObjectExt, StructureExt},
};

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

#[test]
fn node_type() {
    let knowledge = &BASE;

    // None
    assert_eq!(
        Object::Structure(Structure::EMPTY).node_type(knowledge),
        None
    );

    // Single
    assert_eq!(
        Object::Structure(Structure::new_node_function(Object::ZERO)).node_type(knowledge),
        Some(NodeType::FunctionBody)
    );

    /*
    // Multiple
    assert_eq!(
        Object::Structure(Structure::new_node_and([Object::ZERO, Object::KNOWLEDGE]))
            .node_type(knowledge),
        Some(NodeType::And)
    );

    // Mix
    assert_eq!(
        Object::Structure(
            Structure::new_node_function(Object::ZERO)
                .union(&Structure::new_node_count(Object::ZERO)),
        )
        .node_type(knowledge),
        None
    );
     */
}
