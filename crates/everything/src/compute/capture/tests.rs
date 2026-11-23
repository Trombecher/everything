use crate::{
    base::BASE,
    compute::{EvaluationContext, FunctionContext, LazyObject},
    model::{Atom, Construction, Object, Set},
};

use super::*;

fn annotated_function() -> (Object, Node) {
    let object = Set::from([
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
        parameter: LazyObject::Node(node),
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
