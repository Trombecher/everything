#![feature(slice_partition_dedup)]
#![feature(iter_order_by)]

//! read README

mod abstracts;
mod fixed_or_more;
mod objects;
mod properties;
mod structures;

pub use abstracts::*;
pub use fixed_or_more::*;
pub use objects::*;
pub use properties::*;
pub use structures::*;
