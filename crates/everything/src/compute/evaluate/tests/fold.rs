use crate::{
    base::BASE,
    model::{Atom, Construction, Object, Set},
};

use super::super::*;
use super::common::*;

#[test]
fn fold_sums_the_set() {
    let node = Node::Fold {
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
    let union = Node::Union {
        first: Node::Literal(Object::Set(integer_set())).into(),
        second: Node::Literal(Object::Set(integer_set())).into(),
    };

    let node = Node::Fold {
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
fn fold_of_an_empty_set_returns_the_initial_value() {
    let initial = integer(42);

    let node = Node::Fold {
        set: Node::Literal(Object::Set(Set::empty())).into(),
        // The folder is never called, so it may even be undefined.
        folder: Node::Literal(Atom::UNDEFINED.into()).into(),
        initial: Node::Literal(initial.clone()).into(),
    };

    assert_eq!(evaluate(&node, &BASE).into_object(&BASE), initial);
}

#[test]
fn fold_of_an_empty_deferred_set_returns_the_initial_value() {
    let drop_everything = Node::Function {
        body: Node::Literal(Atom::FALSE.into()).into(),
    };
    let empty_set = Node::Filter {
        set: Node::Literal(integer_set().into()).into(),
        filter_function: drop_everything.into(),
    };
    let initial = integer(42);

    let node = Node::Fold {
        set: empty_set.into(),
        // The folder is never called, so it may even be undefined.
        folder: Node::Literal(Atom::UNDEFINED.into()).into(),
        initial: Node::Literal(initial.clone()).into(),
    };

    assert_eq!(evaluate(&node, &BASE).into_object(&BASE), initial);
}

#[test]
fn fold_requires_a_set() {
    let invalid_sets = [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::Literal(Atom::UNDEFINED.into()),
    ];

    for set in invalid_sets {
        assert_eq!(
            evaluate(
                &Node::Fold {
                    set: set.into(),
                    folder: add_folder().into(),
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
fn fold_requires_a_function_folder() {
    let empty_set = Node::Literal(Object::Set(Set::empty()));
    let invalid_folders = [
        Node::Literal(Object::Atom(Atom(42))),
        Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into()),
        Node::Literal(Atom::UNDEFINED.into()),
        Node::Union {
            first: empty_set.clone().into(),
            second: empty_set.into(),
        },
    ];

    for folder in invalid_folders {
        assert_eq!(
            evaluate(
                &Node::Fold {
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
    let folder = Node::Function {
        body: Node::Parameter { depth: 0 }.into(),
    };

    let node = Node::Fold {
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
    let folder = Node::Function {
        body: Node::Literal(Atom::UNDEFINED.into()).into(),
    };

    let node = Node::Fold {
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
fn fold_iterates_deferred_sets() {
    let doubled = Node::Map {
        set: Node::Literal(integer_set().into()).into(),
        mapper_function: Node::Function {
            body: Node::Add {
                first: Node::Parameter { depth: 0 }.into(),
                second: Node::Parameter { depth: 0 }.into(),
            }
            .into(),
        }
        .into(),
    };

    let node = Node::Fold {
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
    let set = Set::from([
        Object::Set(Set::from([integer(1)])),
        Object::Set(Set::from([integer(2)])),
    ]);

    // The curried function `acc |-> element |-> acc | element`.
    let folder = Node::Function {
        body: Node::Function {
            body: Node::Union {
                first: Node::Parameter { depth: 1 }.into(),
                second: Node::Parameter { depth: 0 }.into(),
            }
            .into(),
        }
        .into(),
    };

    let node = Node::Fold {
        set: Node::Literal(set.into()).into(),
        folder: folder.into(),
        initial: Node::Literal(Object::Set(Set::empty())).into(),
    };

    let result = evaluate(&node, &BASE);

    assert!(result.is_set());
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
    let folder = Node::Function {
        body: Node::Function {
            body: Node::Add {
                first: Node::Add {
                    first: Node::Parameter { depth: 1 }.into(),
                    second: Node::Parameter { depth: 0 }.into(),
                }
                .into(),
                second: Node::Parameter { depth: 2 }.into(),
            }
            .into(),
        }
        .into(),
    };

    let outer_function = Node::Function {
        body: Node::Fold {
            set: Node::Literal(integer_set().into()).into(),
            folder: folder.into(),
            initial: Node::Literal(integer(0)).into(),
        }
        .into(),
    };

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
