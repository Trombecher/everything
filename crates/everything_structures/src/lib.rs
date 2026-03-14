#![feature(slice_partition_dedup)]
#![feature(clone_from_ref)]

//! This crate provides the basis of Everything.

mod objects;
mod properties;
mod structures;

pub use objects::*;
pub use properties::*;
pub use structures::*;
