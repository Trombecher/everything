use crate::{Span, parse::FilteredToken};

pub type Error = Box<ErrorInfo>;

#[derive(PartialEq, Debug, Clone)]
pub struct ErrorInfo {
    pub found: Option<Span<FilteredToken>>,
    pub message: String,
}
