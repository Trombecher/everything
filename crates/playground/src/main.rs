use std::{array, time::Instant};

use everything::{
    base::BASE,
    ctx::EvaluationContext,
    ext::{ObjectExt, StructureExt},
    query::query_values_axiomatically,
};
use everything_structures::{Object, Structure};
use everything_structures_ff::parse_structure;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_tree::HierarchicalLayer;

fn main() {
    tracing_subscriber::registry()
        .with(HierarchicalLayer::new(4))
        .init();

    let subject: Object = parse_structure("{(@15, @9)}").unwrap().into();

    let tag = Object::NODE_PARAMETER;

    let tag_constraint = query_values_axiomatically(&BASE, &tag, Object::AXIOMATIC)
        .next()
        .unwrap();

    let value = query_values_axiomatically(&BASE, &subject, Object::NODE_PARAMETER)
        .next()
        .unwrap()
        .clone();

    let result = tag_constraint.call(&BASE, &[subject, value], &mut EvaluationContext::default());

    println!("{result:?}")
}
