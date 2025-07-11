use crate::values::inline::{InlineStr, InlineStrMax};

#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct Email(InlineStr);

#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct EmailMax(InlineStrMax);
