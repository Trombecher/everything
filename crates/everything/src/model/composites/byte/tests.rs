#[allow(non_snake_case)]
mod Byte {
    use super::super::*;

    #[test]
    fn bit() {
        let byte = ByteComposite(0b1011_0111);

        assert_eq!(byte.bit(BitSlot::Slot0), Bit::One);
        assert_eq!(byte.bit(BitSlot::Slot1), Bit::One);
        assert_eq!(byte.bit(BitSlot::Slot2), Bit::One);
        assert_eq!(byte.bit(BitSlot::Slot3), Bit::Zero);
        assert_eq!(byte.bit(BitSlot::Slot4), Bit::One);
        assert_eq!(byte.bit(BitSlot::Slot5), Bit::One);
        assert_eq!(byte.bit(BitSlot::Slot6), Bit::Zero);
        assert_eq!(byte.bit(BitSlot::Slot7), Bit::One);
    }

    #[test]
    fn from_bits() {
        assert_eq!(
            ByteComposite::from_bits([Bit::Zero; 8]),
            ByteComposite(0b0000_0000)
        );
        assert_eq!(
            ByteComposite::from_bits([Bit::One; 8]),
            ByteComposite(0b1111_1111)
        );
        assert_eq!(
            ByteComposite::from_bits([
                Bit::One,
                Bit::Zero,
                Bit::One,
                Bit::Zero,
                Bit::Zero,
                Bit::One,
                Bit::Zero,
                Bit::One,
            ]),
            ByteComposite(0b1010_0101)
        );

        for slot in [
            BitSlot::Slot0,
            BitSlot::Slot1,
            BitSlot::Slot2,
            BitSlot::Slot3,
            BitSlot::Slot4,
            BitSlot::Slot5,
            BitSlot::Slot6,
            BitSlot::Slot7,
        ] {
            let mut bits = [Bit::Zero; 8];
            bits[usize::from(u8::from(slot))] = Bit::One;

            assert_eq!(
                ByteComposite::from_bits(bits),
                ByteComposite(1 << u8::from(slot)),
                "slot {slot:?}"
            );
        }
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
        assert_eq!(format!("{:?}", ByteComposite(0)), "x00");
        assert_eq!(format!("{:?}", ByteComposite(0x10)), "x10");
        assert_eq!(format!("{:?}", ByteComposite(0xA4)), "xA4");
        assert_eq!(format!("{:?}", ByteComposite(0xFF)), "xFF");
    }
}

#[allow(non_snake_case)]
mod ByteValues {
    use super::super::*;

    #[test]
    fn next() {
        let mut values = ByteCompositeValues {
            byte: ByteComposite(0b1010_0101),
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
