//! Tests for the parser.

use everything_structures::{Abstract, Object, Property, Structure};
use parser_tools::Span;

use crate::{FilteredToken, Parser};

#[test]
fn parse_structure_continue() {
    let mut parser = Parser::new(
        [
            Span {
                range: 1..2,
                value: FilteredToken::OpeningParenthesis,
            },
            Span {
                range: 3..6,
                value: FilteredToken::Object(Abstract(1).into()),
            },
            Span {
                range: 6..7,
                value: FilteredToken::Comma,
            },
            Span {
                range: 7..9,
                value: FilteredToken::Object(Abstract(9).into()),
            },
            Span {
                range: 9..10,
                value: FilteredToken::ClosingParenthesis,
            },
            Span {
                range: 10..11,
                value: FilteredToken::ClosingBrace,
            },
        ]
        .into_iter(),
    );

    assert_eq!(
        parser.parse_explicit_structure(),
        Ok(Structure::new(&mut [Property {
            tag: Object::Abstract(Abstract(1)),
            value: Object::Abstract(Abstract(9))
        }]))
    );

    assert_eq!(parser.tokens.peek(), None);
}

#[test]
fn parse_object() {
    let mut parser = Parser::new(
        [Span {
            value: FilteredToken::Object(Abstract(20).into()),
            range: 0..3,
        }]
        .into_iter(),
    );

    assert_eq!(parser.parse_object(), Ok(Object::Abstract(Abstract(20))));

    assert_eq!(parser.tokens.peek(), None);
}
