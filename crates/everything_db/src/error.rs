use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io")]
    Io(#[from] io::Error),
    #[error("database file is invalid UTF-8")]
    DbFileIsInvalidUTF8,
    #[error("error while parsing database file")]
    ErrorWhileParsingDbFile(#[from] everything_tff::parse::Error),
}
