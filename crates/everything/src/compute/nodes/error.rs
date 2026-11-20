use super::kind::{NodeKind, NodeProperty, PropertyRequirement};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum InvalidNodeError {
    #[error("object does not match any node kind")]
    NotANode,

    #[error("object matches more than one node kind (`{first}` and `{second}`)")]
    Ambiguous { first: NodeKind, second: NodeKind },

    #[error(
        "object looks like a node of kind `{kind}`, but required property `{property}` is missing"
    )]
    MissingProperty {
        kind: NodeKind,
        property: NodeProperty,
    },

    #[error(
        "object looks like a node of kind `{kind}`, but required property `{property}` has more than one value"
    )]
    DuplicateProperty {
        kind: NodeKind,
        property: NodeProperty,
    },

    #[error(
        "object looks like a node of kind `{kind}`, but required property `{property}` is not {expected}"
    )]
    InvalidProperty {
        kind: NodeKind,
        property: NodeProperty,
        expected: PropertyRequirement,
    },

    #[error("could not parse `{property}`: {source}")]
    Child {
        property: NodeProperty,
        #[source]
        source: Box<InvalidNodeError>,
    },
}
