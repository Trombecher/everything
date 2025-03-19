mod expr;

pub use expr::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    additional_variable_count: u8,
    root: Expression
}

#[derive(Debug, PartialEq)]
pub struct EncodedConstraint([u8]);

impl EncodedConstraint {
    pub fn decode_partial(&self) -> PartiallyDecodedConstraint {
        todo!()
    }

    pub fn decode(&self) -> Constraint {
        self.decode_partial().decode()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PartiallyDecodedConstraint<'a> {
    additional_variable_count: u8,
    root: PartiallyDecodedExpression<'a>
}

impl<'a> PartiallyDecodedConstraint<'a> {
    pub fn decode(self) -> Constraint {
        todo!()
    }
}