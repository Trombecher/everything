use crate::{Object, Property, Structure};
use std::sync::Arc;

pub const ALICE: Object = Object::Abstract(u128::from_be_bytes(*b"This is Alice!!!"));
pub const BOB: Object = Object::Abstract(u128::from_be_bytes(*b"This is Bob!!!!!"));

fn alice_bob_structure() -> Structure {
    Structure::new(&mut [Property {
        tag: ALICE,
        value: BOB,
    }])
}

#[test]
fn empty_structure() {
    assert_eq!(Structure::new(&mut []), []);
}

#[test]
fn one_structure() {
    assert_eq!(
        alice_bob_structure(),
        [Property {
            tag: ALICE,
            value: BOB
        }]
    );
}

#[test]
fn inner_structure() {
    let inner = alice_bob_structure();

    let outer = Structure::new(&mut [Property {
        tag: ALICE,
        value: inner.clone().into(),
    }]);

    assert_eq!(
        outer,
        [Property {
            tag: ALICE,
            value: inner.into()
        }]
    )
}

#[test]
fn remove_props() {
    let structure = alice_bob_structure();

    let should_be_empty = structure.remove(&mut [Property {
        tag: ALICE,
        value: BOB,
    }]);

    assert_eq!(should_be_empty, []);
}

#[test]
fn has() {
    let structure = alice_bob_structure();

    assert!(structure.has(&Property {
        tag: ALICE,
        value: BOB
    }));

    assert!(!structure.has(&Property {
        tag: ALICE,
        value: ALICE
    }))
}

#[test]
fn has_by_ref() {
    let structure = alice_bob_structure();

    assert!(structure.has_by_ref(&ALICE, &BOB));

    assert!(!structure.has_by_ref(&ALICE, &ALICE));
}

/// This test assures that structurally identical objects
/// will have the same allocation.
#[test]
fn deduping() {
    let structure_a = alice_bob_structure();
    let structure_b = alice_bob_structure();

    assert!(Arc::ptr_eq(
        &structure_a.any().unwrap().properties,
        &structure_b.any().unwrap().properties
    ))
}

#[test]
fn debug() {
    println!("{:?}", alice_bob_structure());

    println!("{:?}", Object::Abstract(42));
}

#[test]
fn no_values() {
    assert_eq!(Structure::Empty.values(ALICE).next(), None)
}

#[test]
fn one_value() {
    let s = alice_bob_structure();

    let mut alices = s.values(ALICE);
    assert_eq!(alices.next(), Some(&BOB));
    assert_eq!(alices.next(), None);

    assert_eq!(s.values(BOB).next(), None);
}

#[test]
fn multiple_values() {
    let s = Structure::new(&mut [
        Property {
            tag: ALICE,
            value: ALICE,
        },
        Property {
            tag: ALICE,
            value: BOB,
        },
    ]);

    let mut alices = s.values(ALICE);
    assert_eq!(alices.next(), Some(&ALICE));
    assert_eq!(alices.next(), Some(&BOB));
    assert_eq!(alices.next(), None);
}

#[test]
fn no_tags() {
    assert_eq!(Structure::EMPTY.tags(&ALICE).next(), None);
}

#[test]
fn one_tag() {
    let s = alice_bob_structure();

    let mut tags_that_have_bob = s.tags(&BOB);
    assert_eq!(tags_that_have_bob.next(), Some(&ALICE));
    assert_eq!(tags_that_have_bob.next(), None);

    assert_eq!(s.tags(&ALICE).next(), None);
}

#[test]
fn multiple_tags() {
    let s = Structure::new(&mut [
        Property {
            tag: ALICE,
            value: ALICE,
        },
        Property {
            tag: BOB,
            value: ALICE,
        },
    ]);

    let mut tags_that_have_alice = s.tags(&ALICE);
    assert_eq!(tags_that_have_alice.next(), Some(&ALICE));
    assert_eq!(tags_that_have_alice.next(), Some(&BOB));
    assert_eq!(tags_that_have_alice.next(), None);
}

#[test]
fn union() {
    let a = alice_bob_structure();
    let b = Structure::new(&mut [Property {
        tag: BOB,
        value: ALICE,
    }]);

    assert_eq!(
        a.union(&b),
        Structure::new(&mut [
            Property {
                tag: ALICE,
                value: BOB
            },
            Property {
                tag: BOB,
                value: ALICE
            }
        ])
    );
}

#[test]
fn subsets() {
    assert!(
        alice_bob_structure().is_subset_of(&alice_bob_structure().add(&mut [Property {
            tag: ALICE,
            value: ALICE
        }]))
    );

    assert!(!alice_bob_structure().is_subset_of(&Structure::EMPTY))
}
