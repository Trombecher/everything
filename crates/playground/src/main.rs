use everything::ext::AbstractExt;
use everything_structures::{
    Abstract, Bit, BitSlot, BytesStructure, Object, Property, Structure, TextStructure,
};
use everything_structures_ff::parse_structure;
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;

fn main() {
    tracing::subscriber::set_global_default(Registry::default().with(HierarchicalLayer::new(2)))
        .unwrap();

    let s = parse_structure("{(@1, \"hello\")}");

    println!("{s:?}");
}
