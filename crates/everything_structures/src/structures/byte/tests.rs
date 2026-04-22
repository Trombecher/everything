#![allow(non_snake_case)]

use super::*;
use crate::{Abstract, Object, Property};

#[test]
fn Byte__bit() {
    let byte = Byte(0b10110111);

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
fn Byte__has() {
    let byte = Byte(0b00110101);

    assert!(!byte.has(
        &Object::Abstract(Abstract(34598)),
        &Object::Abstract(Abstract(306985430))
    ));

    assert!(!byte.has(
        &Object::Abstract(Abstract::BIT_SLOT_0),
        &Object::Abstract(Abstract(34598))
    ));

    assert!(!byte.has(
        &Object::Abstract(Abstract::BIT_SLOT_0),
        &Object::Abstract(Abstract::BIT_0)
    ));

    assert!(byte.has(
        &Object::Abstract(Abstract::BIT_SLOT_0),
        &Object::Abstract(Abstract::BIT_1)
    ));

    assert!(byte.has(
        &Object::Abstract(Abstract::BIT_SLOT_1),
        &Object::Abstract(Abstract::BIT_0)
    ));

    assert!(byte.has(
        &Object::Abstract(Abstract::BIT_SLOT_2),
        &Object::Abstract(Abstract::BIT_1)
    ));

    assert!(byte.has(
        &Object::Abstract(Abstract::BIT_SLOT_3),
        &Object::Abstract(Abstract::BIT_0)
    ));

    assert!(byte.has(
        &Object::Abstract(Abstract::BIT_SLOT_4),
        &Object::Abstract(Abstract::BIT_1)
    ));

    assert!(byte.has(
        &Object::Abstract(Abstract::BIT_SLOT_5),
        &Object::Abstract(Abstract::BIT_1)
    ));

    assert!(byte.has(
        &Object::Abstract(Abstract::BIT_SLOT_6),
        &Object::Abstract(Abstract::BIT_0)
    ));

    assert!(byte.has(
        &Object::Abstract(Abstract::BIT_SLOT_7),
        &Object::Abstract(Abstract::BIT_0)
    ));
}

#[test]
fn Byte__tag() {
    assert_eq!(
        Byte(10).tags(Object::Abstract(Abstract(5834953))),
        ByteTags { slots: 0 }
    );

    assert_eq!(
        Byte(42).tags(Object::Abstract(Abstract::BIT_0)),
        ByteTags { slots: !42 }
    );

    assert_eq!(
        Byte(42).tags(Object::Abstract(Abstract::BIT_1)),
        ByteTags { slots: 42 }
    );
}

#[test]
fn ByteProperties__next() {
    let mut properties = ByteProperties {
        byte: Byte(0b10100101),
        next_slot: Some(BitSlot::Slot0),
    };

    assert_eq!(
        properties.next(),
        Some(Property {
            tag: Object::Abstract(Abstract::BIT_SLOT_0),
            value: Object::Abstract(Abstract::BIT_1)
        })
    );

    assert_eq!(
        properties.next(),
        Some(Property {
            tag: Object::Abstract(Abstract::BIT_SLOT_1),
            value: Object::Abstract(Abstract::BIT_0)
        })
    );

    assert_eq!(
        properties.next(),
        Some(Property {
            tag: Object::Abstract(Abstract::BIT_SLOT_2),
            value: Object::Abstract(Abstract::BIT_1)
        })
    );

    assert_eq!(
        properties.next(),
        Some(Property {
            tag: Object::Abstract(Abstract::BIT_SLOT_3),
            value: Object::Abstract(Abstract::BIT_0)
        })
    );

    assert_eq!(
        properties.next(),
        Some(Property {
            tag: Object::Abstract(Abstract::BIT_SLOT_4),
            value: Object::Abstract(Abstract::BIT_0)
        })
    );

    assert_eq!(
        properties.next(),
        Some(Property {
            tag: Object::Abstract(Abstract::BIT_SLOT_5),
            value: Object::Abstract(Abstract::BIT_1)
        })
    );

    assert_eq!(
        properties.next(),
        Some(Property {
            tag: Object::Abstract(Abstract::BIT_SLOT_6),
            value: Object::Abstract(Abstract::BIT_0)
        })
    );

    assert_eq!(
        properties.next(),
        Some(Property {
            tag: Object::Abstract(Abstract::BIT_SLOT_7),
            value: Object::Abstract(Abstract::BIT_1)
        })
    );

    assert_eq!(properties.next(), None);
    assert_eq!(properties.next(), None);
    assert_eq!(properties.next(), None);
    assert_eq!(properties.next(), None);
}

#[test]
fn ByteValues__next() {
    let mut values = ByteValues(None);
    assert_eq!(values.next(), None);
    assert_eq!(values.next(), None);

    let mut values = ByteValues(Some(Bit::One));
    assert_eq!(values.next(), Some(Object::Abstract(Abstract::BIT_1)));
    assert_eq!(values.next(), None);
    assert_eq!(values.next(), None);
}

#[test]
fn ByteTags__next() {
    let mut tags = ByteTags { slots: 0b10001011 };

    assert_eq!(tags.next(), Some(BitSlot::Slot0));
    assert_eq!(tags.next(), Some(BitSlot::Slot1));
    assert_eq!(tags.next(), Some(BitSlot::Slot3));
    assert_eq!(tags.next(), Some(BitSlot::Slot7));
    assert_eq!(tags.next(), None);
    assert_eq!(tags.next(), None);
    assert_eq!(tags.next(), None);
}
