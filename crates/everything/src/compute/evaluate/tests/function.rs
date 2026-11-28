use crate::{
    base::BASE,
    knowledge::Statement,
    model::{Atom, Composite, Construction, Object},
};

use super::common::*;
use crate::compute::*;

fn annotated_function(body: &Node, annotation: Object) -> Object {
    Composite::from([
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
        evaluate(&Node::SetCount(node.into()), &BASE).into_object(&BASE),
        integer(0)
    );
}

#[test]
fn function_can_inspect_annotations_in_its_own_body() {
    // The function's body retrieves its own body object through `function_self`
    // and checks whether that object carries an annotation. If capture drops
    // the annotation, the function observes a different body and returns
    // `false`.
    let annotation_constructor = Atom(42);
    let annotation: Object =
        Construction::new(annotation_constructor.into(), Atom(7).into()).into();

    // Query the value of `@42 ·· ...` on the function self and test whether
    // the query result has an item.
    let query = Composite::from([
        Construction::new(
            Atom::NODE_CONSTRUCTION_VALUES_OBJECT.into(),
            Node::FunctionSelf { depth: 0 }.into_object(),
        )
        .into(),
        Construction::new(
            Atom::NODE_CONSTRUCTION_VALUES_PROPERTY_CONSTRUCTOR.into(),
            Node::Literal(annotation_constructor.into()).into_object(),
        )
        .into(),
    ]);

    let body =
        Composite::from([Construction::new(Atom::NODE_HAS_A_SET_ITEM.into(), query.into()).into()])
            .into();

    let function = Composite::from([
        Construction::new(Atom::NODE_FUNCTION.into(), body).into(),
        annotation,
    ])
    .into();
    let function = Node::new(&function, &BASE).unwrap();
    let call = Node::Call {
        callee: function.into(),
        with: Node::Literal(integer(1)).into(),
    };

    assert_eq!(evaluate(&call, &BASE).into_object(&BASE), Atom::TRUE.into());
}

#[test]
fn atom_defined_function_materialises_into_a_composite() {
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

    assert!(result.is_composite());
    assert_eq!(
        result.into_object(&knowledge),
        Composite::from([
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
