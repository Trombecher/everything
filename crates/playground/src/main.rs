use everything_structures::Object;
use everything_structures_ff::Parsable;

fn main() {
    /*
    tracing::subscriber::set_global_default(Registry::default().with(HierarchicalLayer::new(2)))
        .unwrap();
     */

    let object = Object::parse("{(@6969, 33)}");

    println!("{object:?}");
}
