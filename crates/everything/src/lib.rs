#![forbid(unsafe_code)]
#![feature(slice_partition_dedup)]
#![warn(clippy::pedantic)]

pub mod base;
pub mod ctx;
pub mod ext;
pub mod knowledge;
pub mod nodes;
pub(crate) mod optimization;
mod set_values;

pub use set_values::*;
