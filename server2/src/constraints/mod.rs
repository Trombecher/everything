mod expr;

pub use expr::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    additional_variable_count: u8,
    root: Expression
}

pub struct EncodedConstraint([u8]);

pub struct PartiallyDecodedConstraint<'a> {
    additional_variable_count: u8,
    root: PartiallyDecodedExpression<'a>
}