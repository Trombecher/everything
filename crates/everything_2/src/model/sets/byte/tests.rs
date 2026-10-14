#[allow(non_snake_case)]
mod Byte {
    use super::super::*;

    #[test]
    fn bit() {
        let byte = ByteSet(0b1011_0111);

        assert_eq!(byte.bit(BitSlot::Slot0), Bit::One);
        assert_eq!(byte.bit(BitSlot::Slot1), Bit::One);
        assert_eq!(byte.bit(BitSlot::Slot2), Bit::One);
        assert_eq!(byte.bit(BitSlot::Slot3), Bit::Zero);
        assert_eq!(byte.bit(BitSlot::Slot4), Bit::One);
        assert_eq!(byte.bit(BitSlot::Slot5), Bit::One);
        assert_eq!(byte.bit(BitSlot::Slot6), Bit::Zero);
        assert_eq!(byte.bit(BitSlot::Slot7), Bit::One);
    }

    /*
    #[test]
    fn has() {
        let byte = Byte(0b0011_0101);

        assert!(!byte.has(&Object::Atom(Atom(34598)), &Object::Atom(Atom(306_985_430))));

        assert!(!byte.has(&Object::Atom(Atom::BIT_SLOT_0), &Object::Atom(Atom(34598))));

        assert!(!byte.has(
            &Object::Atom(Atom::BIT_SLOT_0),
            &Object::Atom(Atom::BIT_OFF)
        ));

        assert!(byte.has(&Object::Atom(Atom::BIT_SLOT_0), &Object::Atom(Atom::BIT_ON)));

        assert!(byte.has(
            &Object::Atom(Atom::BIT_SLOT_1),
            &Object::Atom(Atom::BIT_OFF)
        ));

        assert!(byte.has(&Object::Atom(Atom::BIT_SLOT_2), &Object::Atom(Atom::BIT_ON)));

        assert!(byte.has(
            &Object::Atom(Atom::BIT_SLOT_3),
            &Object::Atom(Atom::BIT_OFF)
        ));

        assert!(byte.has(&Object::Atom(Atom::BIT_SLOT_4), &Object::Atom(Atom::BIT_ON)));

        assert!(byte.has(&Object::Atom(Atom::BIT_SLOT_5), &Object::Atom(Atom::BIT_ON)));

        assert!(byte.has(
            &Object::Atom(Atom::BIT_SLOT_6),
            &Object::Atom(Atom::BIT_OFF)
        ));

        assert!(byte.has(
            &Object::Atom(Atom::BIT_SLOT_7),
            &Object::Atom(Atom::BIT_OFF)
        ));
    }
     */

    #[test]
    fn debug_fmt() {
        assert_eq!(format!("{:?}", ByteSet(0)), "x00");
        assert_eq!(format!("{:?}", ByteSet(0x10)), "x10");
        assert_eq!(format!("{:?}", ByteSet(0xA4)), "xA4");
        assert_eq!(format!("{:?}", ByteSet(0xFF)), "xFF");
    }
}

#[allow(non_snake_case)]
mod ByteValues {
    use super::super::*;

    #[test]
    fn next() {
        let mut values = ByteSetValues {
            byte: ByteSet(0b1010_0101),
            next_slot: Some(BitSlot::Slot0),
        };

        assert_eq!(
            values.next(),
            Some(
                Construction {
                    constructor: Box::new(Object::Atom(Atom::BIT_SLOT_0)),
                    value: Box::new(Object::Atom(Atom::BIT_ON))
                }
                .into()
            )
        );

        assert_eq!(
            values.next(),
            Some(
                Construction {
                    constructor: Box::new(Object::Atom(Atom::BIT_SLOT_1)),
                    value: Box::new(Object::Atom(Atom::BIT_OFF))
                }
                .into()
            )
        );

        assert_eq!(
            values.next(),
            Some(
                Construction {
                    constructor: Box::new(Object::Atom(Atom::BIT_SLOT_2)),
                    value: Box::new(Object::Atom(Atom::BIT_ON))
                }
                .into()
            )
        );

        assert_eq!(
            values.next(),
            Some(
                Construction {
                    constructor: Box::new(Object::Atom(Atom::BIT_SLOT_3)),
                    value: Box::new(Object::Atom(Atom::BIT_OFF))
                }
                .into()
            )
        );

        assert_eq!(
            values.next(),
            Some(
                Construction {
                    constructor: Box::new(Object::Atom(Atom::BIT_SLOT_4)),
                    value: Box::new(Object::Atom(Atom::BIT_OFF))
                }
                .into()
            )
        );

        assert_eq!(
            values.next(),
            Some(
                Construction {
                    constructor: Box::new(Object::Atom(Atom::BIT_SLOT_5)),
                    value: Box::new(Object::Atom(Atom::BIT_ON))
                }
                .into()
            )
        );

        assert_eq!(
            values.next(),
            Some(
                Construction {
                    constructor: Box::new(Object::Atom(Atom::BIT_SLOT_6)),
                    value: Box::new(Object::Atom(Atom::BIT_OFF))
                }
                .into()
            )
        );

        assert_eq!(
            values.next(),
            Some(
                Construction {
                    constructor: Box::new(Object::Atom(Atom::BIT_SLOT_7)),
                    value: Box::new(Object::Atom(Atom::BIT_ON))
                }
                .into()
            )
        );

        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
    }
}
