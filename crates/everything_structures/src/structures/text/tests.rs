#![allow(non_snake_case)]

use super::*;

#[test]
fn TextStructureProperties__next_is_sorted() {
    assert!(TextStructureProperties::TailAndItem("ello, world!", 'h').is_sorted());
}

#[test]
fn TextStructureProperties__next() {
    let mut properties = TextStructureProperties::TailAndItem("tail", 'a');
    assert_eq!(
        properties.next(),
        Some(Property {
            tag: Object::Abstract(Abstract::LIST_ITEM),
            value: Object::Structure(Structure::Character('a'))
        })
    );
    assert_eq!(
        properties.next(),
        Some(Property {
            tag: Object::Abstract(Abstract::LIST_TAIL),
            value: Object::Structure(Structure::Text(TextStructure::new("tail").unwrap()))
        })
    );
    assert_eq!(properties.next(), None);
    assert_eq!(properties.next(), None);
    assert_eq!(properties.next(), None);
}
