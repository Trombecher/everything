use std::ops::Range;

use crate::SourceIndex;

pub type Error = Box<ErrorInfo>;

#[derive(PartialEq, Debug, Clone)]
pub struct ErrorInfo {
    pub range: Option<Range<SourceIndex>>,
    pub message: String,
}
