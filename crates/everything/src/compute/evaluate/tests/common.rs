use num_bigint::BigInt;

use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;

pub fn product_iterator<A: Clone, B>(
    a: impl Iterator<Item = A>,
    b: impl Iterator<Item = B> + Clone,
) -> impl Iterator<Item = (A, B)> {
    a.flat_map(move |a_item| b.clone().map(move |b_item| (a_item.clone(), b_item)))
}

pub fn pair(atom: Atom, value: Object) -> Object {
    Composite::from([
        Construction::new(Atom::ATOM.into(), atom.into()).into(),
        Construction::new(Atom::VALUE.into(), value).into(),
    ])
    .into()
}

pub fn integer(value: i32) -> Object {
    Object::new_integer(BigInt::from(value))
}

pub fn integer_composite() -> Composite {
    Composite::from([integer(1), integer(2), integer(3)])
}

/// The curried function `acc |-> element |-> acc + element`.
pub fn add_folder() -> Node {
    Node::function(Node::function(Node::Add {
        first: Node::Parameter { depth: 1 }.into(),
        second: Node::Parameter { depth: 0 }.into(),
    }))
}

pub fn identity_function() -> Node {
    Node::function(Node::Parameter { depth: 0 })
}

/// A term that does not terminate: `(x |-> self x) 9`.
pub fn diverging_term() -> Node {
    Node::Call {
        callee: Node::function(Node::Call {
            callee: Node::FunctionSelf { depth: 0 }.into(),
            with: Node::Parameter { depth: 0 }.into(),
        })
        .into(),
        with: Node::Literal(integer(9)).into(),
    }
}

pub fn constant_predicate(value: Object) -> Node {
    Node::function(Node::Literal(value))
}

pub fn always_true_predicate() -> Node {
    constant_predicate(Atom::TRUE.into())
}

pub fn always_false_predicate() -> Node {
    constant_predicate(Atom::FALSE.into())
}

pub fn equals_two_predicate() -> Node {
    Node::function(Node::Equal {
        first: Node::Parameter { depth: 0 }.into(),
        second: Node::Literal(integer(2)).into(),
    })
}

/// Returns `false` for the first element and `undefined` for all others.
pub fn false_on_one_undefined_otherwise_predicate() -> Node {
    Node::function(Node::If {
        condition: Node::Equal {
            first: Node::Parameter { depth: 0 }.into(),
            second: Node::Literal(integer(1)).into(),
        }
        .into(),
        then: Node::Literal(false.into()).into(),
        otherwise: Node::Literal(Atom::UNDEFINED.into()).into(),
    })
}

/// Returns `undefined` for the first element and `false` for all others.
pub fn undefined_on_one_false_otherwise_predicate() -> Node {
    Node::function(Node::If {
        condition: Node::Equal {
            first: Node::Parameter { depth: 0 }.into(),
            second: Node::Literal(integer(1)).into(),
        }
        .into(),
        then: Node::Literal(Atom::UNDEFINED.into()).into(),
        otherwise: Node::Literal(false.into()).into(),
    })
}

pub fn every_result(composite: Node, predicate: Node) -> Object {
    evaluate(
        &Node::Every {
            composite: composite.into(),
            predicate: predicate.into(),
        },
        &BASE,
    )
    .into_object(&BASE)
}

pub fn any_result(composite: Node, predicate: Node) -> Object {
    evaluate(
        &Node::Any {
            composite: composite.into(),
            predicate: predicate.into(),
        },
        &BASE,
    )
    .into_object(&BASE)
}

/// Returns `true` for the first element and `undefined` for all others.
pub fn true_on_one_undefined_otherwise_predicate() -> Node {
    Node::function(Node::If {
        condition: Node::Equal {
            first: Node::Parameter { depth: 0 }.into(),
            second: Node::Literal(integer(1)).into(),
        }
        .into(),
        then: Node::Literal(true.into()).into(),
        otherwise: Node::Literal(Atom::UNDEFINED.into()).into(),
    })
}

/// Returns `undefined` for the first element and `true` for all others.
pub fn undefined_on_one_true_otherwise_predicate() -> Node {
    Node::function(Node::If {
        condition: Node::Equal {
            first: Node::Parameter { depth: 0 }.into(),
            second: Node::Literal(integer(1)).into(),
        }
        .into(),
        then: Node::Literal(Atom::UNDEFINED.into()).into(),
        otherwise: Node::Literal(true.into()).into(),
    })
}

pub fn and_apply_folder(predicate: Object) -> Node {
    Node::function(Node::function(Node::And {
        first: Node::Parameter { depth: 1 }.into(),
        second: Node::Call {
            callee: Node::Literal(predicate).into(),
            with: Node::Parameter { depth: 0 }.into(),
        }
        .into(),
    }))
}

pub fn every_by_and_fold(composite: Node, predicate: &Node) -> Object {
    let predicate = evaluate(predicate, &BASE).into_object(&BASE);

    evaluate(
        &Node::Fold {
            composite: composite.into(),
            folder: and_apply_folder(predicate).into(),
            initial: Node::Literal(Atom::TRUE.into()).into(),
        },
        &BASE,
    )
    .into_object(&BASE)
}

pub fn or_apply_folder(predicate: Object) -> Node {
    Node::function(Node::function(Node::Or {
        first: Node::Parameter { depth: 1 }.into(),
        second: Node::Call {
            callee: Node::Literal(predicate).into(),
            with: Node::Parameter { depth: 0 }.into(),
        }
        .into(),
    }))
}

pub fn any_by_or_fold(composite: Node, predicate: &Node) -> Object {
    let predicate = evaluate(predicate, &BASE).into_object(&BASE);

    evaluate(
        &Node::Fold {
            composite: composite.into(),
            folder: or_apply_folder(predicate).into(),
            initial: Node::Literal(Atom::FALSE.into()).into(),
        },
        &BASE,
    )
    .into_object(&BASE)
}
