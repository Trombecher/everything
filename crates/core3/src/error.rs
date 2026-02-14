use tokio::io;

pub enum Error {
    Io(io::Error),
}
