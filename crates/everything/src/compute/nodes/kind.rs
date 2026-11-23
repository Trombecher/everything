use std::fmt;

use crate::model::Atom;

/// A node kind recognized by the computation system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Function,
    Literal,
    FunctionSelfReference,
    Parameter,
    Call,
    Count,
    ValuesOf,
    HasValue,
    Not,
    And,
    Or,
    Equal,
    Xor,
    Add,
    Union,
    Intersection,
    Difference,
    Map,
    Filter,
    Less,
    If,
    Unwrap,
    Multiply,
    IsAtom,
    Every,
    Any,
    QueryAtomsWithProperty,
    QueryAtomsWithConstructor,
    QueryConstructionValues,
    QueryConstructions,
    Contains,
    IsConstruction,
    IsSet,
    Fold,
    NewConstruction,
    NewSet,
    ConstructionConstructor,
    ConstructionValue,
}

impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Function => "function",
            Self::Literal => "literal",
            Self::FunctionSelfReference => "function self-reference",
            Self::Parameter => "parameter",
            Self::Call => "call",
            Self::Count => "count",
            Self::ValuesOf => "values of",
            Self::HasValue => "has value",
            Self::Not => "not",
            Self::And => "and",
            Self::Or => "or",
            Self::Equal => "equal",
            Self::Xor => "xor",
            Self::Add => "add",
            Self::Union => "union",
            Self::Intersection => "intersection",
            Self::Difference => "difference",
            Self::Map => "map",
            Self::Filter => "filter",
            Self::Less => "less",
            Self::If => "if",
            Self::Unwrap => "unwrap",
            Self::Multiply => "multiply",
            Self::IsAtom => "is atom",
            Self::Every => "every",
            Self::Any => "any",
            Self::QueryAtomsWithProperty => "query atoms with property",
            Self::QueryAtomsWithConstructor => "query atoms with constructor",
            Self::QueryConstructionValues => "query construction values",
            Self::QueryConstructions => "query constructions",
            Self::Contains => "contains",
            Self::IsConstruction => "is construction",
            Self::IsSet => "is set",
            Self::Fold => "fold",
            Self::NewConstruction => "new construction",
            Self::NewSet => "new set",
            Self::ConstructionConstructor => "construction constructor",
            Self::ConstructionValue => "construction value",
        })
    }
}

/// The type of value required by a node property.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyRequirement {
    Value,
    /// A non-negative integer that fits into a `u32`.
    Depth,
}

impl fmt::Display for PropertyRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Value => "a value",
            Self::Depth => "a non-negative 32-bit integer",
        })
    }
}

/// A property that is required by at least one node kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeProperty {
    Function,
    Literal,
    FunctionSelf,
    Parameter,
    CallCallee,
    CallWith,
    Count,
    ValuesOf,
    HasValue,
    Not,
    AndFirst,
    AndSecond,
    OrFirst,
    OrSecond,
    EqualFirst,
    EqualSecond,
    XorFirst,
    XorSecond,
    AddFirst,
    AddSecond,
    UnionFirst,
    UnionSecond,
    IntersectionFirst,
    IntersectionSecond,
    DifferenceFirst,
    DifferenceSecond,
    MapSet,
    MapMapper,
    FilterSet,
    FilterFilter,
    LessFirst,
    LessSecond,
    IfCondition,
    IfThen,
    IfElse,
    UnwrapSet,
    MultiplyFirst,
    MultiplySecond,
    IsAtom,
    EverySet,
    EveryPredicate,
    AnySet,
    AnyPredicate,
    AtomsWithProperty,
    AtomsWithConstructor,
    ConstructionValuesObject,
    ConstructionValuesPropertyConstructor,
    QueryConstructions,
    ContainsElement,
    ContainsSet,
    IsConstruction,
    IsSet,
    FoldSet,
    FoldFolder,
    FoldInitial,
    NewConstructionConstructor,
    NewConstructionValue,
    NewSetValue,
    ConstructorOfConstruction,
    ValueOfConstruction,
}

impl NodeProperty {
    #[must_use]
    pub(crate) const fn atom(self) -> Atom {
        match self {
            Self::Function => Atom::NODE_FUNCTION,
            Self::Literal => Atom::NODE_LITERAL,
            Self::FunctionSelf => Atom::NODE_FUNCTION_SELF,
            Self::Parameter => Atom::NODE_PARAMETER,
            Self::CallCallee => Atom::NODE_CALL_CALLEE,
            Self::CallWith => Atom::NODE_CALL_WITH,
            Self::Count => Atom::NODE_COUNT,
            Self::ValuesOf => Atom::NODE_VALUES_OF,
            Self::HasValue => Atom::NODE_HAS_VALUE,
            Self::Not => Atom::NODE_NOT,
            Self::AndFirst => Atom::NODE_AND_FIRST,
            Self::AndSecond => Atom::NODE_AND_SECOND,
            Self::OrFirst => Atom::NODE_OR_FIRST,
            Self::OrSecond => Atom::NODE_OR_SECOND,
            Self::EqualFirst => Atom::NODE_EQUAL_FIRST,
            Self::EqualSecond => Atom::NODE_EQUAL_SECOND,
            Self::XorFirst => Atom::NODE_XOR_FIRST,
            Self::XorSecond => Atom::NODE_XOR_SECOND,
            Self::AddFirst => Atom::NODE_ADD_FIRST,
            Self::AddSecond => Atom::NODE_ADD_SECOND,
            Self::UnionFirst => Atom::NODE_UNION_FIRST,
            Self::UnionSecond => Atom::NODE_UNION_SECOND,
            Self::IntersectionFirst => Atom::NODE_INTERSECTION_FIRST,
            Self::IntersectionSecond => Atom::NODE_INTERSECTION_SECOND,
            Self::DifferenceFirst => Atom::NODE_DIFFERENCE_FIRST,
            Self::DifferenceSecond => Atom::NODE_DIFFERENCE_SECOND,
            Self::MapSet => Atom::NODE_MAP_SET,
            Self::MapMapper => Atom::NODE_MAP_MAPPER,
            Self::FilterSet => Atom::NODE_FILTER_SET,
            Self::FilterFilter => Atom::NODE_FILTER_FILTER,
            Self::LessFirst => Atom::NODE_LESS_FIRST,
            Self::LessSecond => Atom::NODE_LESS_SECOND,
            Self::IfCondition => Atom::NODE_IF_CONDITION,
            Self::IfThen => Atom::NODE_IF_THEN,
            Self::IfElse => Atom::NODE_IF_ELSE,
            Self::UnwrapSet => Atom::NODE_UNWRAP_SET,
            Self::MultiplyFirst => Atom::NODE_MULTIPLY_FIRST,
            Self::MultiplySecond => Atom::NODE_MULTIPLY_SECOND,
            Self::IsAtom => Atom::NODE_IS_ATOM,
            Self::EverySet => Atom::NODE_EVERY_SET,
            Self::EveryPredicate => Atom::NODE_EVERY_PREDICATE,
            Self::AnySet => Atom::NODE_ANY_SET,
            Self::AnyPredicate => Atom::NODE_ANY_PREDICATE,
            Self::AtomsWithProperty => Atom::NODE_ATOMS_WITH_PROPERTY,
            Self::AtomsWithConstructor => Atom::NODE_ATOMS_WITH_CONSTRUCTOR,
            Self::ConstructionValuesObject => Atom::NODE_CONSTRUCTION_VALUES_OBJECT,
            Self::ConstructionValuesPropertyConstructor => {
                Atom::NODE_CONSTRUCTION_VALUES_PROPERTY_CONSTRUCTOR
            }
            Self::QueryConstructions => Atom::NODE_QUERY_CONSTRUCTIONS,
            Self::ContainsElement => Atom::NODE_CONTAINS_ELEMENT,
            Self::ContainsSet => Atom::NODE_CONTAINS_SET,
            Self::IsConstruction => Atom::NODE_IS_CONSTRUCTION,
            Self::IsSet => Atom::NODE_IS_SET,
            Self::FoldSet => Atom::NODE_FOLD_SET,
            Self::FoldFolder => Atom::NODE_FOLD_FOLDER,
            Self::FoldInitial => Atom::NODE_FOLD_INITIAL,
            Self::NewConstructionConstructor => Atom::NODE_NEW_CONSTRUCTION_CONSTRUCTOR,
            Self::NewConstructionValue => Atom::NODE_NEW_CONSTRUCTION_VALUE,
            Self::NewSetValue => Atom::NODE_NEW_SET_VALUE,
            Self::ConstructorOfConstruction => Atom::NODE_CONSTRUCTOR_OF_CONSTRUCTION,
            Self::ValueOfConstruction => Atom::NODE_VALUE_OF_CONSTRUCTION,
        }
    }

    #[must_use]
    pub(crate) const fn requirement(self) -> PropertyRequirement {
        match self {
            Self::FunctionSelf | Self::Parameter => PropertyRequirement::Depth,
            _ => PropertyRequirement::Value,
        }
    }

    /// Returns `true` iff an object may have any positive number of values for
    /// this property. Every other property must have exactly one value.
    #[must_use]
    pub(crate) const fn allows_multiple(self) -> bool {
        matches!(self, Self::NewSetValue)
    }
}

impl fmt::Display for NodeProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Function => "node_function",
            Self::Literal => "node_literal",
            Self::FunctionSelf => "node_function_self",
            Self::Parameter => "node_parameter",
            Self::CallCallee => "node_call_callee",
            Self::CallWith => "node_call_with",
            Self::Count => "node_count",
            Self::ValuesOf => "node_values_of",
            Self::HasValue => "node_has_value",
            Self::Not => "node_not",
            Self::AndFirst => "node_and_first",
            Self::AndSecond => "node_and_second",
            Self::OrFirst => "node_or_first",
            Self::OrSecond => "node_or_second",
            Self::EqualFirst => "node_equal_first",
            Self::EqualSecond => "node_equal_second",
            Self::XorFirst => "node_xor_first",
            Self::XorSecond => "node_xor_second",
            Self::AddFirst => "node_add_first",
            Self::AddSecond => "node_add_second",
            Self::UnionFirst => "node_union_first",
            Self::UnionSecond => "node_union_second",
            Self::IntersectionFirst => "node_intersection_first",
            Self::IntersectionSecond => "node_intersection_second",
            Self::DifferenceFirst => "node_difference_first",
            Self::DifferenceSecond => "node_difference_second",
            Self::MapSet => "node_map_set",
            Self::MapMapper => "node_map_mapper",
            Self::FilterSet => "node_filter_set",
            Self::FilterFilter => "node_filter_filter",
            Self::LessFirst => "node_less_first",
            Self::LessSecond => "node_less_second",
            Self::IfCondition => "node_if_condition",
            Self::IfThen => "node_if_then",
            Self::IfElse => "node_if_else",
            Self::UnwrapSet => "node_unwrap_set",
            Self::MultiplyFirst => "node_multiply_first",
            Self::MultiplySecond => "node_multiply_second",
            Self::IsAtom => "node_is_atom",
            Self::EverySet => "node_every_set",
            Self::EveryPredicate => "node_every_predicate",
            Self::AnySet => "node_any_set",
            Self::AnyPredicate => "node_any_predicate",
            Self::AtomsWithProperty => "node_atoms_with_property",
            Self::AtomsWithConstructor => "node_atoms_with_constructor",
            Self::ConstructionValuesObject => "node_construction_values_object",
            Self::ConstructionValuesPropertyConstructor => {
                "node_construction_values_property_constructor"
            }
            Self::QueryConstructions => "node_query_constructions",
            Self::ContainsElement => "node_contains_element",
            Self::ContainsSet => "node_contains_set",
            Self::IsConstruction => "node_is_construction",
            Self::IsSet => "node_is_set",
            Self::FoldSet => "node_fold_set",
            Self::FoldFolder => "node_fold_folder",
            Self::FoldInitial => "node_fold_initial",
            Self::NewConstructionConstructor => "node_new_construction_constructor",
            Self::NewConstructionValue => "node_new_construction_value",
            Self::NewSetValue => "node_new_set_value",
            Self::ConstructorOfConstruction => "node_constructor_of_construction",
            Self::ValueOfConstruction => "node_value_of_construction",
        })
    }
}
