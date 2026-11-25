use crate::{
    base::BASE,
    model::{Atom, Composite, Construction, Object},
};

use super::super::*;
use super::common::*;

#[test]
fn union_requires_composites() {
    let atom = Node::Literal(Object::Atom(Atom(42)));
    let empty_composite = Node::Literal(Object::Composite(Composite::empty()));
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());

    for invalid in [atom, construction] {
        assert_eq!(
            evaluate(
                &Node::Union {
                    first: invalid.into(),
                    second: empty_composite.clone().into(),
                },
                &BASE,
            )
            .into_object(&BASE),
            Atom::UNDEFINED.into()
        );
    }

    let union = evaluate(
        &Node::Union {
            first: identity_function().into(),
            second: empty_composite.into(),
        },
        &BASE,
    );

    assert!(union.is_composite());
    assert_eq!(union.iter(&BASE).count(), 1);
}

#[test]
fn union_iterates_each_element_once() {
    let union = Node::Union {
        first: Node::Literal(Object::Composite(integer_composite())).into(),
        second: Node::Literal(Object::Composite(integer_composite())).into(),
    };

    let result = evaluate(&union, &BASE);

    assert_eq!(
        result.iter(&BASE).collect::<Vec<_>>(),
        vec![integer(1), integer(2), integer(3)]
    );
    assert_eq!(result.size(&BASE), 3);
}
