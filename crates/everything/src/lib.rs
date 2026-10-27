#![feature(slice_partition_dedup)]
#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

pub mod base;
pub mod compute;
mod iter;
pub mod knowledge;
pub mod model;

// Re-export bigints
pub use num_bigint;
