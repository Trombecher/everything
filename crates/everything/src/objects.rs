use std::sync::Arc;

use arrayvec::ArrayString;

use crate::Structure;

#[derive(Clone, Debug, Eq, Ord)]
pub enum Abstract {
    Inline(ArrayString<15>),
    Allocated(Arc<str>),
}

impl AsRef<str> for Abstract {
    fn as_ref(&self) -> &str {
        match self {
            Self::Inline(s) => s,
            Self::Allocated(a) => a,
        }
    }
}

impl PartialEq for Abstract {
    fn eq(&self, other: &Self) -> bool {
        <str as PartialEq>::eq(self.as_ref(), other.as_ref())
    }
}

impl PartialOrd for Abstract {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        <str as PartialOrd>::partial_cmp(self.as_ref(), other.as_ref())
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Object {
    Abstract(Abstract),
    Structure(Structure),
}

impl Object {}
