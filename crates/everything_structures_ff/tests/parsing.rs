use everything_structures::{Object, Property, Structure};
use everything_structures_ff::parse_structure;

#[test]
fn main() {
    assert_eq!(
        parse_structure("{(@1, @2)}"),
        Ok(Structure::EMPTY.change(
            &mut [],
            &mut [Property {
                tag: Object::Abstract(1),
                value: Object::Abstract(2),
            }]
        ))
    );
}
