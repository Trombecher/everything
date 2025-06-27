use std::fmt::Display;

use tokio::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    InvalidValueForInvalidFormatPolicy
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(error) => error.fmt(f),
            Error::InvalidValueForInvalidFormatPolicy => f.write_str("encountered invalid value for the InvalidFormatPolicy in the database metadata"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl std::error::Error for Error {
}
