use everything::{EvaluationContext, base::BASE, ext::ObjectExt};
use everything_structures::Object;
use everything_structures_ff::parse_structure;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_tree::HierarchicalLayer;

fn main() {
    tracing_subscriber::registry()
        .with(HierarchicalLayer::new(4))
        .init();

    let f: Object = parse_structure("{(@3, {(@3, {(@15, {(@10, @9)})})})}")
        .unwrap()
        .into();

    let parameter = Object::Abstract(1337);

    let result = f.call(&BASE, &parameter, &mut EvaluationContext::default());

    println!("{result:?}");
}
