#[allow(non_snake_case)]
mod BytesComposite {
    use super::super::*;

    #[test]
    fn from_parts() {
        assert_eq!(BytesComposite::from_parts(&[], &[]), None);

        let bytes = BytesComposite::from_parts(&[0, 1, 2], &[]).unwrap();
        assert_eq!(bytes.data.as_ref(), &[0, 1, 2]);

        let bytes = BytesComposite::from_parts(&[], &[3, 4, 5]).unwrap();
        assert_eq!(bytes.data.as_ref(), &[3, 4, 5]);

        let bytes = BytesComposite::from_parts(&[0, 1, 2], &[3, 4, 5]).unwrap();
        assert_eq!(bytes.data.as_ref(), &[0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn parts() {
        let bytes = BytesComposite::from_parts(&[42, 67, 69], &[]).unwrap();

        assert_eq!(bytes.parts(), (Byte(42), [67, 69].as_slice()));
    }

    #[test]
    fn has() {
        assert!(!BytesComposite::new(&[10]).unwrap().has(
            &Object::Abstract(Abstract::BIT_0),
            &Object::Abstract(Abstract(435784))
        ));

        assert!(!BytesComposite::new(&[10]).unwrap().has(
            &Object::Abstract(Abstract::LIST_ITEM),
            &Object::Abstract(Abstract(435784))
        ));

        assert!(!BytesComposite::new(&[10]).unwrap().has(
            &Object::Abstract(Abstract::LIST_TAIL),
            &Object::Abstract(Abstract(435784))
        ));

        assert!(BytesComposite::new(&[42]).unwrap().has(
            &Object::Abstract(Abstract::LIST_ITEM),
            &Object::Composite(Composite::Byte(Byte(42)))
        ));

        assert!(BytesComposite::new(&[42, 69]).unwrap().has(
            &Object::Abstract(Abstract::LIST_TAIL),
            &Object::Composite(Composite::Bytes(BytesComposite::new(&[69]).unwrap()))
        ));

        assert!(BytesComposite::new(&[42]).unwrap().has(
            &Object::Abstract(Abstract::LIST_TAIL),
            &Object::Composite(Composite::Empty)
        ));
    }

    #[test]
    fn register_and_drop() {
        let bytes = BytesComposite::from_parts(&[0, 1, 2], &[]).unwrap();
        assert_eq!(bytes.ref_count(), 2);
    }

    #[test]
    fn debug_fmt() {
        let Composite = BytesComposite::new(&[0x00, 0x10, 0x42, 0xA5, 0xFF]).unwrap();

        assert_eq!(format!("{:?}", Composite), "X001042A5FF");
    }
}

#[allow(non_snake_case)]
mod BytesCompositeProperties {
    use super::super::*;

    /// Test to verify that the properties of [BytesComposite]s
    /// are sorted.
    #[test]
    fn next_is_sorted() {
        let properties = BytesCompositeProperties::TailAndItem(BytesComposite::new(&[0]).unwrap());

        assert!(properties.is_sorted());
    }

    #[test]
    fn next() {
        let mut properties =
            BytesCompositeProperties::TailAndItem(BytesComposite::new(&[42, 69]).unwrap());

        assert_eq!(
            properties.next(),
            Some(Property {
                tag: Object::Abstract(Abstract::LIST_ITEM),
                value: Object::Composite(Composite::Byte(Byte(42))),
            })
        );
        assert_eq!(
            properties.next(),
            Some(Property {
                tag: Object::Abstract(Abstract::LIST_TAIL),
                value: Object::Composite(Composite::Bytes(BytesComposite::new(&[69]).unwrap()))
            })
        );
        assert_eq!(properties.next(), None);
        assert_eq!(properties.next(), None);
        assert_eq!(properties.next(), None);
    }
}

#[allow(non_snake_case)]
mod BytesCompositeValues {
    use std::iter;

    use super::super::*;

    #[test]
    fn next() {
        let mut values = BytesCompositeValues::None;
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);

        let mut values = BytesCompositeValues::ListItem(Byte(42));
        assert_eq!(
            values.next(),
            Some(Object::Composite(Composite::Byte(Byte(42))))
        );
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);

        let mut values = BytesCompositeValues::Tail(BytesComposite::new(&[42, 67, 69]).unwrap());
        assert_eq!(
            values.next(),
            Some(Object::Composite(Composite::Bytes(
                BytesComposite::new(&[67, 69]).unwrap()
            )))
        );
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
    }

    #[test]
    #[should_panic]
    fn from_iter_panic() {
        BytesComposite::from_iter(iter::empty(), NonZeroUsize::new(10).unwrap());
    }

    #[test]
    fn from_iter() {
        const BYTES: [u8; 7] = [0, 1, 2, 3, 255, 42, 67];

        let a =
            BytesComposite::from_iter(BYTES.into_iter(), NonZeroUsize::new(BYTES.len()).unwrap());

        let b = BytesComposite::new(&BYTES).unwrap();

        assert_eq!(a, b)
    }
}
