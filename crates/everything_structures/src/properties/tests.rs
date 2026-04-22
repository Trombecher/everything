use crate::{Abstract, Bit, BitSlot, Object, Property};

#[test]
fn successor_of() {
    assert_eq!(
        Property::successor_of(42),
        Property {
            tag: Object::Abstract(Abstract::SUCCESSOR_OF),
            value: Object::new_natural_number(42)
        }
    )
}

#[test]
fn bit_slot() {
    assert_eq!(
        Property::bit_slot(BitSlot::Slot1, Bit::One),
        Property {
            tag: Object::Abstract(Abstract::BIT_SLOT_1),
            value: Object::Abstract(Abstract::BIT_1)
        }
    );

    // TODO: maybe full coverage
}
