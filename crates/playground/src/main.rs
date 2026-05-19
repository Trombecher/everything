use std::hint::black_box;

use everything::{
    Knowledge,
    base::BASE,
    ext::{AbstractExt, ObjectExt, PropertyExt, StructureExt},
    nodes::{BinaryNode, IfNode, Node, UnwrapOrNode},
};
use everything_structures::{Abstract, Object, Property, Structure};
use tracing_subscriber::layer::SubscriberExt;

fn integer_constraint(tag: Object) -> Object {
    Structure::new_node(Node::Function(
        Structure::new_node(Node::Function(
            Structure::new_node(Node::And(BinaryNode {
                left: Structure::new_node(Node::Equal(BinaryNode {
                    left: Structure::new_node(Node::Count(
                        Structure::new_node_query_values(
                            Structure::new_node(Node::Parameter(1)).into(),
                            tag,
                        )
                        .into(),
                    ))
                    .into(),
                    right: Object::new_integer(1),
                }))
                .into(),
                right: Structure::new_node(Node::Or(BinaryNode {
                    left: Structure::new_node(Node::Equal(BinaryNode {
                        left: Structure::new_node(Node::Parameter(0)).into(),
                        right: Object::new_integer(0),
                    }))
                    .into(),
                    right: Structure::new_node(Node::Or(BinaryNode {
                        left: Structure::new_node_query_values(
                            Structure::new_node(Node::Parameter(0)).into(),
                            Abstract::SUCCESSOR_OF.into(),
                        )
                        .into(),
                        right: Structure::new_node_query_values(
                            Structure::new_node(Node::Parameter(0)).into(),
                            Abstract::PREDECESSOR_OF.into(),
                        )
                        .into(),
                    }))
                    .into(),
                }))
                .into(),
            }))
            .into(),
        ))
        .into(),
    ))
    .into()
}

fn main() {
    // tracing::subscriber::set_global_default(
    //     tracing_subscriber::Registry::default().with(tracing_tree::HierarchicalLayer::new(2)),
    // )
    // .unwrap();

    const PETER: Object = Object::Abstract(Abstract(14575835));
    const ALICE: Object = Object::Abstract(Abstract(33252352));

    const LEFT_POCKET_COUNT: Object = Object::Abstract(Abstract(53453543435));
    const RIGHT_POCKET_COUNT: Object = Object::Abstract(Abstract(3434675347));
    const TOTAL_COUNT: Object = Object::Abstract(Abstract(32982309589));

    let knowledge = Knowledge::new(
        BASE.add(&mut [
            Property::new_contains(
                Structure::new_statement(
                    LEFT_POCKET_COUNT,
                    Abstract::AXIOMATIC.into(),
                    integer_constraint(LEFT_POCKET_COUNT),
                )
                .into(),
            ),
            Property::new_contains(
                Structure::new_statement(
                    RIGHT_POCKET_COUNT,
                    Abstract::AXIOMATIC.into(),
                    integer_constraint(RIGHT_POCKET_COUNT),
                )
                .into(),
            ),
            Property::new_contains(
                Structure::new_statement(PETER, LEFT_POCKET_COUNT, Object::new_integer(67)).into(),
            ),
            Property::new_contains(
                Structure::new_statement(PETER, RIGHT_POCKET_COUNT, Object::new_integer(42)).into(),
            ),
            Property::new_contains(
                Structure::new_statement(
                    TOTAL_COUNT,
                    Abstract::FUNCTION.into(),
                    Structure::new_node(Node::Add(BinaryNode {
                        left: Structure::new_node(Node::UnwrapOr(UnwrapOrNode {
                            set: Structure::new_node_query_values(
                                Structure::new_node(Node::Parameter(0)).into(),
                                LEFT_POCKET_COUNT,
                            )
                            .into(),
                            default: Object::new_integer(0),
                        }))
                        .into(),
                        right: Structure::new_node(Node::UnwrapOr(UnwrapOrNode {
                            set: Structure::new_node_query_values(
                                Structure::new_node(Node::Parameter(0)).into(),
                                RIGHT_POCKET_COUNT,
                            )
                            .into(),
                            default: Object::new_integer(0),
                        }))
                        .into(),
                    }))
                    .into(),
                )
                .into(),
            ),
        ]),
    )
    .unwrap();

    println!("{:?}", knowledge.query_values(PETER, TOTAL_COUNT));
}
