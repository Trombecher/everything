use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn fold_sums_the_composite() {
    let node = Node::SetFold {
        set: Node::Literal(integer_set().into()).into(),
        folder: add_folder().into(),
        initial: Node::Literal(integer(0)).into(),
    };

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        integer(1 + 2 + 3)
    );
}

#[test]
fn fold_of_a_union_iterates_each_element_once() {
    let union = Node::SetUnion {
        first: Node::Literal(Object::Composite(integer_set())).into(),
        second: Node::Literal(Object::Composite(integer_set())).into(),
    };

    let node = Node::SetFold {
        set: union.into(),
        folder: add_folder().into(),
        initial: Node::Literal(integer(0)).into(),
    };

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        integer(1 + 2 + 3)
    );
}

#[test]
fn fold_of_an_empty_composite_returns_the_initial_value() {
    let initial = integer(42);

    let node = Node::SetFold {
        set: Node::Literal(Object::Composite(Composite::empty())).into(),
        // The folder is never called, so it may even be undefined.
        folder: Node::Literal(Atom::UNDEFINED.into()).into(),
        initial: Node::Literal(initial.clone()).into(),
    };

    assert_eq!(evaluate(&node, &BASE).into_object(&BASE), initial);
}

#[test]
fn fold_of_an_empty_deferred_composite_returns_the_initial_value() {
    let drop_everything = Node::function(Node::Literal(Atom::FALSE.into()));
    let empty_composite = Node::SetFilter {
        set: Node::Literal(integer_set().into()).into(),
        filter_function: drop_everything.into(),
    };
    let initial = integer(42);

    let node = Node::SetFold {
        set: empty_composite.into(),
        // The folder is never called, so it may even be undefined.
        folder: Node::Literal(Atom::UNDEFINED.into()).into(),
        initial: Node::Literal(initial.clone()).into(),
    };

    assert_eq!(evaluate(&node, &BASE).into_object(&BASE), initial);
}

#[test]
fn fold_requires_a_set() {
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());

    assert_eq!(
        evaluate(
            &Node::SetFold {
                set: construction.into(),
                folder: add_folder().into(),
                initial: Node::Literal(integer(0)).into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );

    // Atoms are sets with no items, so the initial value is returned.
    for set in [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Atom::UNDEFINED.into()),
    ] {
        assert_eq!(
            evaluate(
                &Node::SetFold {
                    set: set.into(),
                    folder: add_folder().into(),
                    initial: Node::Literal(integer(0)).into(),
                },
                &BASE,
            )
            .into_object(&BASE),
            integer(0)
        );
    }
}

#[test]
fn fold_requires_a_function_folder() {
    let empty_composite = Node::Literal(Object::Composite(Composite::empty()));
    let invalid_folders = [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::Literal(Atom::UNDEFINED.into()),
        Node::SetUnion {
            first: empty_composite.clone().into(),
            second: empty_composite.into(),
        },
    ];

    for folder in invalid_folders {
        assert_eq!(
            evaluate(
                &Node::SetFold {
                    set: Node::Literal(integer_set().into()).into(),
                    folder: folder.into(),
                    initial: Node::Literal(integer(0)).into(),
                },
                &BASE,
            )
            .into_object(&BASE),
            Atom::UNDEFINED.into()
        );
    }
}

#[test]
fn fold_yields_undefined_when_the_folder_result_is_not_callable() {
    // The folder returns the accumulator, which is not a function.
    let folder = Node::function(Node::Parameter { depth: 0 });

    let node = Node::SetFold {
        set: Node::Literal(integer_set().into()).into(),
        folder: folder.into(),
        initial: Node::Literal(integer(0)).into(),
    };

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn fold_yields_undefined_when_the_folder_returns_undefined() {
    let folder = Node::function(Node::Literal(Atom::UNDEFINED.into()));

    let node = Node::SetFold {
        set: Node::Literal(integer_set().into()).into(),
        folder: folder.into(),
        initial: Node::Literal(integer(0)).into(),
    };

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn fold_iterates_deferred_composites() {
    let doubled = Node::SetMap {
        set: Node::Literal(integer_set().into()).into(),
        mapper_function: Node::function(Node::Add {
            first: Node::Parameter { depth: 0 }.into(),
            second: Node::Parameter { depth: 0 }.into(),
        })
        .into(),
    };

    let node = Node::SetFold {
        set: doubled.into(),
        folder: add_folder().into(),
        initial: Node::Literal(integer(0)).into(),
    };

    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        integer(2 + 4 + 6)
    );
}

#[test]
fn fold_can_accumulate_deferred_sets() {
    let set = set_of([set_of([integer(1)]), set_of([integer(2)])]);

    // The curried function `acc |-> element |-> acc | element`.
    let folder = Node::function(Node::function(Node::SetUnion {
        first: Node::Parameter { depth: 1 }.into(),
        second: Node::Parameter { depth: 0 }.into(),
    }));

    let node = Node::SetFold {
        set: Node::Literal(set).into(),
        folder: folder.into(),
        initial: Node::Literal(Object::Composite(Composite::empty())).into(),
    };

    let result = evaluate(&node, &BASE);

    assert!(result.is_composite());
    assert_eq!(
        result.iter(&BASE).collect::<Vec<_>>(),
        vec![integer(1), integer(2)]
    );
}

#[test]
fn fold_captures_the_evaluation_context() {
    // The curried function
    // `acc |-> element |-> acc + element + offset`,
    // where `offset` comes from the surrounding function.
    let folder = Node::function(Node::function(Node::Add {
        first: Node::Add {
            first: Node::Parameter { depth: 1 }.into(),
            second: Node::Parameter { depth: 0 }.into(),
        }
        .into(),
        second: Node::Parameter { depth: 2 }.into(),
    }));

    let outer_function = Node::function(Node::SetFold {
        set: Node::Literal(integer_set().into()).into(),
        folder: folder.into(),
        initial: Node::Literal(integer(0)).into(),
    });

    let node = Node::Call {
        callee: outer_function.into(),
        with: Node::Literal(integer(10)).into(),
    };

    // (0 + 1 + 10) + (2 + 10) + (3 + 10)
    assert_eq!(
        evaluate(&node, &BASE).into_object(&BASE),
        integer(1 + 2 + 3 + 3 * 10)
    );
}
