#![feature(slice_partition_dedup)]

//! # Everything Structure Base
//!
//! This crate provides the basis of Everything.

mod objects;
mod properties;
mod structures;

pub use objects::*;
pub use properties::*;
pub use structures::*;
