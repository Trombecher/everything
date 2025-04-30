//! This module handles errors.

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Everything(EverythingError),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EverythingError {
    
}