#![forbid(unsafe_code)]
#![feature(slice_partition_dedup)]
#![warn(clippy::pedantic)]

pub mod base;
pub mod ctx;
pub mod ext;
pub mod nodes;
mod set_values;
pub mod statements;

pub use set_values::*;
