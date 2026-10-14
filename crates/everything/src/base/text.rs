use std::sync::LazyLock;

use everything_objects::{Abstract, Composite, Object};

use crate::nodes::{BinaryNode, CallNode, Node, QueryValuesNode, UnwrapOrNode};

pub static IS_TEXT: LazyLock<Object> = LazyLock::new(|| {
    Node::Or(BinaryNode {
        left: Node::Equal(BinaryNode {
            left: Node::Parameter(0).into(),
            right: Composite::Empty.into(),
        })
        .into(),
        right: Node::And(BinaryNode {
            // the list item is a character and...
            left: Node::QueryValues(QueryValuesNode {
                subject: Node::UnwrapOr(UnwrapOrNode {
                    set: Node::QueryValues(QueryValuesNode {
                        subject: Node::Parameter(0).into(),
                        tag: Abstract::LIST_ITEM.into(),
                    })
                    .into(),
                    default: Composite::Empty.into(),
                })
                .into(),
                tag: Abstract::CODE_POINT.into(),
            })
            .into(),
            // the tail is text
            right: Node::Call(CallNode {
                callee: Node::FunctionSelf(0).into(),
                with: Node::UnwrapOr(UnwrapOrNode {
                    set: Node::QueryValues(QueryValuesNode {
                        subject: Node::Parameter(0).into(),
                        tag: Abstract::LIST_TAIL.into(),
                    })
                    .into(),
                    // dummy value
                    default: Abstract::ZERO.into(),
                })
                .into(),
            })
            .into(),
        })
        .into(),
    })
    .into()
});
