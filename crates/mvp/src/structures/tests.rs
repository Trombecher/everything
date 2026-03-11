use crate::{Change, Id, Property, Structure};

#[test]
fn empty_structure() {
    assert_eq!(Structure::EMPTY.change(&mut []), Structure::EMPTY);
}

#[test]
fn basic_structure() {
    assert_eq!(
        Structure::EMPTY
            .change(&mut [Change::Add(Property {
                tag: Id::CONTAINS,
                value: Id::CONTAINS,
            })])
            .as_ref(),
        [Property {
            tag: Id::CONTAINS,
            value: Id::CONTAINS
        }]
    );
}
