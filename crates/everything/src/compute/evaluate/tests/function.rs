use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Construction, Object, Set},
};

use super::super::*;
use super::common::*;

fn annotated_function(body: &Node, annotation: Object) -> Object {
    Set::from([
        Construction::new(Atom::NODE_FUNCTION.into(), body.into_object()).into(),
        annotation,
    ])
    .into()
}

#[test]
fn function_evaluation_preserves_extra_properties() {
    let body = Node::Literal(integer(1));
    let annotation = Construction::new(Atom(42).into(), Atom(7).into()).into();
    let object = annotated_function(&body, annotation);
    let node = Node::new(&object, &BASE).unwrap();

    assert_eq!(evaluate(&node, &BASE).into_object(&BASE), object);
    assert_eq!(
        evaluate(&Node::Count(node.into()), &BASE).into_object(&BASE),
        integer(2)
    );
}

#[test]
fn atom_defined_function_materialises_into_a_set() {
    let atom = Atom(99);
    let body = Node::Literal(integer(1));
    let annotation = Construction::new(Atom(42).into(), Atom(7).into()).into();

    let mut knowledge = BASE.clone();
    knowledge.add_statement(Statement {
        atom,
        object: Construction::new(Atom::NODE_FUNCTION.into(), body.into_object()).into(),
    });
    knowledge.add_statement(Statement {
        atom,
        object: annotation,
    });

    let node = Node::new(&Object::Atom(atom), &knowledge).unwrap();
    let result = evaluate(&node, &knowledge);

    assert!(result.is_set());
    assert_eq!(
        result.into_object(&knowledge),
        Set::from([
            Construction::new(Atom::NODE_FUNCTION.into(), body.into_object()).into(),
            Construction::new(Atom(42).into(), Atom(7).into()).into(),
        ])
        .into()
    );
}

#[test]
fn self_reference_preserves_extra_properties() {
    let body = Node::FunctionSelf { depth: 0 };
    let annotation = Construction::new(Atom(42).into(), Atom(7).into()).into();
    let object = annotated_function(&body, annotation);
    let node = Node::new(&object, &BASE).unwrap();

    let call = Node::Call {
        callee: node.into(),
        with: Node::Literal(integer(1)).into(),
    };

    assert_eq!(evaluate(&call, &BASE).into_object(&BASE), object);
}
