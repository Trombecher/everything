#![forbid(unsafe_code)]
#![feature(slice_partition_dedup)]

pub mod base;
pub mod ctx;
pub mod ext;
mod knowledge;
mod lazy;
pub mod nodes;
pub mod query;
mod set_values;

pub use knowledge::*;
pub use lazy::*;
pub use set_values::*;
