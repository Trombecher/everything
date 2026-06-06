#[derive(Debug)]
pub enum Error {
    Io(tokio::io::Error),
    DbFileIsInvalidUTF8,
    ErrorWhileParsingDbFile(everything_tff::parse::Error),
}
