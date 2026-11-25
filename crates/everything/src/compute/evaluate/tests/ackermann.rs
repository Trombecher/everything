use num_bigint::BigInt;

use crate::{
    base::BASE,
    model::{Atom, Object},
};

use super::super::*;
use super::common::*;

/// The reference implementation of the Ackermann function.
fn reference(m: u32, n: u32) -> u32 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        reference(m - 1, 1)
    } else {
        reference(m - 1, reference(m, n - 1))
    }
}

/// The Ackermann function expressed in the computation system:
///
/// ```text
/// ackermann = m -> n ->
///     if m == 0 then n + 1
///     else if n == 0 then ackermann (m - 1) 1
///     else ackermann (m - 1) (ackermann m (n - 1))
/// ```
///
/// There is no subtraction node, but predecessor is only needed for positive
/// numbers, so it is expressed as addition of `-1`.
///
/// Call arguments are lazy but not shared, so referencing `n` several times
/// would recompute `ackermann m (n - 1)` several times and make the evaluation
/// exponentially slow. The `otherwise` branch therefore forces `n` once by
/// folding over the singleton set `%n%`: fold elements are already evaluated
/// values and can be referenced for free.
fn ackermann() -> Node {
    let predecessor = |value: Node| Node::Add {
        first: value.into(),
        second: Node::Literal(BigInt::from(-1).into()).into(),
    };

    let call_ackermann = |callee: Node, first: Node, second: Node| Node::Call {
        callee: Node::Call {
            callee: callee.into(),
            with: first.into(),
        }
        .into(),
        with: second.into(),
    };

    // Inside the body of `n -> ...`, `n` is the parameter at depth 0 and `m`
    // is the parameter at depth 1.
    let m = || Node::Parameter { depth: 1 };
    let n = || Node::Parameter { depth: 0 };

    // Inside the body of the folder `acc -> element -> ...`, `element` is the
    // parameter at depth 0 and `m` is the parameter at depth 3: the two folder
    // functions are nested inside the two Ackermann functions.
    let element = || Node::Parameter { depth: 0 };
    let outer_m = || Node::Parameter { depth: 3 };
    let outer_self_reference = || Node::FunctionSelf { depth: 3 };

    let folder_body = Node::If {
        condition: Node::Equal {
            first: element().into(),
            second: Node::Literal(integer(0)).into(),
        }
        .into(),
        then: call_ackermann(
            outer_self_reference(),
            predecessor(outer_m()),
            Node::Literal(integer(1)),
        )
        .into(),
        otherwise: call_ackermann(
            outer_self_reference(),
            predecessor(outer_m()),
            call_ackermann(outer_self_reference(), outer_m(), predecessor(element())),
        )
        .into(),
    };

    let body = Node::If {
        condition: Node::Equal {
            first: m().into(),
            second: Node::Literal(integer(0)).into(),
        }
        .into(),
        then: Node::Add {
            first: n().into(),
            second: Node::Literal(integer(1)).into(),
        }
        .into(),
        otherwise: Node::SetFold {
            set: Node::NewSet {
                items: vec![n().into()],
            }
            .into(),
            folder: Node::function(Node::function(folder_body)).into(),
            initial: Node::Literal(Atom::UNDEFINED.into()).into(),
        }
        .into(),
    };

    Node::function(Node::function(body))
}

fn evaluate_ackermann(m: u32, n: u32) -> Object {
    let call = Node::Call {
        callee: Node::Call {
            callee: ackermann().into(),
            with: Node::Literal(BigInt::from(m).into()).into(),
        }
        .into(),
        with: Node::Literal(BigInt::from(n).into()).into(),
    };

    evaluate(&call, &BASE).into_object(&BASE)
}

fn check(m: u32, n: u32) {
    assert_eq!(
        evaluate_ackermann(m, n),
        BigInt::from(reference(m, n)).into(),
        "ackermann({m}, {n})"
    );
}

#[test]
fn ackermann_matches_the_reference_implementation() {
    // The classic small values, including the first step of the fourth row.
    const SMALL: [(u32, u32); 16] = [
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (1, 0),
        (1, 1),
        (1, 2),
        (1, 3),
        (2, 0),
        (2, 1),
        (2, 2),
        (2, 3),
        (2, 4),
        (3, 0),
        (3, 1),
        (4, 0),
    ];

    // These grow quickly and are too slow for the unoptimized interpreter.
    const LARGE: [(u32, u32); 3] = [(3, 2), (3, 3), (3, 4)];

    for (m, n) in SMALL {
        check(m, n);
    }

    if !cfg!(debug_assertions) {
        for (m, n) in LARGE {
            check(m, n);
        }
    }
}
