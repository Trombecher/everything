use std::fmt::Debug;

use ecow::EcoString;
use ulid::Ulid;

use crate::{Property, Structure};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Abstract(pub EcoString);

impl Abstract {
    pub fn unique() -> Self {
        Self(EcoString::from(Ulid::new().to_string()))
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Object {
    Abstract(Abstract),
    Structure(Structure),
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Abstract(id) => write!(f, "@{}", id.0),
            Self::Structure(s) => Structure::fmt(s, f),
        }
    }
}

macro_rules! builtin {
    ($s:literal) => {
        Self::Abstract(Abstract(EcoString::inline($s)))
    };
}

// Builtin objects
impl Object {
    // Sets
    pub const CONTAINS: Self = builtin!("Contains");

    // Tags
    pub const AXIOMATIC: Self = builtin!("Axiomatic");
    pub const COMPUTED: Self = builtin!("Computed");
    pub const TAG: Self = builtin!("Tag");

    // Integers
    pub const SUCCESSOR_OF: Self = builtin!("SuccessorOf");
    pub const IS_NATURAL: Self = builtin!("IsNatural");
    pub const ZERO: Self = builtin!("Zero");

    // Nodes
    pub const NODE: Self = builtin!("Node");
    pub const NODE_LITERAL: Self = builtin!("Node.Literal");
    pub const NODE_FUNCTION: Self = builtin!("Node.Function");
    pub const NODE_XOR: Self = builtin!("Node.Xor");
    pub const NODE_OR: Self = builtin!("Node.Or");
    pub const NODE_AND: Self = builtin!("Node.And");
    pub const NODE_EQUALS: Self = builtin!("Node.Equals");
    pub const NODE_E: Self = builtin!("Node.E");
    pub const NODE_E_TARGET: Self = builtin!("Node.E.Target");
    pub const NODE_E_TAG: Self = builtin!("Node.E.Tag");
    pub const NODE_E_VALUE: Self = builtin!("Node.E.Value");
    pub const NODE_NOT: Self = builtin!("Node.Not");
    pub const NODE_COUNT: Self = builtin!("Node.Count");
    pub const NODE_QUERY: Self = builtin!("Node.Query");

    /// Peano-definition of natural numbers.
    pub fn from_natural(n: u64) -> Self {
        if n == 0 {
            Self::ZERO
        } else {
            Self::Structure(Structure::new(&mut [Property {
                tag: Self::SUCCESSOR_OF,
                value: Self::from_natural(n - 1),
            }]))
        }
    }

    pub fn node_not(expr: Object) -> Self {
        Self::Structure(Structure::new(&mut [Property {
            tag: Object::NODE_NOT,
            value: expr,
        }]))
    }
}
