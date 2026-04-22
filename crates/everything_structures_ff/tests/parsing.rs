use everything_structures::{Abstract, Object, Property, Structure};
use everything_structures_ff::parse_structure;

#[test]
fn main() {
    assert_eq!(
        parse_structure("{(@1, @2)}"),
        Ok(Structure::new(&mut [Property {
            tag: Object::Abstract(Abstract(1)),
            value: Object::Abstract(Abstract(2)),
        }]))
    );
}
