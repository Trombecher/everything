use num_bigint::BigInt;

use crate::{
    base::BASE,
    model::{Atom, Construction, Object, Set},
};

use super::super::*;

pub fn product_iterator<A: Clone, B>(
    a: impl Iterator<Item = A>,
    b: impl Iterator<Item = B> + Clone,
) -> impl Iterator<Item = (A, B)> {
    a.flat_map(move |a_item| b.clone().map(move |b_item| (a_item.clone(), b_item)))
}

pub fn pair(atom: Atom, value: Object) -> Object {
    Set::from([
        Construction::new(Atom::ATOM.into(), atom.into()).into(),
        Construction::new(Atom::VALUE.into(), value).into(),
    ])
    .into()
}

pub fn integer(value: i32) -> Object {
    Object::new_integer(BigInt::from(value))
}

pub fn integer_set() -> Set {
    Set::from([integer(1), integer(2), integer(3)])
}

/// The curried function `acc |-> element |-> acc + element`.
pub fn add_folder() -> Node {
    Node::Function {
        body: Node::Function {
            body: Node::Add {
                first: Node::Parameter { depth: 1 }.into(),
                second: Node::Parameter { depth: 0 }.into(),
            }
            .into(),
        }
        .into(),
    }
}

pub fn identity_function() -> Node {
    Node::Function {
        body: Node::Parameter { depth: 0 }.into(),
    }
}

pub fn constant_predicate(value: Object) -> Node {
    Node::Function {
        body: Node::Literal(value).into(),
    }
}

pub fn always_true_predicate() -> Node {
    constant_predicate(Atom::TRUE.into())
}

pub fn always_false_predicate() -> Node {
    constant_predicate(Atom::FALSE.into())
}

pub fn equals_two_predicate() -> Node {
    Node::Function {
        body: Node::Equal {
            first: Node::Parameter { depth: 0 }.into(),
            second: Node::Literal(integer(2)).into(),
        }
        .into(),
    }
}

/// Returns `false` for the first element and `undefined` for all others.
pub fn false_on_one_undefined_otherwise_predicate() -> Node {
    Node::Function {
        body: Node::If {
            condition: Node::Equal {
                first: Node::Parameter { depth: 0 }.into(),
                second: Node::Literal(integer(1)).into(),
            }
            .into(),
            then: Node::Literal(false.into()).into(),
            otherwise: Node::Literal(Atom::UNDEFINED.into()).into(),
        }
        .into(),
    }
}

/// Returns `undefined` for the first element and `false` for all others.
pub fn undefined_on_one_false_otherwise_predicate() -> Node {
    Node::Function {
        body: Node::If {
            condition: Node::Equal {
                first: Node::Parameter { depth: 0 }.into(),
                second: Node::Literal(integer(1)).into(),
            }
            .into(),
            then: Node::Literal(Atom::UNDEFINED.into()).into(),
            otherwise: Node::Literal(false.into()).into(),
        }
        .into(),
    }
}

pub fn every_result(set: Node, predicate: Node) -> Object {
    evaluate(
        &Node::Every {
            set: set.into(),
            predicate: predicate.into(),
        },
        &BASE,
    )
    .into_object(&BASE)
}

pub fn any_result(set: Node, predicate: Node) -> Object {
    evaluate(
        &Node::Any {
            set: set.into(),
            predicate: predicate.into(),
        },
        &BASE,
    )
    .into_object(&BASE)
}

/// Returns `true` for the first element and `undefined` for all others.
pub fn true_on_one_undefined_otherwise_predicate() -> Node {
    Node::Function {
        body: Node::If {
            condition: Node::Equal {
                first: Node::Parameter { depth: 0 }.into(),
                second: Node::Literal(integer(1)).into(),
            }
            .into(),
            then: Node::Literal(true.into()).into(),
            otherwise: Node::Literal(Atom::UNDEFINED.into()).into(),
        }
        .into(),
    }
}

/// Returns `undefined` for the first element and `true` for all others.
pub fn undefined_on_one_true_otherwise_predicate() -> Node {
    Node::Function {
        body: Node::If {
            condition: Node::Equal {
                first: Node::Parameter { depth: 0 }.into(),
                second: Node::Literal(integer(1)).into(),
            }
            .into(),
            then: Node::Literal(Atom::UNDEFINED.into()).into(),
            otherwise: Node::Literal(true.into()).into(),
        }
        .into(),
    }
}

pub fn and_apply_folder(predicate: Object) -> Node {
    Node::Function {
        body: Node::Function {
            body: Node::And {
                first: Node::Parameter { depth: 1 }.into(),
                second: Node::Call {
                    callee: Node::Literal(predicate).into(),
                    with: Node::Parameter { depth: 0 }.into(),
                }
                .into(),
            }
            .into(),
        }
        .into(),
    }
}

pub fn every_by_and_fold(set: Node, predicate: &Node) -> Object {
    let predicate = evaluate(predicate, &BASE).into_object(&BASE);

    evaluate(
        &Node::Fold {
            set: set.into(),
            folder: and_apply_folder(predicate).into(),
            initial: Node::Literal(Atom::TRUE.into()).into(),
        },
        &BASE,
    )
    .into_object(&BASE)
}

pub fn or_apply_folder(predicate: Object) -> Node {
    Node::Function {
        body: Node::Function {
            body: Node::Or {
                first: Node::Parameter { depth: 1 }.into(),
                second: Node::Call {
                    callee: Node::Literal(predicate).into(),
                    with: Node::Parameter { depth: 0 }.into(),
                }
                .into(),
            }
            .into(),
        }
        .into(),
    }
}

pub fn any_by_or_fold(set: Node, predicate: &Node) -> Object {
    let predicate = evaluate(predicate, &BASE).into_object(&BASE);

    evaluate(
        &Node::Fold {
            set: set.into(),
            folder: or_apply_folder(predicate).into(),
            initial: Node::Literal(Atom::FALSE.into()).into(),
        },
        &BASE,
    )
    .into_object(&BASE)
}
