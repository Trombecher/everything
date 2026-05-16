use std::num::NonZeroI128;

use everything::{base::BASE, ctx::EvaluationContext, ext::ObjectExt, query};
use everything_structures::{Abstract, Byte, Object, Property, Structure};
use everything_structures_ff::Parsable;
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;

fn main() {
    tracing::subscriber::set_global_default(Registry::default().with(HierarchicalLayer::new(2)))
        .unwrap();

    println!(
        "{:?}",
        Structure::new(&mut [
            Property::new_list_item(Structure::Character('X').into()),
            Property::new_list_tail(Structure::Empty.into())
        ])
    );
}
