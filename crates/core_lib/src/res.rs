//! This module handles resources.
//!
//! A _resource_ is a separate file containing content that does not fit into a
//! single row value (size >= 24 bytes).
//! Resources are identified using one [NonZeroU64].
//! The value that references a resource is the owner.
//! No other value can obtain a reference (no shared content).
//!
//! ## Types
//!
//! The type of resource is dictated by its owning value.
//! These are the variable-sized values, that may need offloading:
//!
//! * Binary data (+ encrypted variant)
//! * Text (+ encrypted variant)
//! * Emails (+ encrypted variant)
//! * URLs (+ encrypted variant)
//! * Schemas (because they may contain constraints)
//! * Constraints

use std::num::NonZeroU64;

pub type ResourceId = NonZeroU64;

const RESOURCE_CHUNK_SIZE: usize = 2_usize.pow(16); // Magic number