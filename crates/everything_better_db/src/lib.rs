#![feature(pointer_is_aligned_to)]
#![deny(clippy::arithmetic_side_effects)]

mod asserts;
mod convert;
mod db;
mod error;
pub mod pages;
mod versions;

pub use db::*;
