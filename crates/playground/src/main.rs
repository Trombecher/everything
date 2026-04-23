use everything::ext::AbstractExt;
use everything_structures::{Abstract, Object, Structure, TextStructure};
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;

fn main() {
    tracing::subscriber::set_global_default(Registry::default().with(HierarchicalLayer::new(2)))
        .unwrap();

    println!(
        "{:?}",
        Structure::Text(TextStructure::new("Hello, World!").unwrap())
            .values(Object::Abstract(Abstract::LIST_TAIL))
    );
}
