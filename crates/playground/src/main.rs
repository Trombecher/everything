use everything::{
    base::BASE,
    ext::{ObjectExt, StructureExt},
};
use everything_structures::{Object, Structure};
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;

fn main() {
    tracing::subscriber::set_global_default(Registry::default().with(HierarchicalLayer::new(2)))
        .unwrap();

    let result = Object::Structure(Structure::new_node_map(
        Structure::new_set([
            Object::new_integer(10),
            Object::new_integer(20),
            Object::new_integer(30),
        ])
        .into(),
        Structure::new_computed(
            Structure::new_node_add(
                Structure::new_node_parameter(0).into(),
                Object::new_integer(1),
            )
            .into(),
        )
        .into(),
    ))
    .eval(&BASE, &mut Default::default());

    println!("{result:?}")
}
