//! This module handles errors.

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Everything(EverythingError),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EverythingError {
    RootPathDoesNotExist,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}

impl Into<Error> for EverythingError {
    fn into(self) -> Error {
        Error::Everything(self)
    }
}

impl<T> Into<Result<T, Error>> for EverythingError {
    fn into(self) -> Result<T, Error> {
        Err(self.into())
    }
}