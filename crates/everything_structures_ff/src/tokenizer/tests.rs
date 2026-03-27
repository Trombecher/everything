use crate::{Digits, Token, Tokenizer};

#[test]
fn tokenization() {
    let mut tokens = Tokenizer::new("{ }()@42234837@0001\n\t\x0c\r ,üä1234567890".chars());

    assert_eq!(tokens.next(), Some(Token::OpeningBrace));
    assert_eq!(tokens.next(), Some(Token::Whitespace(" ")));
    assert_eq!(tokens.next(), Some(Token::ClosingBrace));
    assert_eq!(tokens.next(), Some(Token::OpeningParenthesis));
    assert_eq!(tokens.next(), Some(Token::ClosingParenthesis));
    assert_eq!(
        tokens.next(),
        Some(Token::Abstract(unsafe {
            Digits::new_unchecked(b"42234837")
        }))
    );
    assert_eq!(
        tokens.next(),
        Some(Token::Abstract(unsafe { Digits::new_unchecked(b"0001") }))
    );
    assert_eq!(tokens.next(), Some(Token::Whitespace("\n\t\x0c\r ")));
    assert_eq!(tokens.next(), Some(Token::Comma));
    assert_eq!(tokens.next(), Some(Token::Invalid("üä")));
    assert_eq!(
        tokens.next(),
        Some(Token::NaturalNumber(unsafe {
            Digits::new_unchecked(b"1234567890")
        }))
    );
    assert_eq!(tokens.next(), None);
    assert_eq!(tokens.next(), None);
}
