#[allow(non_snake_case)]
mod Object {
    use super::super::*;

    #[test]
    fn new_natural_number() {
        assert_eq!(
            Object::new_natural_number(0),
            Object::Abstract(Abstract::ZERO)
        );

        for i in (1..200_u128).map(|x| x * 7) {
            assert_eq!(
                Object::new_natural_number(i),
                Object::Structure(Structure::NaturalNumber(NonZeroU128::new(i).unwrap()))
            );
        }
    }

    #[test]
    fn exact_natural_number() {
        assert_eq!(
            Object::Abstract(Abstract::ZERO).exact_natural_number(),
            Some(0)
        );

        assert_eq!(
            Object::Abstract(Abstract(347539486456)).exact_natural_number(),
            None
        );

        assert_eq!(
            Object::Structure(Structure::NaturalNumber(NonZeroU128::new(10).unwrap()))
                .exact_natural_number(),
            Some(10)
        );

        assert_eq!(
            Object::Structure(Structure::Character('x')).exact_natural_number(),
            None
        );
    }
}
