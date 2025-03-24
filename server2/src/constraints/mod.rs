mod expr;

use std::mem::transmute;

pub use expr::*;

use crate::decode::{Decodable, PartiallyDecodable, read_bytes};

#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    additional_variable_count: u8,
    root: Expression,
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
        Self::validate(slice).then_some(unsafe { Self::new_unchecked(slice) })
    }

    pub(crate) fn validate(slice: &[u8]) -> bool {
        slice.len() >= 1 && EncodedExpression::validate(&slice[4..])
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PartiallyDecodedConstraint<'a> {
    additional_variable_count: u8,
    root: PartiallyDecodedExpression<'a>,
}

impl<'a> PartiallyDecodable for &'a EncodedConstraint {
    type PartialOutput = PartiallyDecodedConstraint<'a>;

    fn decode_partial(&self) -> Self::PartialOutput {
        let avc = unsafe { read_bytes::<1>(&self.0, 0)[0] };
        let root = unsafe { EncodedExpression::new_unchecked(&self.0[1..]) }.decode_partial();

        PartiallyDecodedConstraint {
            additional_variable_count: avc,
            root,
        }
    }
}

impl<'a> Decodable for PartiallyDecodedConstraint<'a> {
    type Output = Constraint;

    fn decode(&self) -> Self::Output {
        let PartiallyDecodedConstraint {
            additional_variable_count,
            root
        } = *self;

        Constraint {
            additional_variable_count,
            root: root.decode()
        }
    }
}
