use everything::{
    Knowledge, ObjectOrSetValues,
    base::BASE,
    ext::{AbstractExt, ObjectExt, PropertyExt, StructureExt},
    nodes::{BinaryNode, Node, UnwrapOrNode},
};
use everything_structures::{Abstract, Object, Property, Structure};
use tracing_subscriber::layer::SubscriberExt;

fn main() {
    let objects = [
        Object::new_integer(3458349),
        Abstract(58349580234958034).into(),
        Object::new_node(Node::Not(Structure::Empty.into())),
    ];

    tracing::subscriber::set_global_default(
        tracing_subscriber::Registry::default().with(tracing_tree::HierarchicalLayer::new(2)),
    )
    .unwrap();

    let capture_to_constant = Object::new_node(Node::Function(Object::new_node(Node::Function(
        Object::new_node(Node::Parameter(1)),
    ))));

    for object in objects.iter().cloned() {
        let constant = capture_to_constant
            .call(
                &BASE,
                &[ObjectOrSetValues::Object(object.clone())],
                &mut Default::default(),
            )
            .into_object();

        println!("|> {constant:?}");

        for other in objects.iter().cloned() {
            assert_eq!(
                constant
                    .call(
                        &BASE,
                        &[ObjectOrSetValues::Object(other.clone())],
                        &mut Default::default()
                    )
                    .into_object(),
                object
            );
        }
    }
}
