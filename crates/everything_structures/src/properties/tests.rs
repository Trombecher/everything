#[allow(non_snake_case)]
mod Property {
    use super::super::*;

    #[test]
    fn successor_of() {
        assert_eq!(
            Property::new_successor_of(42),
            Property {
                tag: Object::Abstract(Abstract::SUCCESSOR_OF),
                value: Object::new_natural_number(42)
            }
        )
    }

    #[test]
    fn bit_slot() {
        assert_eq!(
            Property::new_bit_slot(BitSlot::Slot1, Bit::One),
            Property {
                tag: Object::Abstract(Abstract::BIT_SLOT_1),
                value: Object::Abstract(Abstract::BIT_1)
            }
        );

        // TODO: maybe full coverage
    }

    // TODO: maybe add more tests (though these functions are correct I think)
}
