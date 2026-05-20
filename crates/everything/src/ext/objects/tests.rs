use everything_structures::{Abstract, Object, Property, Structure};

use crate::{
    base::BASE,
    ext::{ObjectExt, StructureExt},
    nodes::{BinaryNode, CallNode, IfNode, Node},
};

#[test]
fn new_integer() {
    assert_eq!(Object::new_integer(0), Object::Abstract(Abstract::ZERO));

    assert_eq!(
        Object::new_integer(2),
        Structure::new(&mut [Property::new_successor_of(
            Structure::new(&mut [Property::new_successor_of(Abstract::ZERO.into())]).into()
        )])
        .into()
    );

    assert_eq!(
        Object::new_integer(-3),
        Structure::new(&mut [Property::new_predecessor_of(
            Structure::new(&mut [Property::new_predecessor_of(
                Structure::new(&mut [Property::new_predecessor_of(Abstract::ZERO.into())]).into()
            )])
            .into()
        )])
        .into()
    );
}

#[test]
fn node_type() {
    let knowledge = &BASE;

    // None
    assert_eq!(Object::Structure(Structure::Empty).node(knowledge), None);

    // Single
    assert_eq!(
        Object::Structure(Structure::new_node(Node::Function(Object::Abstract(
            Abstract::ZERO
        ))))
        .node(knowledge),
        Some(Node::Function(Abstract::ZERO.into()))
    );

    // TODO: more
}

#[test]
fn call() {
    let f: Object = Structure::new_node(Node::Function(
        Structure::new_node(Node::Parameter(0)).into(),
    ))
    .into();

    assert_eq!(
        f.call(
            &BASE,
            &[Object::Abstract(Abstract::ZERO).into()],
            &mut Default::default()
        )
        .into_object(),
        Object::Abstract(Abstract::ZERO)
    );
}

mod eval {
    use everything_structures::{Abstract, Object, Property, Structure};

    use crate::{
        ObjectOrSetValues,
        base::BASE,
        ext::{AbstractExt, ObjectExt, PropertyExt, StructureExt},
        nodes::{BinaryNode, Node},
    };

    #[test]
    fn and() {
        const CASES: &[(bool, bool, bool)] = &[
            (false, false, false),
            (false, true, false),
            (true, false, false),
            (true, true, true),
        ];

        for (left, right, result) in CASES.iter().copied() {
            assert_eq!(
                Object::Structure(Structure::new_node(Node::And(BinaryNode {
                    left: Structure::new_bool(left).into(),
                    right: Structure::new_bool(right).into()
                })))
                .evaluate(&BASE, &mut Default::default())
                .is_truthy(&BASE),
                result
            );
        }
    }

    #[test]
    fn or() {
        const CASES: &[(bool, bool, bool)] = &[
            (false, false, false),
            (false, true, true),
            (true, false, true),
            (true, true, true),
        ];

        for (left, right, result) in CASES.iter().copied() {
            assert_eq!(
                Object::Structure(Structure::new_node(Node::Or(BinaryNode {
                    left: Structure::new_bool(left).into(),
                    right: Structure::new_bool(right).into()
                })))
                .evaluate(&BASE, &mut Default::default())
                .is_truthy(&BASE),
                result
            );
        }
    }

    #[test]
    fn literal() {
        let subjects: [Object; 2] = [
            Abstract::ZERO.into(),
            Structure::new_node(Node::Not(Object::new_integer(42))).into(),
        ];

        for subject in subjects {
            assert_eq!(
                Object::Structure(Structure::new_node(Node::Literal(subject.clone())))
                    .evaluate(&BASE, &mut Default::default())
                    .into_object(),
                subject
            );
        }
    }

    #[test]
    fn eval_count() {
        assert_eq!(
            Object::Structure(Structure::new_node(Node::Count(Structure::Empty.into())))
                .evaluate(&BASE, &mut Default::default())
                .into_object(),
            Object::new_integer(0)
        );

        assert_eq!(
            Object::Structure(Structure::new_node(Node::Count(
                Structure::new_node(Node::Literal(
                    Structure::new(&mut [
                        Property::new_contains(Abstract::ZERO.into()),
                        Property::new_contains(Abstract::KNOWLEDGE.into()),
                    ])
                    .into()
                ))
                .into()
            )))
            .evaluate(&BASE, &mut Default::default())
            .into_object(),
            Object::new_integer(2)
        );
    }

    #[test]
    fn eval_query() {
        assert_eq!(
            Object::Structure(Structure::new_node_query_values(
                Structure::new(&mut [
                    Property::new_contains(Abstract::ZERO.into()),
                    Property::new_contains(Abstract::KNOWLEDGE.into()),
                    Property::new_successor_of(Object::new_integer(0)),
                ])
                .into(),
                Structure::new_node(Node::Literal(Abstract::CONTAINS.into())).into()
            ))
            .evaluate(&BASE, &mut Default::default())
            .into_object(),
            Structure::new_set([Abstract::KNOWLEDGE.into(), Object::Abstract(Abstract::ZERO)])
                .into(),
        );
    }

    #[test]
    fn set_items() {
        let f: Object = Structure::new_node(Node::Function(
            Structure::new_node(Node::Function(
                Structure::new_set([
                    Structure::new_node(Node::Parameter(0)).into(),
                    Structure::new_node(Node::Parameter(1)).into(),
                ])
                .into(),
            ))
            .into(),
        ))
        .into();

        assert_eq!(
            f.call(
                &BASE,
                &[
                    Object::Abstract(Abstract(1337)),
                    Object::Abstract(Abstract(1338))
                ]
                .map(ObjectOrSetValues::Object),
                &mut Default::default(),
            )
            .into_object(),
            Structure::new_set([
                Object::Abstract(Abstract(1337)),
                Object::Abstract(Abstract(1338))
            ])
            .into()
        );
    }

    #[test]
    fn count() {
        for count in 0..10_usize {
            let mut properties = (0..count)
                .map(|i| Property::new_contains(Object::new_integer(i as i128)))
                .collect::<Vec<_>>();

            let node = Object::Structure(Structure::new_node(Node::Count(
                Structure::new_node(Node::Literal(Structure::new(&mut properties).into())).into(),
            )));

            assert_eq!(
                node.evaluate(&BASE, &mut Default::default())
                    .into_object()
                    .to_integer(&BASE),
                Some(count as i128)
            );
        }

        // TODO: Test real count (no duplicates)
    }

    #[test]
    fn multiply() {
        let a = 543895;
        let b = 9345125;

        let node = Object::Structure(Structure::new_node(Node::Multiply(BinaryNode {
            left: Structure::new_node(Node::Literal(Object::new_integer(a))).into(),
            right: Structure::new_node(Node::Literal(Object::new_integer(b))).into(),
        })));

        assert_eq!(
            node.evaluate(&BASE, &mut Default::default())
                .into_object()
                .to_integer(&BASE),
            Some(a * b)
        );
    }
}

#[test]
fn factorial() {
    let factorial = Object::Structure(Structure::new_node(Node::Function(
        Structure::new_node(Node::If(IfNode {
            condition: Structure::new_node(Node::Less(BinaryNode {
                left: Structure::new_node(Node::Parameter(0)).into(),
                right: Object::new_integer(2),
            }))
            .into(),
            then: Object::new_integer(1),
            otherwise: Structure::new_node(Node::Multiply(BinaryNode {
                left: Structure::new_node(Node::Parameter(0)).into(),
                right: Structure::new_node(Node::Call(CallNode {
                    callee: Structure::new_node(Node::FunctionSelf(0)).into(),
                    with: Structure::new_node(Node::Add(BinaryNode {
                        left: Structure::new_node(Node::Parameter(0)).into(),
                        right: Object::new_integer(-1),
                    }))
                    .into(),
                }))
                .into(),
            }))
            .into(),
        }))
        .into(),
    )));

    let points = [
        (-10_i128, 1_i128),
        (-5, 1),
        (0, 1),
        (1, 1),
        (2, 2),
        (3, 6),
        (4, 24),
    ];

    for (input, output) in points {
        assert_eq!(
            factorial
                .call(
                    &BASE,
                    &[Object::new_integer(input).into()],
                    &mut Default::default()
                )
                .into_object(),
            Object::new_integer(output)
        );
    }
}
