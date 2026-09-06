use everything_objects::{Abstract, Composite, Object, Property};

use crate::{
    base::BASE,
    ext::ObjectExt,
    nodes::{BinaryNode, CallNode, IfNode, Node},
};

#[test]
fn new_integer() {
    assert_eq!(Object::new_integer(0), Object::Abstract(Abstract::ZERO));

    assert_eq!(
        Object::new_integer(2),
        Composite::new(&mut [Property::new_successor_of(
            Composite::new(&mut [Property::new_successor_of(Abstract::ZERO.into())]).into()
        )])
        .into()
    );

    assert_eq!(
        Object::new_integer(-3),
        Composite::new(&mut [Property::new_predecessor_of(
            Composite::new(&mut [Property::new_predecessor_of(
                Composite::new(&mut [Property::new_predecessor_of(Abstract::ZERO.into())]).into()
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
    assert_eq!(Object::Composite(Composite::Empty).node(knowledge), None);

    // Single
    assert_eq!(
        Object::new_node(Node::Function(Object::Abstract(Abstract::ZERO))).node(knowledge),
        Some(Node::Function(Abstract::ZERO.into()))
    );

    // TODO: more
}

#[test]
fn call() {
    let f = Object::new_node(Node::Function(Object::new_node(Node::Parameter(0))));

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
    use everything_objects::{Abstract, Composite, Object, Property};

    use crate::{
        ObjectOrSetValues,
        base::BASE,
        ext::{AbstractExt, CompositeExt, ObjectExt, PropertyExt},
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
                Object::new_node(Node::And(BinaryNode {
                    left: Composite::new_bool(left).into(),
                    right: Composite::new_bool(right).into()
                }))
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
                Object::new_node(Node::Or(BinaryNode {
                    left: Composite::new_bool(left).into(),
                    right: Composite::new_bool(right).into()
                }))
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
            Object::new_node(Node::Not(Object::new_integer(42))),
        ];

        for subject in subjects {
            assert_eq!(
                Object::new_node(Node::Literal(subject.clone()))
                    .evaluate(&BASE, &mut Default::default())
                    .into_object(),
                subject
            );
        }
    }

    #[test]
    fn eval_count() {
        assert_eq!(
            Object::new_node(Node::Count(Composite::Empty.into()))
                .evaluate(&BASE, &mut Default::default())
                .into_object(),
            Object::new_integer(0)
        );

        assert_eq!(
            Object::new_node(Node::Count(Object::new_node(Node::Literal(
                Composite::new(&mut [
                    Property::new_contains(Abstract::ZERO.into()),
                    Property::new_contains(Abstract::KNOWLEDGE.into()),
                ])
                .into()
            ))))
            .evaluate(&BASE, &mut Default::default())
            .into_object(),
            Object::new_integer(2)
        );
    }

    #[test]
    fn eval_query() {
        assert_eq!(
            Object::new_node_query_values(
                Composite::new(&mut [
                    Property::new_contains(Abstract::ZERO.into()),
                    Property::new_contains(Abstract::BIT_0.into()),
                    Property::new_successor_of(Object::new_integer(0)),
                ])
                .into(),
                Object::new_node(Node::Literal(Abstract::CONTAINS.into()))
            )
            .evaluate(&BASE, &mut Default::default())
            .into_object(),
            Composite::new_set([Abstract::BIT_0.into(), Object::Abstract(Abstract::ZERO)]).into(),
        );
    }

    #[test]
    fn set_items() {
        let f = Object::new_node(Node::Function(Object::new_node(Node::Function(
            Composite::new_set([
                Object::new_node(Node::Parameter(0)),
                Object::new_node(Node::Parameter(1)),
            ])
            .into(),
        ))));

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
            Composite::new_set([
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

            let node = Object::new_node(Node::Count(Object::new_node(Node::Literal(
                Composite::new(&mut properties).into(),
            ))));

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

        let node = Object::new_node(Node::Multiply(BinaryNode {
            left: Object::new_node(Node::Literal(Object::new_integer(a))),
            right: Object::new_node(Node::Literal(Object::new_integer(b))),
        }));

        assert_eq!(
            node.evaluate(&BASE, &mut Default::default())
                .into_object()
                .to_integer(&BASE),
            Some(a * b)
        );
    }

    #[test]
    fn parameter_references() {
        let objects = [
            Object::new_integer(3458349),
            Abstract(58349580234958034).into(),
            Object::new_node(Node::Not(Composite::Empty.into())),
        ];

        let identity = Object::new_node(Node::Function(Object::new_node(Node::Parameter(0))));

        for object in objects.iter() {
            assert_eq!(
                &identity
                    .call(
                        &BASE,
                        &[ObjectOrSetValues::Object(object.clone())],
                        &mut Default::default()
                    )
                    .into_object(),
                &object.clone()
            );
        }

        let out_of_scope = Object::new_node(Node::Function(Object::new_node(Node::Parameter(1))));

        assert_eq!(
            out_of_scope
                .call(
                    &BASE,
                    &[ObjectOrSetValues::Object(Abstract(348593485934).into())],
                    &mut Default::default()
                )
                .into_object(),
            Object::Composite(Composite::Empty)
        );

        let capture_to_constant = Object::new_node(Node::Function(Object::new_node(
            Node::Function(Object::new_node(Node::Parameter(1))),
        )));

        for object in objects.iter() {
            let constant = capture_to_constant
                .call(
                    &BASE,
                    &[ObjectOrSetValues::Object(object.clone())],
                    &mut Default::default(),
                )
                .into_object();

            for other in objects.iter() {
                assert_eq!(
                    &constant
                        .call(
                            &BASE,
                            &[ObjectOrSetValues::Object(other.clone())],
                            &mut Default::default()
                        )
                        .into_object(),
                    &object.clone()
                );
            }
        }
    }
}

#[test]
fn factorial() {
    let factorial = Object::new_node(Node::Function(Object::new_node(Node::If(IfNode {
        condition: Object::new_node(Node::Less(BinaryNode {
            left: Object::new_node(Node::Parameter(0)),
            right: Object::new_integer(2),
        })),
        then: Object::new_integer(1),
        otherwise: Object::new_node(Node::Multiply(BinaryNode {
            left: Object::new_node(Node::Parameter(0)),
            right: Object::new_node(Node::Call(CallNode {
                callee: Object::new_node(Node::FunctionSelf(0)),
                with: Object::new_node(Node::Add(BinaryNode {
                    left: Object::new_node(Node::Parameter(0)),
                    right: Object::new_integer(-1),
                })),
            })),
        })),
    }))));

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
