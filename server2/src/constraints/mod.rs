mod expr;

use std::mem::transmute;

pub use expr::*;

use crate::decode::{Decodable, PartiallyDecodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    additional_variable_count: u8,
    root: Expression
}

#[derive(Debug, PartialEq)]
pub struct EncodedConstraint([u8]);

impl EncodedConstraint {
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(slice: &[u8]) -> &Self {
        unsafe { transmute(slice) }
    }

    #[inline]
    #[must_use]
    pub fn new(slice: &[u8]) -> Option<&Self> {
        Self::validate(slice)
            .then_some(unsafe { Self::new_unchecked(slice) })
    }

    pub(crate) fn validate(slice: &[u8]) -> bool {
        todo!()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PartiallyDecodedConstraint<'a> {
    additional_variable_count: u8,
    root: PartiallyDecodedExpression<'a>
}

impl<'a> PartiallyDecodable for &'a EncodedConstraint {
    type PartialOutput = PartiallyDecodedConstraint<'a>;

    fn decode_partial(&self) -> Self::PartialOutput {
        todo!()
    }
}

impl<'a> Decodable for PartiallyDecodedConstraint<'a> {
    type Output = Constraint;

    fn decode(&self) -> Self::Output {
        todo!()
    }
}