use everything::{
    Knowledge,
    base::BASE,
    ctx::EvaluationContext,
    ext::{AbstractExt, ObjectExt, StructureExt},
};
use everything_structures::{Abstract, Object, Structure};
use everything_structures_ff::Parsable;
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;

fn main() {
    tracing::subscriber::set_global_default(Registry::default().with(HierarchicalLayer::new(2)))
        .unwrap();

    println!(
        "{:?}",
        Object::parse("{(@6969, 100)}")
            .unwrap()
            .eval(&BASE, &mut EvaluationContext::default())
    )
}
