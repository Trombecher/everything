use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn set_union_requires_a_set() {
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());
    let empty_set = Node::Literal(Object::Composite(Composite::empty()));

    assert_eq!(
        evaluate(
            &Node::SetUnion {
                first: construction.into(),
                second: empty_set.clone().into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );

    // Atoms and plain composites are sets with no items.
    let atom = Node::Literal(Object::Atom(Atom(42)));
    let union = evaluate(
        &Node::SetUnion {
            first: atom.into(),
            second: empty_set.into(),
        },
        &BASE,
    );

    assert!(union.is_composite());
    assert_eq!(union.item_count(&BASE), 0);
}

#[test]
fn set_union_iterates_each_item_once() {
    let union = Node::SetUnion {
        first: Node::Literal(Object::Composite(integer_set())).into(),
        second: Node::Literal(Object::Composite(integer_set())).into(),
    };

    let result = evaluate(&union, &BASE);

    assert_eq!(
        result.iter(&BASE).collect::<Vec<_>>(),
        vec![integer(1), integer(2), integer(3)]
    );
    assert_eq!(result.item_count(&BASE), 3);
}

#[test]
fn set_union_ignores_non_item_properties() {
    let union = Node::SetUnion {
        first: Node::Literal(composite_of([integer(1), integer(2)])).into(),
        second: Node::Literal(set_of([integer(3)])).into(),
    };

    assert_eq!(
        evaluate(&union, &BASE).into_object(&BASE),
        set_of([integer(3)])
    );
}
