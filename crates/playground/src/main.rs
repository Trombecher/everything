use std::num::NonZeroI128;

use everything::{base::BASE, ctx::EvaluationContext, ext::ObjectExt, query};
use everything_structures::{Abstract, Object, Structure};
use everything_structures_ff::Parsable;
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;

fn main() {
    tracing::subscriber::set_global_default(Registry::default().with(HierarchicalLayer::new(2)))
        .unwrap();

    println!(
        "{:?}",
        query::values(
            &BASE,
            Structure::Integer(NonZeroI128::new(30).unwrap()).into(),
            Abstract::SUCCESSOR_OF.into(),
            &mut EvaluationContext::default(),
        )
    );
}
