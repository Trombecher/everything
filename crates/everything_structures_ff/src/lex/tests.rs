use crate::{Token, TokenKind, tokenize};

#[test]
fn tokenization() {
    assert_eq!(
        tokenize("{ }()@42234837@0001\n\t\x0c\r ,üä")
            .collect::<Vec<_>>()
            .as_slice(),
        [
            Token {
                kind: TokenKind::OpeningBrace,
                length: 1
            },
            Token {
                kind: TokenKind::Whitespace,
                length: 1
            },
            Token {
                kind: TokenKind::ClosingBrace,
                length: 1
            },
            Token {
                kind: TokenKind::OpeningParenthesis,
                length: 1
            },
            Token {
                kind: TokenKind::ClosingParenthesis,
                length: 1
            },
            Token {
                kind: TokenKind::Abstract,
                length: 9
            },
            Token {
                kind: TokenKind::Abstract,
                length: 5
            },
            Token {
                kind: TokenKind::Whitespace,
                length: 5
            },
            Token {
                kind: TokenKind::Comma,
                length: 1
            },
            Token {
                kind: TokenKind::Invalid,
                length: 4
            }
        ]
    )
}
