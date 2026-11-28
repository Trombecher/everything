use crate::{base::BASE, model::Object};

use super::common::*;
use crate::compute::*;

fn addition() -> Node {
    Node::Add {
        first: Node::Literal(integer(1)).into(),
        second: Node::Literal(integer(2)).into(),
    }
}

fn addition_with_shared_parameter() -> Node {
    Node::Call {
        callee: Node::function(Node::Add {
            first: Node::Parameter { depth: 0 }.into(),
            second: Node::Parameter { depth: 0 }.into(),
        })
        .into(),
        with: Node::Literal(integer(21)).into(),
    }
}

fn add_call() -> Node {
    Node::Call {
        callee: Node::Call {
            callee: add_folder().into(),
            with: Node::Literal(integer(1)).into(),
        }
        .into(),
        with: Node::Literal(integer(2)).into(),
    }
}

fn run_in_steps(node: &Node, budget: u64) -> Object {
    let mut evaluator = Evaluator::new(node, &BASE);

    loop {
        match evaluator.run(budget) {
            Ok(result) => return result.into_object(&BASE),
            Err(next) => evaluator = next,
        }
    }
}

#[test]
fn a_large_budget_produces_the_ordinary_result() {
    let node = addition();

    let Ok(result) = Evaluator::new(&node, &BASE).run(u64::MAX) else {
        panic!("evaluation did not finish");
    };

    assert_eq!(
        result.into_object(&BASE),
        evaluate(&node, &BASE).into_object(&BASE),
    );
}

#[test]
fn stepping_one_task_at_a_time_produces_the_ordinary_result() {
    let node = add_call();

    assert_eq!(
        run_in_steps(&node, 1),
        evaluate(&node, &BASE).into_object(&BASE),
    );
}

#[test]
fn shared_parameters_survive_a_pause() {
    let node = addition_with_shared_parameter();

    assert_eq!(
        run_in_steps(&node, 1),
        evaluate(&node, &BASE).into_object(&BASE),
    );
}

#[test]
fn a_zero_budget_pauses_without_running_a_step() {
    let node = addition();

    let Err(evaluator) = Evaluator::new(&node, &BASE).run(0) else {
        panic!("evaluation unexpectedly finished");
    };

    assert_eq!(evaluator.steps(), 0);
    assert!(!evaluator.is_complete());
}

#[test]
fn the_budget_is_an_exact_step_limit() {
    let mut evaluator = Evaluator::new(&addition(), &BASE);
    let mut steps = 0;

    loop {
        steps += 1;

        match evaluator.run(1) {
            Ok(_) => break,
            Err(next) => evaluator = next,
        }
    }

    let Err(evaluator) = Evaluator::new(&addition(), &BASE).run(steps - 1) else {
        panic!("evaluation unexpectedly finished");
    };

    assert_eq!(evaluator.steps(), steps - 1);
    assert!(Evaluator::new(&addition(), &BASE).run(steps).is_ok());
}

#[test]
fn a_diverging_evaluation_pauses_instead_of_hanging() {
    let node = diverging_term();
    let mut evaluator = Evaluator::new(&node, &BASE);

    for expected_steps in [100, 200, 300] {
        evaluator = match evaluator.run(100) {
            Ok(_) => panic!("diverging evaluation unexpectedly finished"),
            Err(evaluator) => evaluator,
        };

        assert_eq!(evaluator.steps(), expected_steps);
        assert!(!evaluator.is_complete());
    }
}
