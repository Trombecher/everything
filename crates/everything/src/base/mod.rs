use std::sync::LazyLock;

use imbl::OrdMap;
use num_bigint::BigInt;

use crate::{
    compute::RawNode,
    knowledge::{Knowledge, Properties},
    model::{Atom, Construction, Object},
};

#[cfg(test)]
mod tests;

/// Returns a constraint that accepts every subject and value.
fn accepts_everything() -> Object {
    // node_function ·· ⟨node_function ·· ⟨node_literal ·· true⟩⟩
    RawNode::function(RawNode::function(RawNode::literal(true))).into_object()
}

/// Returns a constraint that accepts every subject and requires the value to
/// be a valid node.
fn accepts_only_nodes() -> Object {
    // node_function ·· ⟨node_function ·· ⟨node_is_node ·· ⟨node_parameter ·· 0⟩⟩⟩
    RawNode::function(RawNode::function(RawNode::is_node(RawNode::parameter(0)))).into_object()
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

/// Returns a constraint for `for_property_constructor` that requires exactly one
/// such property on the subject and restricts its value with `value_constraint`.
///
/// The uniqueness check is open: it places no other demands on the subject.
fn property_constraint_with_value(
    for_property_constructor: Atom,
    value_constraint: impl Into<RawNode>,
) -> Object {
    // construction_values (subject, for_property_constructor)
    let construction_values = RawNode::query_construction_values(
        RawNode::parameter(1),
        RawNode::literal(for_property_constructor),
    );

    // |construction_values| == 1
    let unique = RawNode::equal(
        RawNode::set_count(construction_values),
        RawNode::literal(BigInt::ONE),
    );

    RawNode::function(RawNode::function(RawNode::and(unique, value_constraint))).into_object()
}

/// Returns the constraint for declarations of a property constructor.
///
/// The constraint checks that the subject has exactly one property that is a
/// construction with `for_property_constructor` as its constructor. Applied to
/// the subject of a declaration, this rejects a second, different constraint.
/// It also requires the declared value, which is the constraint function
/// itself, to be a node.
fn property_constraint(for_property_constructor: Atom) -> Object {
    // The value is the constraint function of the declaration. It has to be a
    // node so that it can be evaluated when the declaration is applied.
    let value_is_node = RawNode::is_node(RawNode::parameter(0));

    property_constraint_with_value(for_property_constructor, value_is_node)
}

/// Returns the constraint for a bit slot property constructor.
///
/// The constraint accepts every subject but requires the subject to have
/// exactly one property with `for_property_constructor` as its constructor. The
/// value of that property must be either `bit_on` or `bit_off`.
fn bit_slot_constraint(for_property_constructor: Atom) -> Object {
    let value_is_bit = RawNode::or(
        RawNode::equal(RawNode::parameter(0), RawNode::literal(Atom::BIT_ON)),
        RawNode::equal(RawNode::parameter(0), RawNode::literal(Atom::BIT_OFF)),
    );

    property_constraint_with_value(for_property_constructor, value_is_bit)
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

    // These constructors still accept every value. Checking the remaining
    // value types (integers, lists, characters, fractions, and sets) needs a
    // reasonably fast type check that the computation system does not provide
    // yet.
    let any_value = accepts_everything();

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
        // Fractions
        Atom::NUMERATOR,
        Atom::DENOMINATOR,
        // Sets
        Atom::CONTAINS,
        // `node_literal` accepts arbitrary data. The depth properties take
        // integers, which are not checked yet.
        Atom::NODE_LITERAL,
        Atom::NODE_PARAMETER,
        Atom::NODE_FUNCTION_SELF,
    ] {
        declare_property_constructor(&mut base, atom, any_value.clone());
    }

    // Bit slots accept any subject, but a bit slot may occur at most once and
    // its value must be either `bit_on` or `bit_off`.
    for atom in [
        Atom::BIT_SLOT_0,
        Atom::BIT_SLOT_1,
        Atom::BIT_SLOT_2,
        Atom::BIT_SLOT_3,
        Atom::BIT_SLOT_4,
        Atom::BIT_SLOT_5,
        Atom::BIT_SLOT_6,
        Atom::BIT_SLOT_7,
    ] {
        declare_property_constructor(&mut base, atom, bit_slot_constraint(atom));
    }

    // Every other node property takes a child node. Their values must be
    // well-formed nodes in the current knowledge.
    let node_value = accepts_only_nodes();

    for atom in [
        Atom::NODE_FUNCTION,
        Atom::NODE_AND_FIRST,
        Atom::NODE_AND_SECOND,
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
        Atom::NODE_IS_NODE,
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
        declare_property_constructor(&mut base, atom, node_value.clone());
    }

    Knowledge { map: base }
});
