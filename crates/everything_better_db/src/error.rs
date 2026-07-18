use std::num::NonZeroU64;

#[derive(Debug)]
pub enum Error {
    OutOfPages,
    PageIdDoesNotExist(NonZeroU64),
}
