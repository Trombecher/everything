use ulid::Ulid;

use crate::{Change, Object, Property, Structure, structures::GLOBAL_REGISTRY};
use std::{assert_matches, sync::Arc};

pub const ALICE: Object = Object::Abstract(Ulid::from_bytes(*b"This is Alice!!!"));
pub const BOB: Object = Object::Abstract(Ulid::from_bytes(*b"This is Bob!!!!!"));

fn alice_bob_structure() -> Structure {
    Structure::EMPTY.change(&mut [Change::Add(Property {
        tag: ALICE,
        value: BOB,
    })])
}

#[test]
fn empty_structure() {
    assert_eq!(Structure::EMPTY.change(&mut []), Structure::EMPTY);
}

#[test]
fn one_structure() {
    assert_eq!(
        alice_bob_structure().as_ref(),
        [Property {
            tag: ALICE,
            value: BOB
        }]
    );
}

#[test]
fn inner_structure() {
    let inner = alice_bob_structure();

    let outer = Structure::EMPTY.change(&mut [Change::Add(Property {
        tag: ALICE,
        value: inner.clone().into(),
    })]);

    assert_eq!(GLOBAL_REGISTRY.entries.len(), 2);

    assert_eq!(
        outer.as_ref(),
        [Property {
            tag: ALICE,
            value: inner.into()
        }]
    )
}

#[test]
fn remove_props() {
    let structure = alice_bob_structure();

    let should_be_empty = structure.change(&mut [Change::Remove(Property {
        tag: ALICE,
        value: BOB,
    })]);

    assert_matches!(should_be_empty.propeties, None);
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

/// This test assures that structurally identical objects
/// will have the same allocation.
#[test]
fn deduping() {
    let structure_a = alice_bob_structure();
    let structure_b = alice_bob_structure();

    assert!(Arc::ptr_eq(
        &structure_a.propeties.as_ref().unwrap(),
        &structure_b.propeties.as_ref().unwrap()
    ))
}

#[test]
fn debug() {
    println!("{:?}", alice_bob_structure());

    println!("{:?}", Object::Abstract(Ulid::new()));
}
