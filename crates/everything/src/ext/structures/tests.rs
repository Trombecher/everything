// TODO: tests for structures

use everything_structures::{Object, Property, Structure};

use crate::ext::StructureExt;

const ALICE: Object = Object::Abstract(999_999_999);
const BOB: Object = Object::Abstract(888_888_888);

#[test]
fn has_exactly_one_value_on() {
    assert!(!Structure::EMPTY.has_exactly_one_value_on(&ALICE));

    assert!(
        Structure::new(&mut [Property {
            tag: ALICE,
            value: ALICE
        }])
        .has_exactly_one_value_on(&ALICE)
    );

    assert!(
        !Structure::new(&mut [
            Property {
                tag: ALICE,
                value: ALICE,
            },
            Property {
                tag: ALICE,
                value: BOB,
            }
        ])
        .has_exactly_one_value_on(&ALICE)
    );
}
