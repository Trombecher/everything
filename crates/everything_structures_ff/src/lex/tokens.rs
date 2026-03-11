pub type SourceIndex = u32;

/// A token emitted by the tokenizer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub length: SourceIndex,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TokenKind {
    /// `@<<DIGITS>>`
    Abstract,

    /// `(`
    OpeningParenthesis,

    /// `)`
    ClosingParenthesis,

    /// `{`
    OpeningBrace,

    /// `}`
    ClosingBrace,

    /// `,`
    Comma,

    /// Whitespace tokens
    Whitespace,

    /// Invalid token
    Invalid,
}
