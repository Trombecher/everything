use std::{hint::unreachable_unchecked, mem::transmute};

use num_enum::TryFromPrimitive;

use crate::{
    decode::{Decodable, PartiallyDecodable},
    ff,
    values::{PartiallyDecodedValue, Value},
};

#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum BinaryOperation {
    Addition = ff::ADD,
    Subtraction = ff::SUB,
    Multiplication = ff::MUL,
    Division = ff::DIV,
    Modulus = ff::MOD,
    Equality = ff::EQ,
    Inequality = ff::NEQ,
    LessThan = ff::LTH,
    LessThanOrEqual = ff::LE,
    GreaterThan = ff::GTH,
    GreaterThanOrEqual = ff::GE,
}

/// Asserts that some bytes are a valid expression encoding.
///
/// Currently, encoded expressions are limited to a length of (at most) 127 bytes.
/// This is subject to change.
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct EncodedExpression([u8]);

impl EncodedExpression {
    /// Wraps a slice.
    ///
    /// By calling this function you assert that the provided slice
    /// contains a valid encoding of an expression.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(slice: &[u8]) -> &Self {
        unsafe { transmute(slice) }
    }

    /// Creates a new [EncodedExpression] by validating the input slice.
    #[inline]
    #[must_use]
    pub fn new(slice: &[u8]) -> Option<&Self> {
        Self::validate(slice).then_some(unsafe { Self::new_unchecked(slice) })
    }

    /// Checks if the provided slice is a valid encoded Expression, returns `Some(())`. Otherwise `None`.
    pub(crate) fn validate(slice: &[u8]) -> bool {
        match slice.get(0).copied() {
            Some(ff::VAR) => true,
            Some(ff::NEG) => Self::validate(&slice[1..]),
            Some(x) => match BinaryOperation::try_from_primitive(x) {
                Ok(_) => {
                    let left_expr_len = match slice.get(1).copied() {
                        Some(x) => x as usize,
                        None => return false,
                    };

                    if left_expr_len >= 128 {
                        return false;
                    }

                    Self::validate(&slice[2..left_expr_len + 2])
                        && Self::validate(&slice[left_expr_len + 2..])
                }
                Err(_) => false,
            },
            None => false,
        }
    }
}

/// A fully owned expression.
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Expression {
    Variable(u8) = ff::VAR,
    Value(Value),
    Negate(Box<Expression>) = ff::NEG,
    BinaryOperation(Box<Expression>, BinaryOperation, Box<Expression>),
}

/// An expression whose top level has been decoded.
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum PartiallyDecodedExpression<'a> {
    Variable(u8) = ff::VAR,
    Value(PartiallyDecodedValue<'a>),
    Negate(&'a EncodedExpression) = ff::NEG,
    BinaryOperation(
        &'a EncodedExpression,
        BinaryOperation,
        &'a EncodedExpression,
    ),
}

impl<'a> PartiallyDecodable for &'a EncodedExpression {
    type PartialOutput = PartiallyDecodedExpression<'a>;

    fn decode_partial(&self) -> Self::PartialOutput {
        match (*self).0.get(0).copied() {
            Some(ff::VAR) => PartiallyDecodedExpression::Variable(self.0[1]),
            Some(ff::NEG) => PartiallyDecodedExpression::Negate(unsafe {
                EncodedExpression::new_unchecked(&self.0[1..])
            }),
            Some(x) => match BinaryOperation::try_from_primitive(x) {
                Ok(bin_op) => unsafe {
                    let left_expr_len = *self.0.get(1).unwrap_unchecked() as usize;

                    PartiallyDecodedExpression::BinaryOperation(
                        EncodedExpression::new_unchecked(&self.0[2..left_expr_len + 2]),
                        bin_op,
                        EncodedExpression::new_unchecked(&self.0[2 + left_expr_len..]),
                    )
                },
                Err(_) => unsafe { unreachable_unchecked() },
            },
            None => unsafe { unreachable_unchecked() },
        }
    }
}

impl<'a> Decodable for PartiallyDecodedExpression<'a> {
    type Output = Expression;

    fn decode(&self) -> Self::Output {
        use PartiallyDecodedExpression as P;

        match *self {
            P::Value(value) => Expression::Value(value.decode()),
            P::Variable(var) => Expression::Variable(var),
            P::Negate(n) => Expression::Negate(Box::new(n.decode())),
            P::BinaryOperation(left, op, right) => {
                Expression::BinaryOperation(Box::new(left.decode()), op, Box::new(right.decode()))
            }
        }
    }
}
