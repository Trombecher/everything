#[allow(non_snake_case)]
mod BytesStructure {
    use super::super::*;

    #[test]
    fn from_parts() {
        assert_eq!(BytesStructure::from_parts(&[], &[]), None);

        let bytes = BytesStructure::from_parts(&[0, 1, 2], &[]).unwrap();
        assert_eq!(bytes.data.as_ref(), &[0, 1, 2]);

        let bytes = BytesStructure::from_parts(&[], &[3, 4, 5]).unwrap();
        assert_eq!(bytes.data.as_ref(), &[3, 4, 5]);

        let bytes = BytesStructure::from_parts(&[0, 1, 2], &[3, 4, 5]).unwrap();
        assert_eq!(bytes.data.as_ref(), &[0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn parts() {
        let bytes = BytesStructure::from_parts(&[42, 67, 69], &[]).unwrap();

        assert_eq!(bytes.parts(), (Byte(42), [67, 69].as_slice()));
    }

    #[test]
    fn has() {
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
    fn register_and_drop() {
        let bytes = BytesStructure::from_parts(&[0, 1, 2], &[]).unwrap();
        assert_eq!(bytes.ref_count(), 2);
    }

    #[test]
    fn debug_fmt() {
        let structure = BytesStructure::new(&[0x00, 0x10, 0x42, 0xA5, 0xFF]).unwrap();

        assert_eq!(format!("{:?}", structure), "X001042A5FF");
    }
}

#[allow(non_snake_case)]
mod BytesStructureProperties {
    use super::super::*;

    /// Test to verify that the properties of [BytesStructure]s
    /// are sorted.
    #[test]
    fn next_is_sorted() {
        let properties = BytesStructureProperties::TailAndItem(BytesStructure::new(&[0]).unwrap());

        assert!(properties.is_sorted());
    }

    #[test]
    fn next() {
        let mut properties =
            BytesStructureProperties::TailAndItem(BytesStructure::new(&[42, 69]).unwrap());

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
}

#[allow(non_snake_case)]
mod BytesStructureValues {
    use std::iter;

    use super::super::*;

    #[test]
    fn next() {
        let mut values = BytesStructureValues::None;
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);

        let mut values = BytesStructureValues::ListItem(Byte(42));
        assert_eq!(
            values.next(),
            Some(Object::Structure(Structure::Byte(Byte(42))))
        );
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);

        let mut values = BytesStructureValues::Tail(BytesStructure::new(&[42, 67, 69]).unwrap());
        assert_eq!(
            values.next(),
            Some(Object::Structure(Structure::Bytes(
                BytesStructure::new(&[67, 69]).unwrap()
            )))
        );
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
    }

    #[test]
    #[should_panic]
    fn from_iter_panic() {
        BytesStructure::from_iter(iter::empty(), NonZeroUsize::new(10).unwrap());
    }

    #[test]
    fn from_iter() {
        const BYTES: [u8; 7] = [0, 1, 2, 3, 255, 42, 67];

        let a =
            BytesStructure::from_iter(BYTES.into_iter(), NonZeroUsize::new(BYTES.len()).unwrap());

        let b = BytesStructure::new(&BYTES).unwrap();

        assert_eq!(a, b)
    }
}
