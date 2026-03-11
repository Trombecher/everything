//! Tests for the parser.

use everything_structures::{Change, Object, Property, Structure};
use ulid::Ulid;

use crate::{
    Span,
    parse::{FilteredToken, Parser},
};

#[test]
fn parse_structure_continue() {
    let result = Parser::new(
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
    )
    .parse_structure_continue();

    assert_eq!(
        result,
        Ok(Structure::EMPTY.change(&mut [Change::Add(Property {
            tag: Object::Abstract(Ulid(1)),
            value: Object::Abstract(Ulid(9))
        })]))
    );
}
