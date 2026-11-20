use crate::{
    base::BASE,
    model::{Atom, Construction, Object, Set},
};

use super::super::*;
use super::common::*;

#[test]
fn union_requires_sets() {
    let atom = Node::Literal(Object::Atom(Atom(42)));
    let empty_set = Node::Literal(Object::Set(Set::empty()));
    let construction = Node::Literal(Construction::new(Atom(1).into(), Atom(2).into()).into());

    for invalid in [atom, construction] {
        assert_eq!(
            evaluate(
                &Node::Union {
                    first: invalid.into(),
                    second: empty_set.clone().into(),
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
            second: empty_set.into(),
        },
        &BASE,
    );

    assert!(union.is_set());
    assert_eq!(union.iter(&BASE).count(), 1);
}

#[test]
fn union_iterates_each_element_once() {
    let union = Node::Union {
        first: Node::Literal(Object::Set(integer_set())).into(),
        second: Node::Literal(Object::Set(integer_set())).into(),
    };

    let result = evaluate(&union, &BASE);

    assert_eq!(
        result.iter(&BASE).collect::<Vec<_>>(),
        vec![integer(1), integer(2), integer(3)]
    );
    assert_eq!(result.size(&BASE), 3);
}
