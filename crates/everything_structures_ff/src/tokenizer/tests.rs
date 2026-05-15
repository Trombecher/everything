use crate::{AbstractSource, ByteSource, IntegerSource, TextSource, Token, Tokenizer};

#[test]
fn tokenization() {
    let mut tokens = Tokenizer::new(
        "{ }()@42234837@0001\n\t\x0c\r ,üä1234567890-1234567890x00x10x42xA7xfF\"10-af\"",
    );

    assert_eq!(tokens.next(), Some(Token::OpeningBrace));
    assert_eq!(tokens.next(), Some(Token::Whitespace(" ")));
    assert_eq!(tokens.next(), Some(Token::ClosingBrace));
    assert_eq!(tokens.next(), Some(Token::OpeningParenthesis));
    assert_eq!(tokens.next(), Some(Token::ClosingParenthesis));
    assert_eq!(
        tokens.next(),
        Some(Token::Abstract(unsafe {
            AbstractSource::new_unchecked("@42234837")
        }))
    );
    assert_eq!(
        tokens.next(),
        Some(Token::Abstract(unsafe {
            AbstractSource::new_unchecked("@0001")
        }))
    );
    assert_eq!(tokens.next(), Some(Token::Whitespace("\n\t\x0c\r ")));
    assert_eq!(tokens.next(), Some(Token::Comma));
    assert_eq!(tokens.next(), Some(Token::Invalid("üä")));
    assert_eq!(
        tokens.next(),
        Some(Token::Integer(unsafe {
            IntegerSource::new_unchecked("1234567890")
        }))
    );
    assert_eq!(
        tokens.next(),
        Some(Token::Integer(unsafe {
            IntegerSource::new_unchecked("-1234567890")
        }))
    );
    assert_eq!(
        tokens.next(),
        Some(Token::Byte(unsafe { ByteSource::new_unchecked("x00") }))
    );
    assert_eq!(
        tokens.next(),
        Some(Token::Byte(unsafe { ByteSource::new_unchecked("x10") }))
    );
    assert_eq!(
        tokens.next(),
        Some(Token::Byte(unsafe { ByteSource::new_unchecked("x42") }))
    );
    assert_eq!(
        tokens.next(),
        Some(Token::Byte(unsafe { ByteSource::new_unchecked("xA7") }))
    );
    assert_eq!(
        tokens.next(),
        Some(Token::Byte(unsafe { ByteSource::new_unchecked("xfF") }))
    );
    assert_eq!(
        tokens.next(),
        Some(Token::Text(unsafe {
            TextSource::new_unchecked("\"10-af\"")
        }))
    );
    assert_eq!(tokens.next(), None);
    assert_eq!(tokens.next(), None);
}
