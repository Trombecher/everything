//! This module handles queries.
//!
//! ---
//!
//! To query the database, use [Database::query]. It accepts one parameter, a [Query].
//! This is a sealed type which has been implemented for a limited set of triple.
//! Each implementation will yield a different output type. These output types are either
//! booleans (the input tuple exists) or iterators (over all occurances of the demanded parameters).
//! 
//! Keep in mind that an association triple has the shape `(<object_id>, <tag_id>, <value>)`.
//! 
//! ## Implementations
//! 
//! All bullet points have the shape `InputTuple -> OutputType`.
//! 
//! * `(ObjectId, ObjectId, Value) -> bool`
//! * `(ObjectId, ObjectId, Demanded) -> VFromOT`
//! * `(ObjectId, Demanded, Value) -> TFromOV`
//! * `(Demanded, ObjectId, Value) -> OFromTV`
//! * `(ObjectId, Demanded, Demanded) -> TVFromO`
//! * `(Demanded, ObjectId, Demanded) -> OVFromT`
//! * `(Demanded, Demanded, ObjectId) -> OTFromV`
//! * `(Demanded, Demanded, Demanded) -> OTVIter`
//! 
//! ## Examples
//! 
//! Iterate over all values of the assocation of object #20 with tag #30.
//! 
//! ```no_run
//! for value in db.query((20.into(), 30.into(), Demanded)) {
//!     println("{:?}", value);
//! }
//! ```

use crate::{
    db::Database,
    objects::{ObjectId, core::ROWS},
    values::Value,
};

/// Indicates that a query field of the triple is not known and therefore demanded and returned by the query.
pub struct Demanded;

trait Sealed {}

/// This trait allows flexible querying of the database.
#[allow(private_bounds)]
pub trait Query<'a>: Sealed {
    type Output: 'a;

    fn query(self, db: &'a Database) -> Self::Output;
}

impl Sealed for (ObjectId, ObjectId, Demanded) {}

impl<'a> Query<'a> for (ObjectId, ObjectId, Demanded) {
    type Output = VFromOT<'a>;

    fn query(self, db: &'a Database) -> Self::Output {
        VFromOT {
            db,
            object_id: self.0,
            tag_id: self.1,
            current_row_id: 0,
        }
    }
}

pub struct VFromOT<'a> {
    db: &'a Database,
    object_id: ObjectId,
    tag_id: ObjectId,
    current_row_id: u64,
}

impl<'a> Iterator for VFromOT<'a> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_row_id < ROWS.len() as u64 {
            let row = ROWS[self.current_row_id as usize].clone();

            let (found_object_id, found_tag_id, value) =
                unsafe { row.decode().unwrap_unchecked().assume_association() };

            self.current_row_id += 1;

            if found_object_id == self.object_id && found_tag_id == self.tag_id {
                return Some(value);
            }
        }

        // TODO: search db

        None
    }
}