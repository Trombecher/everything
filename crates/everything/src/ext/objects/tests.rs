use everything_objects::{Abstract, Composite, Object, Property};

use crate::{
    base::BASE,
    ctx::EvaluationContext,
    ext::{AbstractExt, ObjectExt},
    nodes::{
        BinaryNode, CallNode, FilterNode, IfNode, MapNode, Node, QueryExistsNode,
        QuerySubjectsAndTagsNode, QuerySubjectsAndValuesNode, QuerySubjectsNode,
        QueryTagsAndValuesNode, QueryTagsNode, QueryValuesNode, UnwrapOrNode,
    },
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
fn node_parsing() {
    const A: Object = Object::Abstract(Abstract(100));
    const B: Object = Object::Composite(Composite::Character('B'));
    const C: Object = Object::Composite(Composite::Empty);

    let knowledge = &BASE;

    let node_cases = [
        Node::Statements,
        Node::Add(BinaryNode { left: A, right: B }),
        Node::And(BinaryNode { left: A, right: B }),
        Node::Call(CallNode { callee: A, with: B }),
        Node::Count(A),
        Node::Equal(BinaryNode { left: A, right: B }),
        Node::Filter(FilterNode {
            set: A,
            filter_function: B,
        }),
        Node::Function(A),
        Node::FunctionSelf(42),
        Node::If(IfNode {
            condition: A,
            then: B,
            otherwise: C,
        }),
        Node::IsAbstract(A),
        Node::Less(BinaryNode { left: A, right: B }),
        Node::Literal(A),
        Node::Map(MapNode {
            mapper_function: A,
            set: B,
        }),
        Node::Multiply(BinaryNode { left: A, right: B }),
        Node::Not(C),
        Node::Or(BinaryNode { left: A, right: C }),
        Node::Parameter(67),
        Node::QueryExists(QueryExistsNode {
            subject: A,
            tag: B,
            value: C,
        }),
        Node::QuerySubjects(QuerySubjectsNode { tag: A, value: B }),
        Node::QuerySubjectsAndTags(QuerySubjectsAndTagsNode { value: A }),
        Node::QuerySubjectsAndValues(QuerySubjectsAndValuesNode { tag: A }),
        Node::QueryTags(QueryTagsNode {
            subject: A,
            value: B,
        }),
        Node::QueryTagsAndValues(QueryTagsAndValuesNode { subject: C }),
        Node::QueryValues(QueryValuesNode { subject: B, tag: C }),
        Node::Union(BinaryNode { left: A, right: C }),
        Node::UnwrapOr(UnwrapOrNode { default: C, set: B }),
        Node::Xor(BinaryNode { left: A, right: C }),
    ];

    for node in node_cases {
        assert_eq!(Object::new_node(node.clone()).node(knowledge), Some(node));
    }

    // None
    assert_eq!(Object::Composite(Composite::Empty).node(knowledge), None);

    // Double -> None
    assert_eq!(
        Object::Composite(Composite::new(&mut [
            Property {
                tag: Abstract::NODE_NOT.into(),
                value: C
            },
            Property {
                tag: Abstract::FUNCTION.into(),
                value: A,
            }
        ]))
        .node(knowledge),
        None
    );
}

#[test]
fn call() {
    let f = Object::new_node(Node::Function(Object::new_node(Node::Parameter(0))));

    assert_eq!(
        f.call(
            &BASE,
            &[Object::Abstract(Abstract::ZERO).into()],
            &mut EvaluationContext::default()
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
        ctx::EvaluationContext,
        ext::{AbstractExt, CompositeExt, ObjectExt, PropertyExt},
        nodes::{BinaryNode, Node, QueryValuesNode},
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
                .evaluate(&BASE, &mut EvaluationContext::default())
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
                .evaluate(&BASE, &mut EvaluationContext::default())
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
                    .evaluate(&BASE, &mut EvaluationContext::default())
                    .into_object(),
                subject
            );
        }
    }

    #[test]
    fn eval_count() {
        assert_eq!(
            Object::new_node(Node::Count(Composite::Empty.into()))
                .evaluate(&BASE, &mut EvaluationContext::default())
                .into_object(),
            Object::new_integer(0)
        );

        assert_eq!(
            Object::new_node(Node::Count(Object::new_node(Node::Literal(
                Composite::new(&mut [
                    Property::new_contains(Abstract::ZERO.into()),
                    Property::new_contains(Abstract::BIT_0.into()),
                ])
                .into()
            ))))
            .evaluate(&BASE, &mut EvaluationContext::default())
            .into_object(),
            Object::new_integer(2)
        );
    }

    #[test]
    fn eval_query() {
        assert_eq!(
            Object::new_node(Node::QueryValues(QueryValuesNode {
                subject: Composite::new(&mut [
                    Property::new_contains(Abstract::ZERO.into()),
                    Property::new_contains(Abstract::BIT_0.into()),
                    Property::new_successor_of(Object::new_integer(0)),
                ])
                .into(),
                tag: Object::new_node(Node::Literal(Abstract::CONTAINS.into()))
            }))
            .evaluate(&BASE, &mut EvaluationContext::default())
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
                &mut EvaluationContext::default(),
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
                node.evaluate(&BASE, &mut EvaluationContext::default())
                    .into_object()
                    .to_integer(&BASE),
                Some(count as i128)
            );
        }

        // TODO: Test real count (no duplicates)
    }

    #[test]
    fn multiply() {
        let a = 543_895;
        let b = 9_345_125;

        let node = Object::new_node(Node::Multiply(BinaryNode {
            left: Object::new_node(Node::Literal(Object::new_integer(a))),
            right: Object::new_node(Node::Literal(Object::new_integer(b))),
        }));

        assert_eq!(
            node.evaluate(&BASE, &mut EvaluationContext::default())
                .into_object()
                .to_integer(&BASE),
            Some(a * b)
        );
    }

    #[test]
    fn parameter_references() {
        let objects = [
            Object::new_integer(3_458_349),
            Abstract(58_349_580_234_958_034).into(),
            Object::new_node(Node::Not(Composite::Empty.into())),
        ];

        let identity = Object::new_node(Node::Function(Object::new_node(Node::Parameter(0))));

        for object in &objects {
            assert_eq!(
                &identity
                    .call(
                        &BASE,
                        &[ObjectOrSetValues::Object(object.clone())],
                        &mut EvaluationContext::default()
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
                    &[ObjectOrSetValues::Object(Abstract(348_593_485_934).into())],
                    &mut EvaluationContext::default()
                )
                .into_object(),
            Object::Composite(Composite::Empty)
        );

        let capture_to_constant = Object::new_node(Node::Function(Object::new_node(
            Node::Function(Object::new_node(Node::Parameter(1))),
        )));

        for object in &objects {
            let constant = capture_to_constant
                .call(
                    &BASE,
                    &[ObjectOrSetValues::Object(object.clone())],
                    &mut EvaluationContext::default(),
                )
                .into_object();

            for other in &objects {
                assert_eq!(
                    &constant
                        .call(
                            &BASE,
                            &[ObjectOrSetValues::Object(other.clone())],
                            &mut EvaluationContext::default()
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
                    &mut EvaluationContext::default()
                )
                .into_object(),
            Object::new_integer(output)
        );
    }
}
