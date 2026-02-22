use std::sync::Arc;

use crate::Object;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Property {
    tag: Object,
    value: Object,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Structure(pub Arc<[Property]>);
