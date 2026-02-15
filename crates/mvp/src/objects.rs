use std::sync::Arc;

use crate::structures::Structure;

#[derive(Clone, PartialEq, Debug)]
pub enum Id {
    Self_,
    Abstract(u128),
    InlineData(u128),
    Structure(Arc<Structure>),
}

pub const M_TAG: Id = Id::Abstract(1);
pub const M_UNIQUE: Id = Id::Abstract(2);
pub const M_INFERRED: Id = Id::Abstract(3);
pub const M_OBJECT: Id = Id::Abstract(4);
