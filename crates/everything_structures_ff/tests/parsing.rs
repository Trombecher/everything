use everything_structures::{Abstract, Object, Property, Structure};
use everything_structures_ff::Parsable;

#[test]
fn main() {
    assert_eq!(
        Structure::parse("{(@1, @2)}"),
        Ok(Structure::new(&mut [Property {
            tag: Object::Abstract(Abstract(1)),
            value: Object::Abstract(Abstract(2)),
        }]))
    );
}
