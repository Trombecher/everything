//! Tests for the parser.

use everything_structures::{Object, Property, Structure};
use std::assert_matches;
use ulid::Ulid;

use crate::{
    Span,
    parse::{FilteredToken, Parser},
};

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
                value: FilteredToken::Abstract(Ulid(1)),
            },
            Span {
                range: 6..7,
                value: FilteredToken::Comma,
            },
            Span {
                range: 7..9,
                value: FilteredToken::Abstract(Ulid(9)),
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
        parser.parse_structure_continue(),
        Ok(Structure::new(&mut [Property {
            tag: Object::Abstract(Ulid(1)),
            value: Object::Abstract(Ulid(9))
        }]))
    );

    assert_matches!(parser.tokens.peek(), None);
}

#[test]
fn parse_object() {
    let mut parser = Parser::new(
        [Span {
            value: FilteredToken::Abstract(Ulid(20)),
            range: 0..3,
        }]
        .into_iter(),
    );

    assert_eq!(parser.parse_object(), Ok(Object::Abstract(Ulid(20))));

    assert_matches!(parser.tokens.peek(), None);
}
