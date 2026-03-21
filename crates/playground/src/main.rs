use everything::{base::BASE, ext::ObjectExt};
use everything_structures::Object;
use everything_structures_ff::parse_structure;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_tree::HierarchicalLayer;

fn main() {
    tracing_subscriber::registry()
        .with(HierarchicalLayer::new(4))
        .init();

    let f: Object = parse_structure("{(@10, @9)}").unwrap().into();

    println!("is valid: {:?}", f.is_valid(&BASE, false));
}
