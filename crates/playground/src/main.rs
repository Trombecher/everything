use everything::{base::BASE, ctx::EvaluationContext, ext::ObjectExt};
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

    let mut ctx = EvaluationContext::default();

    let result = f.call(
        &BASE,
        &[Object::Abstract(1337), Object::Abstract(5345349)],
        &mut ctx,
    );

    println!("{result:?}");
}
