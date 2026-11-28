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
    kind!(NodeKind::SetCount, [NodeProperty::SetCount]),
    kind!(NodeKind::PropertiesOf, [NodeProperty::PropertiesOf]),
    kind!(NodeKind::CompositeFromSet, [NodeProperty::CompositeFromSet]),
    kind!(NodeKind::HasASetItem, [NodeProperty::HasASetItem]),
    kind!(
        NodeKind::HasProperty,
        [
            NodeProperty::HasPropertySubject,
            NodeProperty::HasPropertyProperty,
        ]
    ),
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
        NodeKind::SetUnion,
        [NodeProperty::SetUnionFirst, NodeProperty::SetUnionSecond]
    ),
    kind!(
        NodeKind::SetIntersection,
        [
            NodeProperty::SetIntersectionFirst,
            NodeProperty::SetIntersectionSecond,
        ]
    ),
    kind!(
        NodeKind::SetDifference,
        [
            NodeProperty::SetDifferenceFirst,
            NodeProperty::SetDifferenceSecond,
        ]
    ),
    kind!(
        NodeKind::SetMap,
        [NodeProperty::SetMapSet, NodeProperty::SetMapMapper]
    ),
    kind!(
        NodeKind::SetFilter,
        [NodeProperty::SetFilterSet, NodeProperty::SetFilterFilter]
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
    kind!(NodeKind::SetUnwrap, [NodeProperty::SetUnwrapSet]),
    kind!(
        NodeKind::Multiply,
        [NodeProperty::MultiplyFirst, NodeProperty::MultiplySecond]
    ),
    kind!(NodeKind::IsAtom, [NodeProperty::IsAtom]),
    kind!(
        NodeKind::SetEvery,
        [NodeProperty::SetEverySet, NodeProperty::SetEveryPredicate]
    ),
    kind!(
        NodeKind::SetAny,
        [NodeProperty::SetAnySet, NodeProperty::SetAnyPredicate]
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
        NodeKind::SetContains,
        [
            NodeProperty::SetContainsElement,
            NodeProperty::SetContainsSet
        ]
    ),
    kind!(NodeKind::IsConstruction, [NodeProperty::IsConstruction]),
    kind!(NodeKind::IsComposite, [NodeProperty::IsComposite]),
    kind!(NodeKind::IsNode, [NodeProperty::IsNode]),
    kind!(
        NodeKind::SetFold,
        [
            NodeProperty::SetFoldSet,
            NodeProperty::SetFoldFolder,
            NodeProperty::SetFoldInitial,
        ]
    ),
    kind!(
        NodeKind::NewConstruction,
        [
            NodeProperty::NewConstructionConstructor,
            NodeProperty::NewConstructionValue,
        ]
    ),
    kind!(NodeKind::NewSet, [NodeProperty::NewSetItem]),
    kind!(NodeKind::NewComposite, [NodeProperty::NewCompositeProperty]),
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
            let mut values = knowledge
                .query_construction_values(object, property.atom().into())
                .into_iter()
                .flatten();

            let Some(value) = values.next() else {
                error.get_or_insert(InvalidNodeError::MissingProperty {
                    kind: spec.kind,
                    property: *property,
                });

                continue;
            };

            present += 1;

            if !property.allows_multiple() && values.next().is_some() {
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
