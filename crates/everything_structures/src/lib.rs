#![feature(slice_partition_dedup)]
#![feature(clone_from_ref)]

//! read README

mod fixed_or_more;
mod objects;
mod properties;
mod structures;

pub use fixed_or_more::*;
pub use objects::*;
pub use properties::*;
pub use structures::*;
