#![feature(slice_partition_dedup)]
#![feature(clone_from_ref)]

//! This crate provides the basis of Everything.

mod objects;
mod properties;
mod structures;

use deranged::RangedU128;
pub use objects::*;
pub use properties::*;
pub use structures::*;

#[allow(non_camel_case_types)]
pub type u126 = RangedU128<0, { u128::MAX >> 2 }>;
