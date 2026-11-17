use std::{assert_matches, ops::RangeInclusive};

use num_bigint::BigInt;

use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Construction, Set},
};

use super::*;

fn product_iterator<A: Clone, B>(
    a: impl Iterator<Item = A>,
    b: impl Iterator<Item = B> + Clone,
) -> impl Iterator<Item = (A, B)> {
    a.flat_map(move |a_item| b.clone().map(move |b_item| (a_item.clone(), b_item)))
}

#[test]
fn less() {
    const RANGE: RangeInclusive<i128> = -10..=20;

    for (a, b) in product_iterator(RANGE, RANGE) {
        assert_eq!(
            evaluate(
                &Node::Less {
                    first: Node::Literal(BigInt::from(a).into()).into(),
                    second: Node::Literal(BigInt::from(b).into()).into(),
                },
                &BASE,
            )
            .into_object(),
            (a < b).into()
        );
    }
}

#[test]
fn and() {
    let cases = [
        Object::Set(Set::empty()),
        Atom::TRUE.into(),
        Atom::FALSE.into(),
    ];

    for (first, second) in product_iterator(cases.iter().cloned(), cases.iter().cloned()) {
        assert_eq!(
            evaluate(
                &Node::And {
                    first: Node::Literal(first.clone()).into(),
                    second: Node::Literal(second.clone()).into()
                },
                &BASE,
            )
            .into_object(),
            match first {
                Object::Atom(Atom::TRUE) => {
                    MaybeUndefined::from(second.truthy()).into()
                }
                Object::Atom(Atom::FALSE) => Atom::FALSE.into(),
                _ => Atom::UNDEFINED.into(),
            },
            "case {first:?} {second:?}"
        );
    }
}

#[test]
fn add() {
    const RANGE: RangeInclusive<i128> = -10..=20;

    for (a, b) in product_iterator(RANGE, RANGE) {
        assert_eq!(
            evaluate(
                &Node::Add {
                    first: Node::Literal(BigInt::from(a).into()).into(),
                    second: Node::Literal(BigInt::from(b).into()).into(),
                },
                &BASE,
            )
            .into_object(),
            BigInt::from(a + b).into()
        );
    }
}

#[test]
fn unwrap() {
    let cases = [
        (Object::Set(Set::from([])), Object::Atom(Atom::UNDEFINED)),
        (
            Object::Set(Set::from([Object::Atom(Atom(42))])),
            Object::Atom(Atom(42)),
        ),
        (
            Object::Set(Set::from([Object::Atom(Atom(42)), Object::Atom(Atom(56))])),
            Object::Atom(Atom::UNDEFINED),
        ),
    ];

    for (source, unwrapped) in cases {
        assert_matches!(
            evaluate(
                &Node::Unwrap(Node::Literal(source).into()),
                &BASE
            ),
            LazyObject::Eager(object) if object == unwrapped
        );
    }
}

#[test]
fn values_of() {
    let atom = Atom(42);
    let value = Object::new_integer(BigInt::from(1));

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: value.clone(),
    });

    let set = Set::from([Object::Atom(atom)]);

    assert_eq!(
        evaluate(
            &Node::ValuesOf(Node::Literal(set.clone().into()).into()),
            &knowledge,
        )
        .into_object(),
        Object::Set(set)
    );

    assert_matches!(
        evaluate(
            &Node::ValuesOf(identity_function().into()),
            &knowledge
        ),
        LazyObject::Node(node) if node == identity_function()
    );

    assert_eq!(
        evaluate(
            &Node::ValuesOf(Node::Literal(Object::Atom(atom)).into()),
            &knowledge,
        )
        .into_object(),
        Object::Set(Set::from([value.clone()]))
    );

    assert_matches!(
        evaluate(
            &Node::ValuesOf(Node::Literal(Object::Atom(atom)).into()),
            &knowledge,
        ),
        LazyObject::AtomValues(values) if values.size() == 1
    );

    assert_eq!(
        evaluate(
            &Node::Count(Node::ValuesOf(Node::Literal(Object::Atom(atom)).into()).into()),
            &knowledge,
        )
        .into_object(),
        Object::new_integer(BigInt::from(1))
    );

    assert_eq!(
        evaluate(
            &Node::ValuesOf(Node::Literal(Object::Atom(atom)).into()),
            &knowledge,
        )
        .iter(&knowledge)
        .collect::<Vec<_>>(),
        vec![value]
    );

    let construction = Construction::new(Atom(1).into(), Atom(2).into());

    assert_eq!(
        evaluate(
            &Node::ValuesOf(Node::Literal(construction.into()).into()),
            &knowledge,
        )
        .into_object(),
        Atom::UNDEFINED.into()
    );

    assert_eq!(
        evaluate(
            &Node::ValuesOf(Node::Literal(Atom::UNDEFINED.into()).into()),
            &knowledge
        )
        .into_object(),
        Object::Set(Set::empty())
    );

    let undefined_value = Object::new_integer(BigInt::from(2));

    knowledge.add_statement(Statement {
        atom: Atom::UNDEFINED,
        object: undefined_value.clone(),
    });

    assert_eq!(
        evaluate(
            &Node::ValuesOf(Node::Literal(Atom::UNDEFINED.into()).into()),
            &knowledge
        )
        .into_object(),
        Object::Set(Set::from([undefined_value]))
    );
}

#[test]
fn has_value() {
    let atom = Atom(42);
    let value = Object::new_integer(BigInt::from(1));
    let empty_set = Object::Set(Set::empty());
    let non_empty_set = Object::Set(Set::from([value.clone()]));
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));

    let mut knowledge = BASE.clone();

    let has_value = |object: Object, knowledge: &Knowledge| {
        evaluate(&Node::HasValue(Node::Literal(object).into()), knowledge).into_object()
    };

    assert_eq!(
        has_value(Object::Atom(atom), &knowledge),
        Atom::FALSE.into()
    );
    assert_eq!(has_value(empty_set.clone(), &knowledge), Atom::FALSE.into());
    assert_eq!(
        has_value(non_empty_set.clone(), &knowledge),
        Atom::TRUE.into()
    );
    assert_eq!(has_value(construction, &knowledge), Atom::UNDEFINED.into());
    assert_eq!(
        has_value(Atom::UNDEFINED.into(), &knowledge),
        Atom::UNDEFINED.into()
    );

    assert_eq!(
        evaluate(&Node::HasValue(identity_function().into()), &knowledge).into_object(),
        Atom::TRUE.into()
    );

    knowledge.add_statement(Statement {
        atom,
        object: value,
    });

    assert_eq!(has_value(Object::Atom(atom), &knowledge), Atom::TRUE.into());

    let union = Node::Union {
        first: Node::Literal(empty_set).into(),
        second: Node::Literal(non_empty_set).into(),
    };

    assert_eq!(
        evaluate(&Node::HasValue(union.into()), &knowledge).into_object(),
        Atom::TRUE.into()
    );
}

#[test]
fn query_atoms_with_property() {
    let property = Atom(1);
    let matching = [Atom(42), Atom(43)];
    let other = Atom(44);

    let mut knowledge = BASE.clone();

    for atom in matching {
        knowledge.add_statement(Statement {
            atom,
            object: property.into(),
        });
    }

    knowledge.add_statement(Statement {
        atom: other,
        object: Atom(2).into(),
    });

    let result = evaluate(
        &Node::QueryAtomsWithProperty {
            property: Node::Literal(property.into()).into(),
        },
        &knowledge,
    );

    assert!(result.is_set());
    assert_eq!(result.size(&knowledge), 2);
    assert!(result.has(&matching[0].into(), &knowledge));
    assert!(result.has(&matching[1].into(), &knowledge));
    assert!(!result.has(&other.into(), &knowledge));
    assert_eq!(
        result.iter(&knowledge).collect::<Vec<_>>(),
        vec![matching[0].into(), matching[1].into()]
    );
}

#[test]
fn query_atoms_with_deferred_property_is_undefined() {
    let empty_set = Node::Literal(Object::Set(Set::empty()));

    assert_eq!(
        evaluate(
            &Node::QueryAtomsWithProperty {
                property: Node::Union {
                    first: empty_set.clone().into(),
                    second: empty_set.into(),
                }
                .into(),
            },
            &BASE,
        )
        .into_object(),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn query_construction_values() {
    let atom = Atom(42);
    let property_constructor = Atom(1);
    let other_constructor = Atom(2);
    let matching_values = [
        Object::new_integer(BigInt::from(1)),
        Object::new_integer(BigInt::from(2)),
    ];

    let mut knowledge = BASE.clone();

    for value in &matching_values {
        knowledge.add_statement(Statement {
            atom,
            object: Construction::new(property_constructor.into(), value.clone()).into(),
        });
    }

    knowledge.add_statement(Statement {
        atom,
        object: Construction::new(
            other_constructor.into(),
            Object::new_integer(BigInt::from(3)),
        )
        .into(),
    });
    knowledge.add_statement(Statement {
        atom,
        object: Object::new_integer(BigInt::from(4)),
    });

    let result = evaluate(
        &Node::QueryConstructionValues {
            set: Node::ValuesOf(Node::Literal(Object::Atom(atom)).into()).into(),
            property_constructor: Node::Literal(property_constructor.into()).into(),
        },
        &knowledge,
    );

    assert!(result.is_set());
    assert_eq!(result.size(&knowledge), 2);
    assert!(result.has(&matching_values[0], &knowledge));
    assert!(result.has(&matching_values[1], &knowledge));
    assert!(!result.has(&Object::new_integer(BigInt::from(3)), &knowledge));
    assert!(!result.has(&Object::new_integer(BigInt::from(4)), &knowledge));
    assert_eq!(
        result.iter(&knowledge).collect::<Vec<_>>(),
        matching_values.to_vec()
    );
}

#[test]
fn query_construction_values_of_literal_set() {
    let constructor = Atom(1);

    let set = Set::from([
        Construction::new(constructor.into(), Object::new_integer(BigInt::from(1))).into(),
        Construction::new(Atom(2).into(), Object::new_integer(BigInt::from(2))).into(),
        Construction::new(constructor.into(), Object::new_integer(BigInt::from(3))).into(),
    ]);

    let result = evaluate(
        &Node::QueryConstructionValues {
            set: Node::Literal(set.into()).into(),
            property_constructor: Node::Literal(constructor.into()).into(),
        },
        &BASE,
    );

    assert_eq!(
        result.iter(&BASE).collect::<Vec<_>>(),
        vec![
            Object::new_integer(BigInt::from(1)),
            Object::new_integer(BigInt::from(3)),
        ]
    );
}

#[test]
fn query_construction_values_requires_a_set() {
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));
    let property_constructor = Node::Literal(Atom(1).into());

    let evaluate_query = |set: Node, property_constructor: Node| {
        evaluate(
            &Node::QueryConstructionValues {
                set: set.into(),
                property_constructor: property_constructor.into(),
            },
            &BASE,
        )
        .into_object()
    };

    assert_eq!(
        evaluate_query(
            Node::Literal(construction.clone()),
            property_constructor.clone()
        ),
        Atom::UNDEFINED.into()
    );
    assert_eq!(
        evaluate_query(
            Node::Literal(Object::Atom(Atom(42))),
            property_constructor.clone()
        ),
        Atom::UNDEFINED.into()
    );
    assert_eq!(
        evaluate_query(Node::Literal(Atom::UNDEFINED.into()), property_constructor),
        Atom::UNDEFINED.into()
    );

    // A deferred set is a set, but cannot be used as a constructor.
    let empty_set = Node::Literal(Object::Set(Set::empty()));
    let deferred_set = Node::Union {
        first: empty_set.clone().into(),
        second: empty_set.clone().into(),
    };

    assert_eq!(
        evaluate_query(
            Node::Literal(Object::Set(Set::empty())),
            deferred_set.clone()
        ),
        Atom::UNDEFINED.into()
    );

    // A deferred set target yields a deferred result.
    let result = evaluate(
        &Node::QueryConstructionValues {
            set: deferred_set.into(),
            property_constructor: Node::Literal(Atom(1).into()).into(),
        },
        &BASE,
    );

    assert!(result.is_set());
    assert_eq!(result.size(&BASE), 0);
    assert_eq!(result.iter(&BASE).count(), 0);
}

#[test]
fn count_requires_sets() {
    let atom = Atom(42);
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));
    let set = Object::Set(Set::from([Object::new_integer(BigInt::from(1))]));

    let count =
        |object: Object| evaluate(&Node::Count(Node::Literal(object).into()), &BASE).into_object();

    assert_eq!(count(Object::Atom(atom)), Atom::UNDEFINED.into());
    assert_eq!(count(construction), Atom::UNDEFINED.into());
    assert_eq!(count(Atom::UNDEFINED.into()), Atom::UNDEFINED.into());
    assert_eq!(count(set), Object::new_integer(BigInt::from(1)));

    // A function value is a set: its node representation has one property.
    assert_eq!(
        evaluate(&Node::Count(identity_function().into()), &BASE).into_object(),
        Object::new_integer(BigInt::from(1))
    );
}

#[test]
fn unwrap_requires_sets() {
    let atom = Atom(42);
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));

    let unwrap = |node: Node| evaluate(&Node::Unwrap(node.into()), &BASE).into_object();

    assert_eq!(
        unwrap(Node::Literal(Object::Atom(atom))),
        Atom::UNDEFINED.into()
    );
    assert_eq!(unwrap(Node::Literal(construction)), Atom::UNDEFINED.into());

    // A function value is a set of its node properties.
    assert_eq!(
        unwrap(identity_function()),
        Construction::new(
            Atom::NODE_FUNCTION.into(),
            Node::Parameter { depth: 0 }.into_object()
        )
        .into()
    );
}

#[test]
fn function_values_are_sets() {
    assert_eq!(
        evaluate(&Node::IsSet(identity_function().into()), &BASE).into_object(),
        Atom::TRUE.into()
    );
    assert_eq!(
        evaluate(
            &Node::IsSet(Node::Literal(Object::Atom(Atom(42))).into()),
            &BASE
        )
        .into_object(),
        Atom::FALSE.into()
    );
    assert_eq!(
        evaluate(
            &Node::IsSet(
                Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()).into()
            ),
            &BASE
        )
        .into_object(),
        Atom::FALSE.into()
    );
}

#[test]
fn map_and_filter_require_sets() {
    let atom = Node::Literal(Object::Atom(Atom(42)));
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());
    let identity = identity_function();

    assert_eq!(
        evaluate(
            &Node::Map {
                set: atom.clone().into(),
                mapper_function: identity.clone().into(),
            },
            &BASE,
        )
        .into_object(),
        Atom::UNDEFINED.into()
    );
    assert_eq!(
        evaluate(
            &Node::Filter {
                set: atom.into(),
                filter_function: identity.clone().into(),
            },
            &BASE,
        )
        .into_object(),
        Atom::UNDEFINED.into()
    );
    assert_eq!(
        evaluate(
            &Node::Map {
                set: construction.clone().into(),
                mapper_function: identity.clone().into(),
            },
            &BASE,
        )
        .into_object(),
        Atom::UNDEFINED.into()
    );
    assert_eq!(
        evaluate(
            &Node::Filter {
                set: construction.into(),
                filter_function: identity.into(),
            },
            &BASE,
        )
        .into_object(),
        Atom::UNDEFINED.into()
    );

    // A function value is a set, so mapping over it yields its node property.
    let values = evaluate(
        &Node::Map {
            set: identity_function().into(),
            mapper_function: identity_function().into(),
        },
        &BASE,
    )
    .iter(&BASE)
    .collect::<Vec<_>>();

    assert_eq!(
        values,
        vec![
            Construction::new(
                Atom::NODE_FUNCTION.into(),
                Node::Parameter { depth: 0 }.into_object()
            )
            .into()
        ]
    );
}

#[test]
fn union_requires_sets() {
    let atom = Node::Literal(Object::Atom(Atom(42)));
    let empty_set = Node::Literal(Object::Set(Set::empty()));
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());

    for invalid in [atom, construction] {
        assert_eq!(
            evaluate(
                &Node::Union {
                    first: invalid.into(),
                    second: empty_set.clone().into(),
                },
                &BASE,
            )
            .into_object(),
            Atom::UNDEFINED.into()
        );
    }

    let union = evaluate(
        &Node::Union {
            first: identity_function().into(),
            second: empty_set.into(),
        },
        &BASE,
    );

    assert!(union.is_set());
    assert_eq!(union.iter(&BASE).count(), 1);
}

#[test]
fn sets_are_rejected_by_scalar_operations() {
    let union = Node::Union {
        first: Node::Literal(Object::Set(Set::empty())).into(),
        second: Node::Literal(Object::Set(Set::empty())).into(),
    };

    assert_eq!(
        evaluate(
            &Node::Add {
                first: union.clone().into(),
                second: Node::Literal(BigInt::from(1).into()).into(),
            },
            &BASE,
        )
        .into_object(),
        Atom::UNDEFINED.into()
    );

    assert_eq!(
        evaluate(
            &Node::Add {
                first: identity_function().into(),
                second: Node::Literal(BigInt::from(1).into()).into(),
            },
            &BASE,
        )
        .into_object(),
        Atom::UNDEFINED.into()
    );

    assert_eq!(
        evaluate(
            &Node::Call {
                callee: union.into(),
                with: Node::Literal(BigInt::from(1).into()).into(),
            },
            &BASE,
        )
        .into_object(),
        Atom::UNDEFINED.into()
    );
}

fn integer_set() -> Set {
    Set::from([
        Object::new_integer(BigInt::from(1)),
        Object::new_integer(BigInt::from(2)),
        Object::new_integer(BigInt::from(3)),
    ])
}

fn identity_function() -> Node {
    Node::Function {
        body: Node::Parameter { depth: 0 }.into(),
    }
}

#[test]
fn filter() {
    let node = Node::Filter {
        set: Node::Literal(integer_set().into()).into(),
        filter_function: Node::Function {
            body: Node::Equal {
                first: Node::Parameter { depth: 0 }.into(),
                second: Node::Literal(Object::new_integer(BigInt::from(2))).into(),
            }
            .into(),
        }
        .into(),
    };

    let values = evaluate(&node, &BASE).iter(&BASE).collect::<Vec<_>>();

    assert_eq!(values, vec![Object::new_integer(BigInt::from(2))]);
}

#[test]
fn map() {
    let node = Node::Map {
        set: Node::Literal(integer_set().into()).into(),
        mapper_function: identity_function().into(),
    };

    let values = evaluate(&node, &BASE).iter(&BASE).collect::<Vec<_>>();

    assert_eq!(
        values,
        [
            Object::new_integer(BigInt::from(1)),
            Object::new_integer(BigInt::from(2)),
            Object::new_integer(BigInt::from(3)),
        ]
    );
}
