use crate::model::{Atom, AtomOrComposite, Composite, Construction, Object};

/// The reason why a [`Knowledge`](super::Knowledge) is invalid.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum InvalidKnowledgeError {
    /// A base atom occurs, but the constraint declared for it is not the one
    /// from the [base knowledge](crate::base::BASE).
    #[error("base atom `{atom:?}` does not have its constraint from the base knowledge")]
    BaseConstraintNotPinned { atom: Atom },

    /// A statement is not valid.
    #[error("statement `{atom:?} is {property:?}` is invalid: {source}")]
    InvalidStatement {
        atom: Atom,
        property: Object,
        #[source]
        source: Box<InvalidPropertyError>,
    },
}

/// The reason why an object is invalid.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum InvalidObjectError {
    /// The constructor of a construction is not declared as a property
    /// constructor.
    #[error("`{constructor:?}` is not a property constructor")]
    NotAPropertyConstructor { constructor: Object },

    /// The constructor of a construction is itself invalid.
    #[error("the constructor of `{construction:?}` is invalid: {source}")]
    InvalidConstructor {
        construction: Construction,
        #[source]
        source: Box<Self>,
    },

    /// The value of a construction is itself invalid.
    #[error("the value of `{construction:?}` is invalid: {source}")]
    InvalidValue {
        construction: Construction,
        #[source]
        source: Box<Self>,
    },

    /// A member of a composite is not a valid property of the composite.
    #[error("`{property:?}` is not a valid property of `{composite:?}`: {source}")]
    InvalidCompositeProperty {
        composite: Composite,
        property: Object,
        #[source]
        source: Box<InvalidPropertyError>,
    },
}

/// The reason why a property is invalid for an object.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum InvalidPropertyError {
    /// The property is not a valid object.
    #[error("`{property:?}` is not a valid object: {source}")]
    InvalidObject {
        property: Object,
        #[source]
        source: Box<InvalidObjectError>,
    },

    /// The property does not belong to the object.
    #[error("`{property:?}` is not a property of `{object:?}`")]
    DoesNotBelong {
        object: AtomOrComposite,
        property: Object,
    },

    /// A non-constructed property has no unique constraint, so its use is
    /// rejected.
    #[error("`{property:?}` has no unique constraint for non-constructed properties")]
    MissingNonConstructedPropertyConstraint { property: AtomOrComposite },

    /// The constraint declared for the property rejects its use on the object.
    #[error("constraint `{constraint:?}` rejects `{property:?}` as a property of `{object:?}`")]
    RejectedByConstraint {
        object: AtomOrComposite,
        property: Object,
        constraint: Object,
    },
}
