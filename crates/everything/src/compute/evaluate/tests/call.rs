use num_bigint::BigInt;

use crate::{
    base::BASE,
    model::{Atom, Composite, Object},
};

use super::super::*;
use super::common::*;

fn evaluate_with_timeout(node: &Node) -> Object {
    let node = node.clone();
    let knowledge = BASE.clone();
    let (sender, receiver) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let result = evaluate(&node, &knowledge).into_object(&knowledge);

        let _ = sender.send(result);
    });

    receiver
        .recv_timeout(std::time::Duration::from_secs(5))
        .expect("evaluation did not terminate")
}

#[test]
fn curried_add_function() {
    let first_call = Node::Call {
        callee: add_folder().into(),
        with: Node::Literal(integer(0)).into(),
    };
    let second_call = Node::Call {
        callee: first_call.into(),
        with: Node::Literal(integer(5)).into(),
    };

    assert_eq!(evaluate(&second_call, &BASE).into_object(&BASE), integer(5));
}

#[test]
fn call_rejects_a_deferred_set_callee() {
    let function = identity_function().into_object();
    let empty_set = Node::Literal(Object::Composite(Composite::empty()));
    let callee = Node::SetUnion {
        first: Node::Literal(function).into(),
        second: empty_set.into(),
    };

    assert_eq!(
        evaluate(
            &Node::Call {
                callee: callee.into(),
                with: Node::Literal(integer(5)).into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn call_with_undefined_callee_is_undefined() {
    assert_eq!(
        evaluate(
            &Node::Call {
                callee: Node::Literal(Atom::UNDEFINED.into()).into(),
                with: Node::Literal(integer(1)).into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn call_rejects_composites() {
    let union = Node::SetUnion {
        first: Node::Literal(Object::Composite(Composite::empty())).into(),
        second: Node::Literal(Object::Composite(Composite::empty())).into(),
    };

    assert_eq!(
        evaluate(
            &Node::Call {
                callee: union.into(),
                with: Node::Literal(BigInt::from(1).into()).into(),
            },
            &BASE,
        )
        .into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn used_argument_is_evaluated() {
    let call = Node::Call {
        callee: identity_function().into(),
        with: Node::Add {
            first: Node::Literal(integer(1)).into(),
            second: Node::Literal(integer(2)).into(),
        }
        .into(),
    };

    assert_eq!(evaluate(&call, &BASE).into_object(&BASE), integer(3));
}

#[test]
fn captured_argument_is_evaluated_when_the_closure_is_called() {
    let function = Node::function(Node::function(Node::Parameter { depth: 1 }));
    let partially_applied = Node::Call {
        callee: function.into(),
        with: Node::Literal(integer(1)).into(),
    };
    let call = Node::Call {
        callee: partially_applied.into(),
        with: Node::Literal(integer(2)).into(),
    };

    assert_eq!(evaluate(&call, &BASE).into_object(&BASE), integer(1));
}

#[test]
fn unused_argument_is_not_evaluated() {
    let call = Node::Call {
        callee: Node::function(Node::Literal(integer(1))).into(),
        with: diverging_term().into(),
    };

    assert_eq!(evaluate_with_timeout(&call), integer(1));
}

#[test]
fn captured_unused_argument_in_an_unselected_branch_is_not_evaluated() {
    let function = Node::function(Node::function(Node::If {
        condition: Node::Literal(false.into()).into(),
        // x
        then: Node::Parameter { depth: 1 }.into(),
        otherwise: Node::Literal(integer(1)).into(),
    }));
    let partially_applied = Node::Call {
        callee: function.into(),
        with: diverging_term().into(),
    };
    let call = Node::Call {
        callee: partially_applied.into(),
        with: Node::Literal(integer(2)).into(),
    };

    assert_eq!(evaluate_with_timeout(&call), integer(1));
}

#[test]
fn unused_argument_in_an_unselected_branch_is_not_evaluated() {
    let function = Node::function(Node::function(Node::If {
        condition: Node::Literal(false.into()).into(),
        // y
        then: Node::Parameter { depth: 0 }.into(),
        // x
        otherwise: Node::Parameter { depth: 1 }.into(),
    }));
    let partially_applied = Node::Call {
        callee: function.into(),
        with: Node::Literal(integer(1)).into(),
    };
    let call = Node::Call {
        callee: partially_applied.into(),
        with: diverging_term().into(),
    };

    assert_eq!(evaluate_with_timeout(&call), integer(1));
}

#[test]
fn free_parameter_in_an_argument_is_closed_to_undefined() {
    let call = Node::Call {
        callee: identity_function().into(),
        with: Node::Parameter { depth: 0 }.into(),
    };

    assert_eq!(
        evaluate(&call, &BASE).into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}

#[test]
fn free_function_self_in_an_argument_is_closed_to_undefined() {
    let call = Node::Call {
        callee: identity_function().into(),
        with: Node::FunctionSelf { depth: 0 }.into(),
    };

    assert_eq!(
        evaluate(&call, &BASE).into_object(&BASE),
        Atom::UNDEFINED.into()
    );
}
