#[allow(non_snake_case)]
mod Object {
    use super::super::*;

    #[test]
    fn new_integer() {
        assert_eq!(Object::new_integer(0), Object::Abstract(Abstract::ZERO));

        for i in (-200..200_i128).map(|x| x * 7) {
            assert_eq!(
                Object::new_integer(i),
                NonZeroI128::new(i).map_or_else(
                    || Object::Abstract(Abstract::ZERO),
                    |i| { Object::Composite(Composite::Integer(i)) }
                )
            );
        }
    }

    #[test]
    fn exact_integer() {
        assert_eq!(Object::Abstract(Abstract::ZERO).exact_integer(), Some(0));

        assert_eq!(
            Object::Abstract(Abstract(347539486456)).exact_integer(),
            None
        );

        assert_eq!(
            Object::Composite(Composite::Integer(NonZeroI128::new(10).unwrap())).exact_integer(),
            Some(10)
        );

        assert_eq!(
            Object::Composite(Composite::Character('x')).exact_integer(),
            None
        );
    }
}
