use everything_structures::{Change, Object, Property, Structure};
use everything_structures_ff::parse_structure;
use ulid::Ulid;

#[test]
fn main() {
    assert_eq!(
        parse_structure("{(@1, @2)}"),
        Ok(Structure::EMPTY.change(&mut [Change::Add(Property {
            tag: Object::Abstract(Ulid(1)),
            value: Object::Abstract(Ulid(2)),
        })]))
    );
}
