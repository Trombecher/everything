use std::mem::transmute;

use num_enum::TryFromPrimitive;

use crate::{ff, values::{PartiallyDecodedValue, Value}};

#[derive(Debug, Copy, Clone, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum BinaryOperation {
    Addition = ff::expr::ADD,
    Subtraction = ff::expr::SUB,
    Multiplication = ff::expr::MUL,
    Division = ff::expr::DIV,
    Modulus = ff::expr::MOD,
    Equality = ff::expr::EQ,
    Inequality = ff::expr::NEG,
    LessThan = ff::expr::LTH,
    LessThanOrEqual = ff::expr::LE,
    GreaterThan = ff::expr::GTH,
    GreaterThanOrEqual = ff::expr::GE,
}

/// Asserts that some bytes are a valid expression encoding.
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
    pub unsafe fn new(slice: &[u8]) -> &Self {
        unsafe { transmute(slice) }
    }

    pub fn decode(&self) -> PartiallyDecodedExpression {
        match self.0.get(0).copied() {
            Some(ff::expr::VAR) => PartiallyDecodedExpression::Variable(self.0[1]),
            Some(ff::expr::NEG) => {
                PartiallyDecodedExpression::Negate(unsafe {
                    Self::new(&self.0[1..])
                })
            }
            Some(x) if let Ok(bin_op) = BinaryOperation::try_from_primitive(x) => {
                
            }
            _ => unreachable!()
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