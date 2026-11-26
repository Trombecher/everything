use std::sync::Arc;

use crate::{
    base::BASE,
    compute::{Argument, EvaluationContext, FunctionContext, LazyObject, Thunk},
    model::{Atom, Composite, Construction, Object},
};

use super::*;

fn annotated_function() -> (Object, Node) {
    let object = Composite::from([
        Construction::new(
            Atom::NODE_FUNCTION.into(),
            Node::Literal(Atom(1).into()).into_object(),
        )
        .into(),
        Construction::new(Atom(42).into(), Atom(7).into()).into(),
    ])
    .into();

    let node = Node::new(&object, &BASE).unwrap();

    (object, node)
}

#[test]
fn captured_function_property_preserves_extra_properties() {
    let (object, node) = annotated_function();

    let mut context = EvaluationContext::default();
    context.push(FunctionContext {
        function: Node::function(Node::Literal(Atom(1).into())),
        parameter: Argument::Value(LazyObject::Node(node)),
    });

    let inner = Node::function(Node::Parameter { depth: 1 });
    let captured = capture(&inner, 0, &context, &BASE);

    let Node::Function { body, .. } = captured else {
        panic!("expected a function");
    };

    let Node::Literal(captured_object) = body.as_ref() else {
        panic!("expected a literal");
    };

    assert_eq!(*captured_object, object);
}

#[test]
fn capture_preserves_annotations_of_transformed_nodes() {
    let annotation: Object = Construction::new(Atom(42).into(), Atom(7).into()).into();

    let object = Composite::from([
        Construction::new(
            Atom::NODE_ADD_FIRST.into(),
            Node::Parameter { depth: 0 }.into_object(),
        )
        .into(),
        Construction::new(
            Atom::NODE_ADD_SECOND.into(),
            Node::Literal(Atom(1).into()).into_object(),
        )
        .into(),
        annotation.clone(),
    ])
    .into();

    let node = Node::new(&object, &BASE).unwrap();

    let mut context = EvaluationContext::default();
    context.push(FunctionContext {
        function: Node::function(Node::Literal(Atom(1).into())),
        parameter: Argument::Value(LazyObject::Eager(Atom(2).into())),
    });

    let captured = capture(&node, 0, &context, &BASE);

    let expected = Composite::from([
        Construction::new(
            Atom::NODE_ADD_FIRST.into(),
            Node::Literal(Atom(2).into()).into_object(),
        )
        .into(),
        Construction::new(
            Atom::NODE_ADD_SECOND.into(),
            Node::Literal(Atom(1).into()).into_object(),
        )
        .into(),
        annotation,
    ])
    .into();

    assert_eq!(captured.into_object(), expected);
}

#[test]
fn capture_substitutes_an_unevaluated_argument_as_a_shared_thunk() {
    let expression = Node::Add {
        first: Node::Literal(Atom(1).into()).into(),
        second: Node::Literal(Atom(2).into()).into(),
    };
    let thunk = Arc::new(Thunk::new(expression.clone()));

    let mut context = EvaluationContext::default();
    context.push(FunctionContext {
        function: Node::function(Node::Literal(Atom(1).into())),
        parameter: Argument::Unevaluated(Arc::clone(&thunk)),
    });

    let inner = Node::function(Node::Parameter { depth: 1 });
    let captured = capture(&inner, 0, &context, &BASE);

    let Node::Function { body, .. } = captured else {
        panic!("expected a function");
    };

    let Node::Thunk(captured_thunk) = body.as_ref() else {
        panic!("expected a shared thunk");
    };

    assert!(Arc::ptr_eq(captured_thunk, &thunk));
    assert_eq!(body.into_object(), expression.into_object());
}

#[test]
fn capture_substitutes_an_evaluated_thunk_as_its_value() {
    let expression = Node::Add {
        first: Node::Literal(Atom(1).into()).into(),
        second: Node::Literal(Atom(2).into()).into(),
    };
    let thunk = Arc::new(Thunk::new(expression));
    thunk.cache(LazyObject::Eager(Atom(3).into()));

    let mut context = EvaluationContext::default();
    context.push(FunctionContext {
        function: Node::function(Node::Literal(Atom(1).into())),
        parameter: Argument::Unevaluated(thunk),
    });

    let inner = Node::function(Node::Parameter { depth: 1 });
    let captured = capture(&inner, 0, &context, &BASE);

    let Node::Function { body, .. } = captured else {
        panic!("expected a function");
    };

    assert_eq!(body.as_ref(), &Node::Literal(Atom(3).into()));
}

#[test]
fn capture_transforms_both_has_property_children() {
    let mut context = EvaluationContext::default();
    context.push(FunctionContext {
        function: Node::function(Node::Literal(Atom(1).into())),
        parameter: Argument::Value(LazyObject::Eager(Atom(2).into())),
    });

    let node = Node::HasProperty {
        subject: Node::Parameter { depth: 0 }.into(),
        property: Node::Parameter { depth: 0 }.into(),
    };
    let captured = capture(&node, 0, &context, &BASE);

    let literal = Node::Literal(Atom(2).into());

    assert_eq!(
        captured,
        Node::HasProperty {
            subject: literal.clone().into(),
            property: literal.into(),
        }
    );
}

#[test]
fn capture_transforms_all_new_composite_properties() {
    let mut context = EvaluationContext::default();
    context.push(FunctionContext {
        function: Node::function(Node::Literal(Atom(1).into())),
        parameter: Argument::Value(LazyObject::Eager(Atom(2).into())),
    });

    let node = Node::NewComposite {
        properties: vec![
            Node::Parameter { depth: 0 }.into(),
            Node::Literal(Atom(3).into()).into(),
        ],
    };
    let captured = capture(&node, 0, &context, &BASE);

    assert_eq!(
        captured,
        Node::NewComposite {
            properties: vec![
                Node::Literal(Atom(2).into()).into(),
                Node::Literal(Atom(3).into()).into(),
            ],
        }
    );
}

#[test]
fn capture_argument_closes_free_parameters_to_undefined() {
    let captured = capture_argument(
        &Node::Parameter { depth: 0 },
        &EvaluationContext::default(),
        &BASE,
    );

    assert_eq!(captured, Node::Literal(Atom::UNDEFINED.into()));
}

#[test]
fn capture_argument_closes_free_function_self_references_to_undefined() {
    let captured = capture_argument(
        &Node::FunctionSelf { depth: 0 },
        &EvaluationContext::default(),
        &BASE,
    );

    assert_eq!(captured, Node::Literal(Atom::UNDEFINED.into()));
}

#[test]
fn capture_argument_keeps_parameters_of_inner_functions() {
    let function = Node::function(Node::Parameter { depth: 0 });
    let captured = capture_argument(&function, &EvaluationContext::default(), &BASE);

    assert_eq!(captured, function);
}
