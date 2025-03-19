use std::{hint::unreachable_unchecked, mem::transmute};

use num_enum::TryFromPrimitive;

use crate::{decode::{Decodable, PartiallyDecodable}, ff, values::{PartiallyDecodedValue, Value}};

#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum BinaryOperation {
    Addition = ff::expr::ADD,
    Subtraction = ff::expr::SUB,
    Multiplication = ff::expr::MUL,
    Division = ff::expr::DIV,
    Modulus = ff::expr::MOD,
    Equality = ff::expr::EQ,
    Inequality = ff::expr::NEQ,
    LessThan = ff::expr::LTH,
    LessThanOrEqual = ff::expr::LE,
    GreaterThan = ff::expr::GTH,
    GreaterThanOrEqual = ff::expr::GE,
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
        Self::validate(slice).map(|_| unsafe { Self::new_unchecked(slice) })
    }

    /// Checks if the provided slice is a valid encoded Expression, returns `Some(())`. Otherwise `None`.
    fn validate(slice: &[u8]) -> Option<()> {
        match slice.get(0).copied() {
            Some(ff::expr::VAR) => Some(()),
            Some(ff::expr::NEG) => Self::validate(&slice[1..]),
            Some(x) => {
                match BinaryOperation::try_from_primitive(x) {
                    Ok(_) => {
                        let left_expr_len = slice.get(1).copied()? as usize;
                        (left_expr_len < 128).then_some(())?;

                        Self::validate(&slice[2..left_expr_len + 2])
                            .and_then(|_| Self::validate(&slice[left_expr_len + 2..]))
                    }
                    Err(_) => None
                }
            }
            None => None
        }
    }
}

/// A fully owned expression.
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Expression {
    Variable(u8) = ff::expr::VAR,
    Value(Value),
    Negate(Box<Expression>) = ff::expr::NEG,
    BinaryOperation(Box<Expression>, BinaryOperation, Box<Expression>)
}

/// An expression whose top level has been decoded.
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum PartiallyDecodedExpression<'a> {
    Variable(u8) = ff::expr::VAR,
    Value(PartiallyDecodedValue<'a>),
    Negate(&'a EncodedExpression) = ff::expr::NEG,
    BinaryOperation(&'a EncodedExpression, BinaryOperation, &'a EncodedExpression)
}

impl<'a> PartiallyDecodable for &'a EncodedExpression  {
    type PartialOutput = PartiallyDecodedExpression<'a>;

    fn decode_partial(&self) -> Self::PartialOutput {
        match (*self).0.get(0).copied() {
            Some(ff::expr::VAR) => PartiallyDecodedExpression::Variable(self.0[1]),
            Some(ff::expr::NEG) => {
                PartiallyDecodedExpression::Negate(unsafe {
                    EncodedExpression::new_unchecked(&self.0[1..])
                })
            }
            Some(x) => match BinaryOperation::try_from_primitive(x) {
                Ok(bin_op) => unsafe {
                    let left_expr_len = *self.0.get(1).unwrap_unchecked() as usize;

                    PartiallyDecodedExpression::BinaryOperation(
                        EncodedExpression::new_unchecked(&self.0[2..left_expr_len + 2]),
                        bin_op,
                        EncodedExpression::new_unchecked(&self.0[2 + left_expr_len..])
                    )
                }
                Err(_) => unsafe { unreachable_unchecked() }
            }
            None => unsafe { unreachable_unchecked() }
        }
    }
}

impl<'a> Decodable for PartiallyDecodedExpression<'a>  {
    type Output = Expression;

    fn decode(&self) -> Self::Output {
        match *self {
            PartiallyDecodedExpression::Value(value) => Expression::Value(value.decode()),
            PartiallyDecodedExpression::Variable(var) => Expression::Variable(var),
            PartiallyDecodedExpression::Negate(n) => Expression::Negate(Box::new(n.decode())),
            PartiallyDecodedExpression::BinaryOperation(
                left,
                op,
                right
            ) => Expression::BinaryOperation(Box::new(left.decode()), op, Box::new(right.decode()))
        }
    }
}