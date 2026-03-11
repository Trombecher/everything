//! Tests for the parser.

fn tokens<const N: usize>(
    tokens: [Token; N],
) -> impl FallibleIterator<Item = Token, Error = Error> {
    IntoFallible::from(tokens.into_iter()).map_err(|_| "".into())
}

#[test]
fn parse_function_expression() {}

#[test]
fn parse_abstract_object_expression() {
    let tokens = tokens([Token::AbstractObject("A".into())]);

    assert_eq!(
        Parser::new(tokens).parse_expression(BindingPrecedance::Minimum),
        Ok(Object::Abstract(Abstract("A".into())))
    );
}

#[test]
fn parse_natural_expression() {
    let mut parser = Parser::new(tokens([Token::Natural(42)]));

    assert_eq!(
        parser.parse_expression(BindingPrecedance::Minimum),
        Ok(Object::from_natural(42))
    );
}

#[test]
fn parse_structure_expression() {
    let tokens = tokens([
        Token::LeftBrace,
        Token::LeftParenthesis,
        Token::AbstractObject("A".into()),
        Token::Comma,
        Token::AbstractObject("B".into()),
        Token::Comma, // Trailing comma
        Token::RightParenthesis,
        Token::LeftParenthesis,
        Token::AbstractObject("C".into()),
        Token::Comma,
        Token::AbstractObject("D".into()),
        Token::RightParenthesis,
        Token::RightBrace,
    ]);

    assert_eq!(
        Parser::new(tokens).parse_expression(BindingPrecedance::Minimum),
        Ok(Object::Structure(Structure::new(&mut [
            Property {
                tag: Object::Abstract(Abstract("A".into())),
                value: Object::Abstract(Abstract("B".into()))
            },
            Property {
                tag: Object::Abstract(Abstract("C".into())),
                value: Object::Abstract(Abstract("D".into()))
            }
        ])))
    )
}
