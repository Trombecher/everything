use ulid::Ulid;

use crate::{Change, Id, Property, Structure, structures::GLOBAL_REGISTRY};
use std::{assert_matches, sync::Arc};

fn contains_knowledge_structure() -> Structure {
    Structure::EMPTY.change(&mut [Change::Add(Property {
        tag: Id::CONTAINS,
        value: Id::KNOWLEDGE,
    })])
}

#[test]
fn empty_structure() {
    assert_eq!(Structure::EMPTY.change(&mut []), Structure::EMPTY);
}

#[test]
fn one_structure() {
    assert_eq!(
        contains_knowledge_structure().as_ref(),
        [Property {
            tag: Id::CONTAINS,
            value: Id::KNOWLEDGE
        }]
    );
}

#[test]
fn inner_structure() {
    let inner = contains_knowledge_structure();

    let outer = Structure::EMPTY.change(&mut [Change::Add(Property {
        tag: Id::CONTAINS,
        value: inner.clone().into(),
    })]);

    assert_eq!(GLOBAL_REGISTRY.entries.len(), 2);

    assert_eq!(
        outer.as_ref(),
        [Property {
            tag: Id::CONTAINS,
            value: inner.into()
        }]
    )
}

#[test]
fn remove_props() {
    let structure = contains_knowledge_structure();

    let should_be_empty = structure.change(&mut [Change::Remove(Property {
        tag: Id::CONTAINS,
        value: Id::KNOWLEDGE,
    })]);

    assert_matches!(should_be_empty.propeties, None);
}

#[test]
fn has() {
    let structure = Structure::EMPTY.change(&mut [Change::Add(Property {
        tag: Id::CONTAINS,
        value: Id::KNOWLEDGE,
    })]);

    assert!(structure.has(&Property {
        tag: Id::CONTAINS,
        value: Id::KNOWLEDGE
    }));

    assert!(!structure.has(&Property {
        tag: Id::CONTAINS,
        value: Id::CONTAINS
    }))
}

/// This test assures that structurally identical objects
/// will have the same allocation.
#[test]
fn deduping() {
    let structure_a = contains_knowledge_structure();
    let structure_b = contains_knowledge_structure();

    assert!(Arc::ptr_eq(
        &structure_a.propeties.as_ref().unwrap(),
        &structure_b.propeties.as_ref().unwrap()
    ))
}

#[test]
fn debug() {
    println!("{:?}", contains_knowledge_structure());

    println!("{:?}", Id::Abstract(Ulid::new()));
}
