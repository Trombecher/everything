mod properties;

use imbl::OrdMap;

use crate::model::{Atom, Object, Set};

pub struct Statement {
    pub atom: Atom,
    pub is_object: Object,
}

#[derive(Clone)]
pub struct Knowledge {
    pub map: OrdMap<Atom, Set>,
}

impl Knowledge {}
