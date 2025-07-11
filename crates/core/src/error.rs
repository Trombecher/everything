use std::fmt::Display;

use crate::pages::PageId;
use tokio::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    InvalidPageKind(PageId, u8),
    InvalidValueKind(u8),
    PageCorrupted(PageId),
    InvalidValueForInvalidFormatPolicy,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(error) => error.fmt(f),
            Error::InvalidValueForInvalidFormatPolicy => f.write_str(
                "encountered invalid value for the InvalidFormatPolicy in the database metadata",
            ),
            Error::InvalidPageKind(byte, page_id) => write!(
                f,
                "encountered invalid page kind with value {} on page id {}",
                byte, page_id
            ),
            Error::PageCorrupted(page_id) => {
                write!(f, "encountered corrupted page with id {}", page_id)
            }
            Error::InvalidValueKind(kind) => write!(f, "encountered invalid value kind {}", kind),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl std::error::Error for Error {}
