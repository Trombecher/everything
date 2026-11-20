use super::super::{InvalidNodeError, NodeKind, NodeProperty, PropertyRequirement};
use crate::{knowledge::Knowledge, model::Object};

struct KindSpec {
    kind: NodeKind,
    properties: &'static [NodeProperty],
}

/// Builds a [`KindSpec`] from a kind and its required properties.
macro_rules! kind {
    ($kind:expr, [$($property:expr),* $(,)?]) => {
        KindSpec {
            kind: $kind,
            properties: &[$($property),*],
        }
    };
}

/// Every node kind and its required properties, in the order used by
/// [`RawNode::new`]. This is only consulted after recognition has failed, to
/// explain why the object did not match a kind.
const KINDS: &[KindSpec] = &[
    kind!(NodeKind::Function, [NodeProperty::Function]),
    kind!(NodeKind::Literal, [NodeProperty::Literal]),
    kind!(
        NodeKind::FunctionSelfReference,
        [NodeProperty::FunctionSelf]
    ),
    kind!(NodeKind::Parameter, [NodeProperty::Parameter]),
    kind!(
        NodeKind::Call,
        [NodeProperty::CallCallee, NodeProperty::CallWith]
    ),
    kind!(NodeKind::Count, [NodeProperty::Count]),
    kind!(NodeKind::ValuesOf, [NodeProperty::ValuesOf]),
    kind!(NodeKind::HasValue, [NodeProperty::HasValue]),
    kind!(NodeKind::Not, [NodeProperty::Not]),
    kind!(
        NodeKind::And,
        [NodeProperty::AndFirst, NodeProperty::AndSecond]
    ),
    kind!(
        NodeKind::Or,
        [NodeProperty::OrFirst, NodeProperty::OrSecond]
    ),
    kind!(
        NodeKind::Equal,
        [NodeProperty::EqualFirst, NodeProperty::EqualSecond]
    ),
    kind!(
        NodeKind::Xor,
        [NodeProperty::XorFirst, NodeProperty::XorSecond]
    ),
    kind!(
        NodeKind::Add,
        [NodeProperty::AddFirst, NodeProperty::AddSecond]
    ),
    kind!(
        NodeKind::Union,
        [NodeProperty::UnionFirst, NodeProperty::UnionSecond]
    ),
    kind!(
        NodeKind::Intersection,
        [
            NodeProperty::IntersectionFirst,
            NodeProperty::IntersectionSecond,
        ]
    ),
    kind!(
        NodeKind::Difference,
        [
            NodeProperty::DifferenceFirst,
            NodeProperty::DifferenceSecond,
        ]
    ),
    kind!(
        NodeKind::Map,
        [NodeProperty::MapSet, NodeProperty::MapMapper]
    ),
    kind!(
        NodeKind::Filter,
        [NodeProperty::FilterSet, NodeProperty::FilterFilter]
    ),
    kind!(
        NodeKind::Less,
        [NodeProperty::LessFirst, NodeProperty::LessSecond]
    ),
    kind!(
        NodeKind::If,
        [
            NodeProperty::IfCondition,
            NodeProperty::IfThen,
            NodeProperty::IfElse,
        ]
    ),
    kind!(NodeKind::Unwrap, [NodeProperty::UnwrapSet]),
    kind!(
        NodeKind::Multiply,
        [NodeProperty::MultiplyFirst, NodeProperty::MultiplySecond]
    ),
    kind!(NodeKind::IsAtom, [NodeProperty::IsAtom]),
    kind!(
        NodeKind::Every,
        [NodeProperty::EverySet, NodeProperty::EveryPredicate]
    ),
    kind!(
        NodeKind::Any,
        [NodeProperty::AnySet, NodeProperty::AnyPredicate]
    ),
    kind!(
        NodeKind::QueryAtomsWithProperty,
        [NodeProperty::AtomsWithProperty]
    ),
    kind!(
        NodeKind::QueryAtomsWithConstructor,
        [NodeProperty::AtomsWithConstructor]
    ),
    kind!(
        NodeKind::QueryConstructionValues,
        [
            NodeProperty::ConstructionValuesObject,
            NodeProperty::ConstructionValuesPropertyConstructor,
        ]
    ),
    kind!(
        NodeKind::QueryConstructions,
        [NodeProperty::QueryConstructions]
    ),
    kind!(
        NodeKind::Contains,
        [NodeProperty::ContainsElement, NodeProperty::ContainsSet]
    ),
    kind!(NodeKind::IsConstruction, [NodeProperty::IsConstruction]),
    kind!(NodeKind::IsSet, [NodeProperty::IsSet]),
    kind!(
        NodeKind::Fold,
        [
            NodeProperty::FoldSet,
            NodeProperty::FoldFolder,
            NodeProperty::FoldInitial,
        ]
    ),
    kind!(
        NodeKind::NewConstruction,
        [
            NodeProperty::NewConstructionConstructor,
            NodeProperty::NewConstructionValue,
        ]
    ),
    kind!(
        NodeKind::ConstructionConstructor,
        [NodeProperty::ConstructorOfConstruction]
    ),
    kind!(
        NodeKind::ConstructionValue,
        [NodeProperty::ValueOfConstruction]
    ),
];

/// Explains why no node kind matched. This is only called on the failure path,
/// so node parsing itself stays as cheap as before.
pub(super) fn diagnose(object: &Object, knowledge: &Knowledge) -> InvalidNodeError {
    let mut best: Option<(usize, usize, InvalidNodeError)> = None;

    for spec in KINDS {
        let mut present = 0;
        let mut satisfied = 0;
        let mut error = None;

        for property in spec.properties {
            let mut values = knowledge.query_construction_values(object, property.atom().into());

            let Some(value) = values.next() else {
                error.get_or_insert(InvalidNodeError::MissingProperty {
                    kind: spec.kind,
                    property: *property,
                });

                continue;
            };

            present += 1;

            if values.next().is_some() {
                error.get_or_insert(InvalidNodeError::DuplicateProperty {
                    kind: spec.kind,
                    property: *property,
                });

                continue;
            }

            let valid = match property.requirement() {
                PropertyRequirement::Value => true,
                PropertyRequirement::Depth => value
                    .integer()
                    .and_then(|depth| u32::try_from(depth).ok())
                    .is_some(),
            };

            if valid {
                satisfied += 1;
            } else {
                error.get_or_insert(InvalidNodeError::InvalidProperty {
                    kind: spec.kind,
                    property: *property,
                    expected: property.requirement(),
                });
            }
        }

        let Some(error) = error else {
            continue;
        };

        if present == 0 {
            continue;
        }

        let is_better = best
            .as_ref()
            .is_none_or(|(best_present, best_satisfied, _)| {
                (present, satisfied) > (*best_present, *best_satisfied)
            });

        if is_better {
            best = Some((present, satisfied, error));
        }
    }

    match best {
        Some((_, _, error)) => error,
        None => InvalidNodeError::NotANode,
    }
}
