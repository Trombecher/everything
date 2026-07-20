#![feature(pointer_is_aligned_to)]
#![warn(clippy::pedantic)]
#![deny(clippy::arithmetic_side_effects)]
#![allow(clippy::missing_errors_doc)]
#![deny(clippy::std_instead_of_core)]

mod asserts;
mod convert;
mod db;
mod error;
pub mod pages;
mod versions;

pub use db::*;
