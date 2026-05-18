use std::{hint::black_box, time::Instant};

use everything::{
    base::BASE,
    ext::{ObjectExt, StructureExt},
};
use everything_structures::{Object, Structure};
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;

fn main() {
    // tracing::subscriber::set_global_default(Registry::default().with(HierarchicalLayer::new(2)))
    //     .unwrap();

    let node = black_box(Object::Structure(Structure::new_node_map(
        Structure::new_node_filter(
            Structure::new_set([
                Object::new_integer(1),
                Object::new_integer(2),
                Object::new_integer(3),
                Object::new_integer(4),
                Object::new_integer(10),
                Object::new_integer(2423),
                Object::new_integer(45654),
            ])
            .into(),
            Structure::new_computed(
                Structure::new_node_less(
                    Object::new_integer(11),
                    Structure::new_node_parameter(0).into(),
                )
                .into(),
            )
            .into(),
        )
        .into(),
        Structure::new_computed(
            Structure::new_node_add(
                Structure::new_node_parameter(0).into(),
                Object::new_integer(1),
            )
            .into(),
        )
        .into(),
    )));

    let result = node.eval(&BASE, &mut Default::default());

    println!("{result:?}");
}
