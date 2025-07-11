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

use crate::sp::StorageProvider;
use crate::{
    db::Database,
    objects::ObjectId,
    values::Value,
};

/// Indicates that a query field of the triple is not known and therefore demanded and returned by the query.
pub struct Demanded;

trait Sealed {}

/// This trait allows flexible querying of the database.
#[allow(private_bounds)]
pub trait Query<'a, P: StorageProvider>: Sealed {
    type Output: 'a;

    fn query(self, db: &'a Database<P>) -> Self::Output;
}

impl Sealed for (ObjectId, ObjectId, Demanded) {}

impl<'a, P: StorageProvider + 'a> Query<'a, P> for (ObjectId, ObjectId, Demanded) {
    type Output = VFromOT<'a, P>;

    fn query(self, db: &'a Database<P>) -> Self::Output {
        VFromOT {
            db,
            object_id: self.0,
            tag_id: self.1,
            current_row_id: 0,
        }
    }
}

pub struct VFromOT<'a, P: StorageProvider> {
    db: &'a Database<P>,
    object_id: ObjectId,
    tag_id: ObjectId,
    current_row_id: u64,
}

impl<'a, P: StorageProvider> Iterator for VFromOT<'a, P> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: search db

        None
    }
}
