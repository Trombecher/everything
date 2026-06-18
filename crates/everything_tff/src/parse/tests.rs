#[allow(non_snake_case)]
mod Parser {
    mod parse_object {
        use base64::display::Base64Display;

        use super::super::super::*;
        use core::assert_matches;

        #[test]
        fn character_Composite() {
            let cases = ['1', 'ä', '🤙'];

            for c in cases {
                let source = format!(">{c}");
                let mut parser = Parser::new(&source);

                assert_eq!(
                    parser.parse_object(),
                    Ok(Object::Composite(Composite::Character(c)))
                );

                assert_eq!(parser.bytes.index(), 1 + c.len_utf8());
            }
        }

        #[test]
        fn empty_Composite() {
            let mut parser = Parser::new("ExyzÜ");

            assert_eq!(
                parser.parse_object(),
                Ok(Object::Composite(Composite::Empty))
            );

            assert_eq!(parser.bytes.index(), 1);
        }

        #[test]
        fn abstract_objects() {
            let cases = [0_u128, 1, 4359843590834, Abstract::BIT_0.0, u128::MAX];

            for case in cases {
                let source = format!(
                    "@{}.yy3495405",
                    Base64Display::new(
                        &case.to_be_bytes(),
                        &base64::engine::general_purpose::URL_SAFE_NO_PAD
                    )
                );
                let mut parser = Parser::new(&source);

                assert_eq!(parser.parse_object(), Ok(Object::Abstract(Abstract(case))));

                assert_eq!(parser.bytes.next(), Some(b'.'));
            }
        }

        #[test]
        fn positive_integer() {
            let cases = [0_i128, 1, 357863094851, i128::MAX];

            for case in cases {
                let source = format!("{case}.");
                let mut parser = Parser::new(&source);

                assert_eq!(parser.parse_object(), Ok(Object::new_integer(case)));
                assert_eq!(parser.bytes.next(), Some(b'.'));
            }
        }

        #[test]
        fn positive_integer_error() {
            let source = format!("{}1.", i128::MAX);
            let mut parser = Parser::new(&source);

            assert_matches!(parser.parse_object(), Err(_));
            assert_eq!(parser.bytes.index(), source.len() - 2);
        }
    }
}
