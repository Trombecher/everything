//! This module declares the representations of [Duration] and [DateTime].
//!
//! ## Why not use std?
//!
//! * std's implementation of time does not allow for negative durations.
//! * A moment in time's representation is system-dependent and therefore
//! not suitable for a system-independent DBMS.

mod duration;
mod datetime;
mod i120;

pub use duration::*;
pub use datetime::*;
pub use i120::*;