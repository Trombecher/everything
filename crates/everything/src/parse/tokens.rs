use ecow::EcoString;

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    /// `@<<NAME>>`
    AbstractObject(EcoString),

    /// E.g. `0`, `1`, `42`
    Natural(u64),

    /// `(`
    LeftParenthesis,

    /// `)`
    RightParenthesis,

    /// `{`
    LeftBrace,

    /// `}`
    RightBrace,

    /// `,`
    Comma,

    /// `=>`
    EqualsRightAngle,

    /// `union`
    Union,

    /// `inter`
    Intersection,

    /// `xor`
    Xor,

    /// `exists`
    Exists,

    /// `not`
    Not,

    /// `query`
    Query,

    /// `count`
    Count,

    /// `==`
    EqualsEquals,

    /// `<`
    LeftAngle,

    /// `>`
    RightAngle,

    /// `_`
    Underscore,
}
