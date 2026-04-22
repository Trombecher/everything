#![allow(non_snake_case)]

use super::*;

#[test]
fn BytesStructure__from_parts() {
    assert_eq!(BytesStructure::from_parts(&[], &[]), None);

    let bytes = BytesStructure::from_parts(&[0, 1, 2], &[]).unwrap();
    assert_eq!(bytes.data.as_ref(), &[0, 1, 2]);

    let bytes = BytesStructure::from_parts(&[], &[3, 4, 5]).unwrap();
    assert_eq!(bytes.data.as_ref(), &[3, 4, 5]);

    let bytes = BytesStructure::from_parts(&[0, 1, 2], &[3, 4, 5]).unwrap();
    assert_eq!(bytes.data.as_ref(), &[0, 1, 2, 3, 4, 5]);
}

#[test]
fn BytesStructure__parts() {
    let bytes = BytesStructure::from_parts(&[42, 67, 69], &[]).unwrap();

    assert_eq!(bytes.parts(), (Byte(42), [67, 69].as_slice()));
}

#[test]
fn BytesStructure__has() {
    assert!(!BytesStructure::new(&[10]).unwrap().has(
        &Object::Abstract(Abstract::BIT_0),
        &Object::Abstract(Abstract(435784))
    ));

    assert!(!BytesStructure::new(&[10]).unwrap().has(
        &Object::Abstract(Abstract::LIST_ITEM),
        &Object::Abstract(Abstract(435784))
    ));

    assert!(!BytesStructure::new(&[10]).unwrap().has(
        &Object::Abstract(Abstract::LIST_TAIL),
        &Object::Abstract(Abstract(435784))
    ));

    assert!(BytesStructure::new(&[42]).unwrap().has(
        &Object::Abstract(Abstract::LIST_ITEM),
        &Object::Structure(Structure::Byte(Byte(42)))
    ));

    assert!(BytesStructure::new(&[42, 69]).unwrap().has(
        &Object::Abstract(Abstract::LIST_TAIL),
        &Object::Structure(Structure::Bytes(BytesStructure::new(&[69]).unwrap()))
    ));

    assert!(BytesStructure::new(&[42]).unwrap().has(
        &Object::Abstract(Abstract::LIST_TAIL),
        &Object::Structure(Structure::Empty)
    ));
}

#[test]
fn BytesStructure__register_and_drop() {
    let count = GLOBAL_BINARY_DATA.len();

    {
        let bytes = BytesStructure::from_parts(&[0, 1, 2], &[]).unwrap();
        assert_eq!(bytes.ref_count(), 2);

        assert_eq!(GLOBAL_BINARY_DATA.len(), count + 1);
    }

    assert_eq!(GLOBAL_BINARY_DATA.len(), count);
}

/// Test to verify that the properties of [BytesStructure]s
/// are sorted.
#[test]
fn BytesStructureProperties__next_is_sorted() {
    let properties = BytesStructureProperties::TailAndItem(&[], Byte(0));

    assert!(properties.is_sorted());
}

#[test]
fn BytesStructure__next() {
    let mut properties = BytesStructureProperties::TailAndItem(&[69], Byte(42));

    assert_eq!(
        properties.next(),
        Some(Property {
            tag: Object::Abstract(Abstract::LIST_ITEM),
            value: Object::Structure(Structure::Byte(Byte(42))),
        })
    );
    assert_eq!(
        properties.next(),
        Some(Property {
            tag: Object::Abstract(Abstract::LIST_TAIL),
            value: Object::Structure(Structure::Bytes(BytesStructure::new(&[69]).unwrap()))
        })
    );
    assert_eq!(properties.next(), None);
    assert_eq!(properties.next(), None);
    assert_eq!(properties.next(), None);
}
