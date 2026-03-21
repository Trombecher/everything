use std::time::Instant;

use everything::{base::BASE, ext::ObjectExt};
use everything_structures::Object;
use everything_structures_ff::parse_structure;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_tree::HierarchicalLayer;

fn main() {
    /*
    tracing_subscriber::registry()
        .with(HierarchicalLayer::new(4))
        .init();
     */

    let now = Instant::now();
    let is_valid = Object::new_natural_number(1000).is_valid(&BASE, false);

    println!("is valid: {:?} in {:?}", is_valid, now.elapsed());
}
