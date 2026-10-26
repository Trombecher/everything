#[allow(non_snake_case)]
mod Object {
    use super::super::*;

    #[test]
    pub fn good_size() {
        assert!(size_of::<Object>() <= 32);
    }

    #[test]
    pub fn new_integer() {
        assert_eq!(Object::new_integer(BigInt::ZERO), Object::Atom(Atom::ZERO));

        assert_eq!(
            Object::new_integer(BigInt::ONE),
            Object::Set(Set::new_move([Construction::new(
                Atom::SUCCESSOR_OF.into(),
                Atom::ZERO.into()
            )
            .into()]))
        );

        assert_eq!(
            Object::new_integer(BigInt::NEG_ONE),
            Object::Set(Set::new_move([Construction::new(
                Atom::PREDECESSOR_OF.into(),
                Atom::ZERO.into()
            )
            .into()]))
        );
    }

    #[test]
    pub fn integer() {
        for integer in [
            BigInt::ZERO,
            BigInt::ONE,
            BigInt::ONE + BigInt::ONE,
            BigInt::NEG_ONE,
        ] {
            assert_eq!(
                Object::new_integer(integer.clone()).integer(),
                Some(integer)
            );
        }

        assert_eq!(Object::Set(Set::empty()).integer(), None);
    }
}
