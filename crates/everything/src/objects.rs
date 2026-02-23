use ecow::EcoString;

use crate::Structure;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Object {
    Abstract(EcoString),
    Structure(Structure),
}

macro_rules! builtin {
    ($s:literal) => {
        Self::Abstract(EcoString::inline($s))
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
    pub const IS_INTEGER: Self = builtin!("IsInteger");
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
}
