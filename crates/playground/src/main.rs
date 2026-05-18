use everything::{
    base::BASE,
    ext::{ObjectExt, StructureExt},
    nodes::{BinaryNode, IfNode, Node},
};
use everything_structures::{Object, Structure};
use tracing_subscriber::layer::SubscriberExt;

fn main() {
    // tracing::subscriber::set_global_default(
    //     tracing_subscriber::Registry::default().with(tracing_tree::HierarchicalLayer::new(2)),
    // )
    // .unwrap();

    /*
    let node = black_box(Object::Structure(Structure::new_node(Node::Map(MapNode {
        set: Structure::new_node(Node::Filter(FilterNode {
            set: Structure::new_set([
                Object::new_integer(1),
                Object::new_integer(2),
                Object::new_integer(3),
                Object::new_integer(4),
                Object::new_integer(10),
                Object::new_integer(2423),
                Object::new_integer(45654),
            ])
            .into(),
            filter_function: Structure::new_node(Node::Computed(
                Structure::new_node(Node::Less(BinaryNode {
                    left: Object::new_integer(11),
                    right: Structure::new_node(Node::Parameter(0)).into(),
                }))
                .into(),
            ))
            .into(),
        }))
        .into(),
        mapper_function: Structure::new_node(Node::Computed(
            Structure::new_node(Node::Add(BinaryNode {
                left: Structure::new_node(Node::Parameter(0)).into(),
                right: Object::new_integer(1),
            }))
            .into(),
        ))
        .into(),
    }))));

    let result = node.eval(&BASE, &mut Default::default());

    println!("{result:?}");
     */

    let gauss = Object::Structure(Structure::new_node(Node::Computed(
        Structure::new_node(Node::If(IfNode {
            condition: Structure::new_node(Node::Less(BinaryNode {
                left: Structure::new_node(Node::Parameter(0)).into(),
                right: Object::new_integer(2),
            }))
            .into(),
            then: Structure::new_node(Node::Parameter(0)).into(),
            otherwise: Structure::new_node(Node::Add(BinaryNode {
                left: Structure::new_node(Node::Parameter(0)).into(),
                right: Structure::new_node_query_values(
                    Structure::new_node(Node::Add(BinaryNode {
                        left: Structure::new_node(Node::Parameter(0)).into(),
                        right: Object::new_integer(-1),
                    }))
                    .into(),
                    Structure::new_node(Node::FunctionSelf(0)).into(),
                )
                .into(),
            }))
            .into(),
        }))
        .into(),
    )));

    dbg!(gauss.call(
        &BASE,
        &[Object::new_integer(42424)],
        &mut Default::default()
    ));
}
