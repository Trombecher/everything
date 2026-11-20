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
            .into_object(&BASE),
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
            .into_object(&BASE),
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
            .into_object(&BASE),
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
        .into_object(&knowledge),
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
        .into_object(&knowledge),
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
        .into_object(&knowledge),
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
        .into_object(&knowledge),
        Atom::UNDEFINED.into()
    );

    assert_eq!(
        evaluate(
            &Node::ValuesOf(Node::Literal(Atom::UNDEFINED.into()).into()),
            &knowledge
        )
        .into_object(&knowledge),
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
        .into_object(&knowledge),
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
        evaluate(&Node::HasValue(Node::Literal(object).into()), knowledge).into_object(knowledge)
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
        evaluate(&Node::HasValue(identity_function().into()), &knowledge).into_object(&knowledge),
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
        evaluate(&Node::HasValue(union.into()), &knowledge).into_object(&knowledge),
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
        .into_object(&BASE),
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
            object: Node::ValuesOf(Node::Literal(Object::Atom(atom)).into()).into(),
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

    // An atom can be queried directly without materialising its values.
    let direct = evaluate(
        &Node::QueryConstructionValues {
            object: Node::Literal(Object::Atom(atom)).into(),
            property_constructor: Node::Literal(property_constructor.into()).into(),
        },
        &knowledge,
    );

    assert_eq!(
        direct.iter(&knowledge).collect::<Vec<_>>(),
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
            object: Node::Literal(set.into()).into(),
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
fn query_construction_values_resolves_objects_through_the_knowledge() {
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));
    let property_constructor = Node::Literal(Atom(1).into());

    let evaluate_query = |object: Node, property_constructor: Node| {
        evaluate(
            &Node::QueryConstructionValues {
                object: object.into(),
                property_constructor: property_constructor.into(),
            },
            &BASE,
        )
    };

    // Constructions have no values and cannot be queried.
    assert_eq!(
        evaluate_query(Node::Literal(construction), property_constructor.clone())
            .into_object(&BASE),
        Atom::UNDEFINED.into()
    );

    // Atoms are resolved through the knowledge. Atoms without values yield an
    // empty deferred set.
    let atom_result = evaluate_query(
        Node::Literal(Object::Atom(Atom(42))),
        property_constructor.clone(),
    );
    assert!(atom_result.is_set());
    assert_eq!(atom_result.size(&BASE), 0);
    assert_eq!(atom_result.iter(&BASE).count(), 0);

    // `undefined` is an atom and is queried like any other atom.
    let undefined_result = evaluate_query(
        Node::Literal(Atom::UNDEFINED.into()),
        property_constructor.clone(),
    );
    assert!(undefined_result.is_set());
    assert_eq!(undefined_result.size(&BASE), 0);
    assert_eq!(undefined_result.iter(&BASE).count(), 0);

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
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );

    // A deferred set target yields a deferred result.
    let result = evaluate_query(deferred_set, Node::Literal(Atom(1).into()));

    assert!(result.is_set());
    assert_eq!(result.size(&BASE), 0);
    assert_eq!(result.iter(&BASE).count(), 0);
}

#[test]
fn query_construction_values_queries_undefined_as_an_atom() {
    let constructor = Atom(1);
    let value = Object::new_integer(BigInt::from(7));

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom: Atom::UNDEFINED,
        object: Construction::new(constructor.into(), value.clone()).into(),
    });

    let result = evaluate(
        &Node::QueryConstructionValues {
            object: Node::Literal(Atom::UNDEFINED.into()).into(),
            property_constructor: Node::Literal(constructor.into()).into(),
        },
        &knowledge,
    );

    assert_eq!(result.iter(&knowledge).collect::<Vec<_>>(), vec![value]);
}

fn pair(atom: Atom, value: Object) -> Object {
    Set::from([
        Construction::new(Atom::ATOM.into(), atom.into()).into(),
        Construction::new(Atom::VALUE.into(), value).into(),
    ])
    .into()
}

#[test]
fn query_atoms_with_constructor() {
    let constructor = Atom(1);
    let other_constructor = Atom(2);
    let first_atom = Atom(42);
    let second_atom = Atom(43);
    let other_atom = Atom(44);

    let mut knowledge = BASE.clone();

    for value in [1, 2] {
        knowledge.add_statement(Statement {
            atom: first_atom,
            object: Construction::new(constructor.into(), Object::new_integer(BigInt::from(value)))
                .into(),
        });
    }

    knowledge.add_statement(Statement {
        atom: second_atom,
        object: Construction::new(constructor.into(), Object::new_integer(BigInt::from(3))).into(),
    });
    knowledge.add_statement(Statement {
        atom: other_atom,
        object: Construction::new(
            other_constructor.into(),
            Object::new_integer(BigInt::from(4)),
        )
        .into(),
    });

    let result = evaluate(
        &Node::QueryAtomsWithConstructor {
            constructor: Node::Literal(constructor.into()).into(),
        },
        &knowledge,
    );

    assert!(result.is_set());
    assert_eq!(result.size(&knowledge), 2);
    assert!(result.has(&first_atom.into(), &knowledge));
    assert!(result.has(&second_atom.into(), &knowledge));
    assert!(!result.has(&other_atom.into(), &knowledge));
    assert_eq!(
        result.iter(&knowledge).collect::<Vec<_>>(),
        vec![first_atom.into(), second_atom.into()]
    );
}

#[test]
fn query_atoms_with_constructor_materializes_deferred_constructor() {
    let atom = Atom(42);
    let constructor = Object::Set(Set::from([Atom(1).into(), Atom(2).into()]));

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: Construction::new(constructor, Object::new_integer(BigInt::from(7))).into(),
    });

    let result = evaluate(
        &Node::QueryAtomsWithConstructor {
            constructor: Node::Union {
                first: Node::Literal(Object::Set(Set::from([Atom(1).into()]))).into(),
                second: Node::Literal(Object::Set(Set::from([Atom(2).into()]))).into(),
            }
            .into(),
        },
        &knowledge,
    );

    assert_eq!(
        result.iter(&knowledge).collect::<Vec<_>>(),
        vec![atom.into()]
    );
}

#[test]
fn query_constructions() {
    let constructor = Atom(1);
    let other_constructor = Atom(2);
    let first_atom = Atom(42);
    let second_atom = Atom(43);

    let mut knowledge = BASE.clone();

    for value in [1, 2] {
        knowledge.add_statement(Statement {
            atom: first_atom,
            object: Construction::new(constructor.into(), Object::new_integer(BigInt::from(value)))
                .into(),
        });
    }

    knowledge.add_statement(Statement {
        atom: first_atom,
        object: Construction::new(
            other_constructor.into(),
            Object::new_integer(BigInt::from(3)),
        )
        .into(),
    });
    knowledge.add_statement(Statement {
        atom: second_atom,
        object: Construction::new(constructor.into(), Object::new_integer(BigInt::from(4))).into(),
    });

    let result = evaluate(
        &Node::QueryConstructions {
            constructor: Node::Literal(constructor.into()).into(),
        },
        &knowledge,
    );

    let expected = vec![
        pair(first_atom, Object::new_integer(BigInt::from(1))),
        pair(first_atom, Object::new_integer(BigInt::from(2))),
        pair(second_atom, Object::new_integer(BigInt::from(4))),
    ];

    assert!(result.is_set());
    assert_eq!(result.size(&knowledge), 3);
    assert!(result.has(&expected[0], &knowledge));
    assert!(result.has(&expected[1], &knowledge));
    assert!(result.has(&expected[2], &knowledge));
    assert!(!result.has(
        &pair(first_atom, Object::new_integer(BigInt::from(3))),
        &knowledge
    ));
    assert_eq!(result.iter(&knowledge).collect::<Vec<_>>(), expected);
}

#[test]
fn query_constructions_materializes_deferred_constructor() {
    let atom = Atom(42);
    let value = Object::new_integer(BigInt::from(7));
    let constructor = Object::Set(Set::from([Atom(1).into(), Atom(2).into()]));

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: Construction::new(constructor, value.clone()).into(),
    });

    let result = evaluate(
        &Node::QueryConstructions {
            constructor: Node::Union {
                first: Node::Literal(Object::Set(Set::from([Atom(1).into()]))).into(),
                second: Node::Literal(Object::Set(Set::from([Atom(2).into()]))).into(),
            }
            .into(),
        },
        &knowledge,
    );

    assert_eq!(
        result.iter(&knowledge).collect::<Vec<_>>(),
        vec![pair(atom, value)]
    );
}

#[test]
fn query_constructions_queries_undefined_as_an_atom() {
    let constructor = Atom(1);
    let value = Object::new_integer(BigInt::from(7));

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom: Atom::UNDEFINED,
        object: Construction::new(constructor.into(), value.clone()).into(),
    });

    let result = evaluate(
        &Node::QueryConstructions {
            constructor: Node::Literal(constructor.into()).into(),
        },
        &knowledge,
    );

    assert_eq!(
        result.iter(&knowledge).collect::<Vec<_>>(),
        vec![pair(Atom::UNDEFINED, value)]
    );
}

#[test]
fn count_requires_sets() {
    let atom = Atom(42);
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));
    let set = Object::Set(Set::from([Object::new_integer(BigInt::from(1))]));

    let count = |object: Object| {
        evaluate(&Node::Count(Node::Literal(object).into()), &BASE).into_object(&BASE)
    };

    assert_eq!(count(Object::Atom(atom)), Atom::UNDEFINED.into());
    assert_eq!(count(construction), Atom::UNDEFINED.into());
    assert_eq!(count(Atom::UNDEFINED.into()), Atom::UNDEFINED.into());
    assert_eq!(count(set), Object::new_integer(BigInt::from(1)));

    // A function value is a set: its node representation has one property.
    assert_eq!(
        evaluate(&Node::Count(identity_function().into()), &BASE).into_object(&BASE),
        Object::new_integer(BigInt::from(1))
    );
}

#[test]
fn unwrap_requires_sets() {
    let atom = Atom(42);
    let construction = Object::Construction(Construction::new(Atom(1).into(), Atom(2).into()));

    let unwrap = |node: Node| evaluate(&Node::Unwrap(node.into()), &BASE).into_object(&BASE);

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
        evaluate(&Node::IsSet(identity_function().into()), &BASE).into_object(&BASE),
        Atom::TRUE.into()
    );
    assert_eq!(
        evaluate(
            &Node::IsSet(Node::Literal(Object::Atom(Atom(42))).into()),
            &BASE
        )
        .into_object(&BASE),
        Atom::FALSE.into()
    );
    assert_eq!(
        evaluate(
            &Node::IsSet(
                Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()).into()
            ),
            &BASE
        )
        .into_object(&BASE),
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
        .into_object(&BASE),
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
        .into_object(&BASE),
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
        .into_object(&BASE),
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
        .into_object(&BASE),
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
            .into_object(&BASE),
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
        .into_object(&BASE),
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
        .into_object(&BASE),
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
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}

fn integer(value: i32) -> Object {
    Object::new_integer(BigInt::from(value))
}

fn integer_set() -> Set {
    Set::from([integer(1), integer(2), integer(3)])
}

/// The curried function `acc |-> element |-> acc + element`.
fn add_folder() -> Node {
    Node::Function {
        body: Node::Function {
            body: Node::Add {
                first: Node::Parameter { depth: 1 }.into(),
                second: Node::Parameter { depth: 0 }.into(),
            }
            .into(),
        }
        .into(),
    }
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

#[test]
fn curried_add_function() {
    let first_call = Node::Call {
        callee: add_folder().into(),
        with: Node::Literal(integer(0)).into(),
    };
    let second_call = Node::Call {
        callee: first_call.into(),
        with: Node::Literal(integer(5)).into(),
    };

    assert_eq!(evaluate(&second_call, &BASE).into_object(&BASE), integer(5));
}

#[test]
fn fold_sums_the_set() {
    let node = Node::Fold {
        set: Node::Literal(integer_set().into()).into(),
        folder: add_folder().into(),
        initial: Node::Literal(integer(0)).into(),
    };

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        integer(1 + 2 + 3)
    );
}

#[test]
fn fold_of_an_empty_set_returns_the_initial_value() {
    let initial = integer(42);

    let node = Node::Fold {
        set: Node::Literal(Object::Set(Set::empty())).into(),
        // The folder is never called, so it may even be undefined.
        folder: Node::Literal(Atom::UNDEFINED.into()).into(),
        initial: Node::Literal(initial.clone()).into(),
    };

    assert_eq!(evaluate(&node, &BASE).into_object(&BASE), initial);
}

#[test]
fn fold_of_an_empty_deferred_set_returns_the_initial_value() {
    let drop_everything = Node::Function {
        body: Node::Literal(Atom::FALSE.into()).into(),
    };
    let empty_set = Node::Filter {
        set: Node::Literal(integer_set().into()).into(),
        filter_function: drop_everything.into(),
    };
    let initial = integer(42);

    let node = Node::Fold {
        set: empty_set.into(),
        // The folder is never called, so it may even be undefined.
        folder: Node::Literal(Atom::UNDEFINED.into()).into(),
        initial: Node::Literal(initial.clone()).into(),
    };

    assert_eq!(evaluate(&node, &BASE).into_object(&BASE), initial);
}

#[test]
fn fold_requires_a_set() {
    let invalid_sets = [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::Literal(Atom::UNDEFINED.into()),
    ];

    for set in invalid_sets {
        assert_eq!(
            evaluate(
                &Node::Fold {
                    set: set.into(),
                    folder: add_folder().into(),
                    initial: Node::Literal(integer(0)).into(),
                },
                &BASE,
            )
            .into_object(&BASE),
            Atom::UNDEFINED.into()
        );
    }
}

#[test]
fn fold_requires_a_function_folder() {
    let empty_set = Node::Literal(Object::Set(Set::empty()));
    let invalid_folders = [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::Literal(Atom::UNDEFINED.into()),
        Node::Union {
            first: empty_set.clone().into(),
            second: empty_set.into(),
        },
    ];

    for folder in invalid_folders {
        assert_eq!(
            evaluate(
                &Node::Fold {
                    set: Node::Literal(integer_set().into()).into(),
                    folder: folder.into(),
                    initial: Node::Literal(integer(0)).into(),
                },
                &BASE,
            )
            .into_object(&BASE),
            Atom::UNDEFINED.into()
        );
    }
}

#[test]
fn fold_yields_undefined_when_the_folder_result_is_not_callable() {
    // The folder returns the accumulator, which is not a function.
    let folder = Node::Function {
        body: Node::Parameter { depth: 0 }.into(),
    };

    let node = Node::Fold {
        set: Node::Literal(integer_set().into()).into(),
        folder: folder.into(),
        initial: Node::Literal(integer(0)).into(),
    };

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn fold_yields_undefined_when_the_folder_returns_undefined() {
    let folder = Node::Function {
        body: Node::Literal(Atom::UNDEFINED.into()).into(),
    };

    let node = Node::Fold {
        set: Node::Literal(integer_set().into()).into(),
        folder: folder.into(),
        initial: Node::Literal(integer(0)).into(),
    };

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn fold_iterates_deferred_sets() {
    let doubled = Node::Map {
        set: Node::Literal(integer_set().into()).into(),
        mapper_function: Node::Function {
            body: Node::Add {
                first: Node::Parameter { depth: 0 }.into(),
                second: Node::Parameter { depth: 0 }.into(),
            }
            .into(),
        }
        .into(),
    };

    let node = Node::Fold {
        set: doubled.into(),
        folder: add_folder().into(),
        initial: Node::Literal(integer(0)).into(),
    };

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        integer(2 + 4 + 6)
    );
}

#[test]
fn fold_can_accumulate_deferred_sets() {
    let set = Set::from([
        Object::Set(Set::from([integer(1)])),
        Object::Set(Set::from([integer(2)])),
    ]);

    // The curried function `acc |-> element |-> acc | element`.
    let folder = Node::Function {
        body: Node::Function {
            body: Node::Union {
                first: Node::Parameter { depth: 1 }.into(),
                second: Node::Parameter { depth: 0 }.into(),
            }
            .into(),
        }
        .into(),
    };

    let node = Node::Fold {
        set: Node::Literal(set.into()).into(),
        folder: folder.into(),
        initial: Node::Literal(Object::Set(Set::empty())).into(),
    };

    let result = evaluate(&node, &BASE);

    assert!(result.is_set());
    assert_eq!(
        result.iter(&BASE).collect::<Vec<_>>(),
        vec![integer(1), integer(2)]
    );
}

#[test]
fn fold_captures_the_evaluation_context() {
    // The curried function
    // `acc |-> element |-> acc + element + offset`,
    // where `offset` comes from the surrounding function.
    let folder = Node::Function {
        body: Node::Function {
            body: Node::Add {
                first: Node::Add {
                    first: Node::Parameter { depth: 1 }.into(),
                    second: Node::Parameter { depth: 0 }.into(),
                }
                .into(),
                second: Node::Parameter { depth: 2 }.into(),
            }
            .into(),
        }
        .into(),
    };

    let outer_function = Node::Function {
        body: Node::Fold {
            set: Node::Literal(integer_set().into()).into(),
            folder: folder.into(),
            initial: Node::Literal(integer(0)).into(),
        }
        .into(),
    };

    let node = Node::Call {
        callee: outer_function.into(),
        with: Node::Literal(integer(10)).into(),
    };

    // (0 + 1 + 10) + (2 + 10) + (3 + 10)
    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        integer(1 + 2 + 3 + 3 * 10)
    );
}

fn constant_predicate(value: Object) -> Node {
    Node::Function {
        body: Node::Literal(value).into(),
    }
}

fn always_true_predicate() -> Node {
    constant_predicate(Atom::TRUE.into())
}

fn equals_two_predicate() -> Node {
    Node::Function {
        body: Node::Equal {
            first: Node::Parameter { depth: 0 }.into(),
            second: Node::Literal(integer(2)).into(),
        }
        .into(),
    }
}

/// Returns `false` for the first element and `undefined` for all others.
fn false_on_one_undefined_otherwise_predicate() -> Node {
    Node::Function {
        body: Node::If {
            condition: Node::Equal {
                first: Node::Parameter { depth: 0 }.into(),
                second: Node::Literal(integer(1)).into(),
            }
            .into(),
            then: Node::Literal(false.into()).into(),
            otherwise: Node::Literal(Atom::UNDEFINED.into()).into(),
        }
        .into(),
    }
}

/// Returns `undefined` for the first element and `false` for all others.
fn undefined_on_one_false_otherwise_predicate() -> Node {
    Node::Function {
        body: Node::If {
            condition: Node::Equal {
                first: Node::Parameter { depth: 0 }.into(),
                second: Node::Literal(integer(1)).into(),
            }
            .into(),
            then: Node::Literal(Atom::UNDEFINED.into()).into(),
            otherwise: Node::Literal(false.into()).into(),
        }
        .into(),
    }
}

fn every_result(set: Node, predicate: Node) -> Object {
    evaluate(
        &Node::Every {
            set: set.into(),
            predicate: predicate.into(),
        },
        &BASE,
    )
    .into_object(&BASE)
}

#[test]
fn every_of_an_empty_set_is_true() {
    let empty_set = Node::Literal(Object::Set(Set::empty()));
    let invalid_predicates = [
        Node::Literal(Atom::UNDEFINED.into()),
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::Union {
            first: empty_set.clone().into(),
            second: empty_set.clone().into(),
        },
    ];

    assert_eq!(
        every_result(empty_set.clone(), always_true_predicate()),
        Atom::TRUE.into()
    );

    for predicate in invalid_predicates {
        assert_eq!(
            every_result(empty_set.clone(), predicate),
            Atom::TRUE.into()
        );
    }
}

#[test]
fn every_of_an_empty_deferred_set_is_true() {
    let drop_everything = Node::Function {
        body: Node::Literal(Atom::FALSE.into()).into(),
    };
    let empty_set = Node::Filter {
        set: Node::Literal(integer_set().into()).into(),
        filter_function: drop_everything.into(),
    };

    assert_eq!(
        every_result(empty_set, Node::Literal(Atom::UNDEFINED.into())),
        Atom::TRUE.into()
    );
}

#[test]
fn every_requires_a_set() {
    let invalid_sets = [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::Literal(Atom::UNDEFINED.into()),
    ];

    for set in invalid_sets {
        assert_eq!(
            every_result(set, always_true_predicate()),
            Atom::UNDEFINED.into()
        );
    }
}

#[test]
fn every_requires_a_function_predicate() {
    let empty_set = Node::Literal(Object::Set(Set::empty()));
    let invalid_predicates = [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::Literal(Atom::UNDEFINED.into()),
        Node::Union {
            first: empty_set.clone().into(),
            second: empty_set.into(),
        },
    ];

    for predicate in invalid_predicates {
        assert_eq!(
            every_result(Node::Literal(integer_set().into()), predicate),
            Atom::UNDEFINED.into()
        );
    }
}

#[test]
fn every_returns_true_when_all_calls_are_true() {
    let set = Node::Literal(Object::Set(Set::from([integer(2)])));

    assert_eq!(every_result(set, equals_two_predicate()), Atom::TRUE.into());
    assert_eq!(
        every_result(Node::Literal(integer_set().into()), always_true_predicate()),
        Atom::TRUE.into()
    );
}

#[test]
fn every_returns_false_when_a_call_is_false() {
    assert_eq!(
        every_result(Node::Literal(integer_set().into()), equals_two_predicate()),
        Atom::FALSE.into()
    );
}

#[test]
fn every_returns_undefined_when_a_call_is_undefined() {
    assert_eq!(
        every_result(
            Node::Literal(integer_set().into()),
            constant_predicate(Atom::UNDEFINED.into()),
        ),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn every_returns_undefined_when_a_call_is_not_boolean() {
    let non_boolean_results = [
        integer(0),
        Object::Set(Set::empty()),
        Construction::new(Atom(1).into(), Atom(2).into()).into(),
        Object::new_node(identity_function()),
    ];

    for result in non_boolean_results {
        assert_eq!(
            every_result(
                Node::Literal(integer_set().into()),
                constant_predicate(result)
            ),
            Atom::UNDEFINED.into()
        );
    }
}

#[test]
fn every_short_circuits_on_false() {
    assert_eq!(
        every_result(
            Node::Literal(integer_set().into()),
            false_on_one_undefined_otherwise_predicate(),
        ),
        Atom::FALSE.into()
    );
}

#[test]
fn every_short_circuits_on_undefined() {
    assert_eq!(
        every_result(
            Node::Literal(integer_set().into()),
            undefined_on_one_false_otherwise_predicate(),
        ),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn every_iterates_deferred_sets() {
    let doubled = Node::Map {
        set: Node::Literal(integer_set().into()).into(),
        mapper_function: Node::Function {
            body: Node::Add {
                first: Node::Parameter { depth: 0 }.into(),
                second: Node::Parameter { depth: 0 }.into(),
            }
            .into(),
        }
        .into(),
    };
    let is_positive = Node::Function {
        body: Node::Less {
            first: Node::Literal(integer(0)).into(),
            second: Node::Parameter { depth: 0 }.into(),
        }
        .into(),
    };

    assert_eq!(every_result(doubled, is_positive), Atom::TRUE.into());
}

#[test]
fn every_captures_the_evaluation_context() {
    let outer_function = Node::Function {
        body: Node::Every {
            set: Node::Literal(integer_set().into()).into(),
            predicate: Node::Function {
                body: Node::Less {
                    first: Node::Parameter { depth: 0 }.into(),
                    second: Node::Parameter { depth: 1 }.into(),
                }
                .into(),
            }
            .into(),
        }
        .into(),
    };

    let call = |value| {
        evaluate(
            &Node::Call {
                callee: outer_function.clone().into(),
                with: Node::Literal(integer(value)).into(),
            },
            &BASE,
        )
        .into_object(&BASE)
    };

    assert_eq!(call(5), Atom::TRUE.into());
    assert_eq!(call(2), Atom::FALSE.into());
}

fn and_apply_folder(predicate: Object) -> Node {
    Node::Function {
        body: Node::Function {
            body: Node::And {
                first: Node::Parameter { depth: 1 }.into(),
                second: Node::Call {
                    callee: Node::Literal(predicate).into(),
                    with: Node::Parameter { depth: 0 }.into(),
                }
                .into(),
            }
            .into(),
        }
        .into(),
    }
}

fn every_by_and_fold(set: Node, predicate: &Node) -> Object {
    let predicate = evaluate(predicate, &BASE).into_object(&BASE);

    evaluate(
        &Node::Fold {
            set: set.into(),
            folder: and_apply_folder(predicate).into(),
            initial: Node::Literal(Atom::TRUE.into()).into(),
        },
        &BASE,
    )
    .into_object(&BASE)
}

#[test]
fn every_matches_and_folding() {
    let empty_set = Node::Literal(Object::Set(Set::empty()));
    let integer_set_node = || Node::Literal(integer_set().into());
    let cases = [
        (empty_set.clone(), always_true_predicate()),
        (empty_set.clone(), Node::Literal(Atom::UNDEFINED.into())),
        (integer_set_node(), always_true_predicate()),
        (integer_set_node(), equals_two_predicate()),
        (
            integer_set_node(),
            constant_predicate(Atom::UNDEFINED.into()),
        ),
        (integer_set_node(), constant_predicate(integer(0))),
        (
            integer_set_node(),
            false_on_one_undefined_otherwise_predicate(),
        ),
        (
            integer_set_node(),
            undefined_on_one_false_otherwise_predicate(),
        ),
    ];

    for (set, predicate) in cases {
        let every = evaluate(
            &Node::Every {
                set: set.clone().into(),
                predicate: predicate.clone().into(),
            },
            &BASE,
        )
        .into_object(&BASE);

        assert_eq!(every, every_by_and_fold(set, &predicate));
    }
}
