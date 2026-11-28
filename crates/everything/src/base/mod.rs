use std::sync::LazyLock;

use imbl::OrdMap;
use num_bigint::BigInt;

use crate::{
    knowledge::{Knowledge, Properties},
    model::{Atom, Composite, Construction, Object},
};

#[cfg(test)]
mod tests;

/// Returns a constraint that accepts every subject and value.
fn accepts_everything() -> Object {
    // node_function ·· ⟨node_function ·· ⟨node_literal ·· true⟩⟩
    let inner = Composite::from([Construction::new(
        Atom::NODE_FUNCTION.into(),
        Composite::from([Construction::new(Atom::NODE_LITERAL.into(), Atom::TRUE.into()).into()])
            .into(),
    )
    .into()]);

    Composite::from([Construction::new(Atom::NODE_FUNCTION.into(), inner.into()).into()]).into()
}

/// Declares `atom` to be a property constructor with the given `constraint`.
fn declare_property_constructor(
    base: &mut OrdMap<Atom, Properties>,
    atom: Atom,
    constraint: Object,
) {
    base.insert(
        atom,
        Properties::from([Construction::new(Atom::PROPERTY_CONSTRUCTOR.into(), constraint).into()]),
    );
}

/// Returns the constraint that enforces uniqueness of the declaration of
/// `for_property_constructor` on its subject.
///
/// The constraint checks that the subject has exactly one property that is a
/// construction with `for_property_constructor` as its constructor. Applied to
/// the subject of a declaration, this rejects a second, different constraint.
fn property_constraint(for_property_constructor: Atom) -> Object {
    // contruction_values (subject, for_property_constructor)
    let construction_values: Object = Composite::from([
        Construction::new(
            Atom::NODE_CONSTRUCTION_VALUES_OBJECT.into(),
            Composite::from([Construction::new(
                Atom::NODE_PARAMETER.into(),
                Object::from(BigInt::ONE),
            )
            .into()])
            .into(),
        )
        .into(),
        Construction::new(
            Atom::NODE_CONSTRUCTION_VALUES_PROPERTY_CONSTRUCTOR.into(),
            Composite::from([Construction::new(
                Atom::NODE_LITERAL.into(),
                for_property_constructor.into(),
            )
            .into()])
            .into(),
        )
        .into(),
    ])
    .into();

    // |construction_values| == 1
    let equal = Composite::from([
        Construction::new(
            Atom::NODE_EQUAL_FIRST.into(),
            Composite::from([
                Construction::new(Atom::NODE_SET_COUNT.into(), construction_values).into(),
            ])
            .into(),
        )
        .into(),
        Construction::new(
            Atom::NODE_EQUAL_SECOND.into(),
            Composite::from([Construction::new(
                Atom::NODE_LITERAL.into(),
                Object::from(BigInt::ONE),
            )
            .into()])
            .into(),
        )
        .into(),
    ])
    .into();

    // TODO: add check that value is a node; but for that we need
    // a node that checks if an object is a node (which would
    // be very slow to implement in raw Everything).

    Composite::from([Construction::new(
        Atom::NODE_FUNCTION.into(),
        Composite::from([Construction::new(Atom::NODE_FUNCTION.into(), equal).into()]).into(),
    )
    .into()])
    .into()
}

pub static BASE: LazyLock<Knowledge> = LazyLock::new(|| {
    let mut base = OrdMap::new();

    // --- Absolute base of Everything ---

    declare_property_constructor(
        &mut base,
        Atom::NON_CONSTRUCTED_PROPERTY,
        property_constraint(Atom::NON_CONSTRUCTED_PROPERTY),
    );
    declare_property_constructor(
        &mut base,
        Atom::PROPERTY_CONSTRUCTOR,
        property_constraint(Atom::PROPERTY_CONSTRUCTOR),
    );

    // --- Statements for the computation system to work ---

    // TODO: the declarations below accept every value. They should check
    // that the value is a well-formed node, integer, list, and so on. For
    // nodes this is deferred until there is a reasonably fast node check
    // (see `property_constraint`).

    let constraint = accepts_everything();

    for atom in [
        // Integers
        Atom::SUCCESSOR_OF,
        Atom::PREDECESSOR_OF,
        // Pairs
        Atom::ATOM,
        Atom::VALUE,
        // Lists
        Atom::LIST_ITEM,
        Atom::LIST_TAIL,
        // Characters
        Atom::CODE_POINT,
        // Bytes
        Atom::BIT_SLOT_0,
        Atom::BIT_SLOT_1,
        Atom::BIT_SLOT_2,
        Atom::BIT_SLOT_3,
        Atom::BIT_SLOT_4,
        Atom::BIT_SLOT_5,
        Atom::BIT_SLOT_6,
        Atom::BIT_SLOT_7,
        // Fractions
        Atom::NUMERATOR,
        Atom::DENOMINATOR,
        // Sets
        Atom::CONTAINS,
        // Nodes
        Atom::NODE_FUNCTION,
        Atom::NODE_LITERAL,
        Atom::NODE_AND_FIRST,
        Atom::NODE_AND_SECOND,
        Atom::NODE_PARAMETER,
        Atom::NODE_SET_COUNT,
        Atom::NODE_PROPERTIES_OF,
        Atom::NODE_HAS_A_SET_ITEM,
        Atom::NODE_HAS_PROPERTY_SUBJECT,
        Atom::NODE_HAS_PROPERTY_PROPERTY,
        Atom::NODE_EQUAL_FIRST,
        Atom::NODE_EQUAL_SECOND,
        Atom::NODE_OR_FIRST,
        Atom::NODE_OR_SECOND,
        Atom::NODE_XOR_FIRST,
        Atom::NODE_XOR_SECOND,
        Atom::NODE_NOT,
        Atom::NODE_FUNCTION_SELF,
        Atom::NODE_ADD_FIRST,
        Atom::NODE_ADD_SECOND,
        Atom::NODE_SET_UNION_FIRST,
        Atom::NODE_SET_UNION_SECOND,
        Atom::NODE_SET_INTERSECTION_FIRST,
        Atom::NODE_SET_INTERSECTION_SECOND,
        Atom::NODE_SET_DIFFERENCE_FIRST,
        Atom::NODE_SET_DIFFERENCE_SECOND,
        Atom::NODE_SET_MAP_SET,
        Atom::NODE_SET_MAP_MAPPER,
        Atom::NODE_SET_EVERY_SET,
        Atom::NODE_SET_EVERY_PREDICATE,
        Atom::NODE_SET_ANY_SET,
        Atom::NODE_SET_ANY_PREDICATE,
        Atom::NODE_SET_FILTER_SET,
        Atom::NODE_SET_FILTER_FILTER,
        Atom::NODE_LESS_FIRST,
        Atom::NODE_LESS_SECOND,
        Atom::NODE_IF_CONDITION,
        Atom::NODE_IF_THEN,
        Atom::NODE_IF_ELSE,
        Atom::NODE_SET_UNWRAP_SET,
        Atom::NODE_MULTIPLY_FIRST,
        Atom::NODE_MULTIPLY_SECOND,
        Atom::NODE_CALL_CALLEE,
        Atom::NODE_CALL_WITH,
        Atom::NODE_CONSTRUCTION_VALUES_OBJECT,
        Atom::NODE_CONSTRUCTION_VALUES_PROPERTY_CONSTRUCTOR,
        Atom::NODE_ATOMS_WITH_PROPERTY,
        Atom::NODE_SET_CONTAINS_ELEMENT,
        Atom::NODE_SET_CONTAINS_SET,
        Atom::NODE_IS_ATOM,
        Atom::NODE_IS_CONSTRUCTION,
        Atom::NODE_IS_COMPOSITE,
        Atom::NODE_NEW_CONSTRUCTION_CONSTRUCTOR,
        Atom::NODE_NEW_CONSTRUCTION_VALUE,
        Atom::NODE_NEW_SET_ITEM,
        Atom::NODE_NEW_COMPOSITE_PROPERTY,
        Atom::NODE_CONSTRUCTOR_OF_CONSTRUCTION,
        Atom::NODE_VALUE_OF_CONSTRUCTION,
        Atom::NODE_SET_FOLD_SET,
        Atom::NODE_SET_FOLD_FOLDER,
        Atom::NODE_SET_FOLD_INITIAL,
        Atom::NODE_QUERY_CONSTRUCTIONS,
        Atom::NODE_ATOMS_WITH_CONSTRUCTOR,
    ] {
        declare_property_constructor(&mut base, atom, constraint.clone());
    }

    Knowledge { map: base }
});
