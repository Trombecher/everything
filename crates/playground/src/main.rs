use everything_structures_ff::parse_structure;
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;

fn main() {
    tracing::subscriber::set_global_default(Registry::default().with(HierarchicalLayer::new(2)))
        .unwrap();

    let s = parse_structure(
        "{(@4353, {(@5000, @9843), (@5001, @9843), (@5002, @9843), (@5003, @9843), (@5004, @9843), (@5005, @9843), (@5006, @9843), (@5007, @6767)}), (@4354, {})}",
    );

    println!("{s:?}");
}
