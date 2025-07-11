#![feature(if_let_guard)]
#![feature(new_zeroed_alloc)]
#![feature(generic_const_exprs)]
#![feature(core_intrinsics)]

mod error;
mod ff;

pub use error::*;

pub mod db;
pub mod objects;
pub mod query;
pub mod sp;
pub mod values;

mod alloc;
mod btree;
mod pages;
mod validation;
mod wal;
