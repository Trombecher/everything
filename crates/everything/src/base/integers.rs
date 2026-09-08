use std::sync::LazyLock;

use everything_objects::{Abstract, Composite, Object};

use crate::{
    ext::AbstractExt,
    nodes::{
        BinaryNode, CallNode, Node, PredicateNode, QueryTagsAndValuesNode, QueryValuesNode,
        UnwrapOrNode,
    },
};

// If this object was inlined, the formatter would not work.
fn inner_constraint() -> Object {
    Node::Every(PredicateNode {
        set: Node::QueryTagsAndValues(QueryTagsAndValuesNode {
            subject: Node::Parameter(0).into(),
        })
        .into(),
        predicate: Node::Function(
            Node::And(BinaryNode {
                left: Node::Or(BinaryNode {
                    left: Node::Equal(BinaryNode {
                        left: Node::UnwrapOr(UnwrapOrNode {
                            set: Node::QueryValues(QueryValuesNode {
                                subject: Node::Parameter(0).into(),
                                tag: Abstract::STATEMENT_TAG.into(),
                            })
                            .into(),
                            default: Composite::Empty.into(),
                        })
                        .into(),
                        right: Abstract::SUCCESSOR_OF.into(),
                    })
                    .into(),
                    right: Node::Equal(BinaryNode {
                        left: Node::UnwrapOr(UnwrapOrNode {
                            set: Node::QueryValues(QueryValuesNode {
                                subject: Node::Parameter(0).into(),
                                tag: Abstract::STATEMENT_TAG.into(),
                            })
                            .into(),
                            default: Composite::Empty.into(),
                        })
                        .into(),
                        right: Abstract::PREDECESSOR_OF.into(),
                    })
                    .into(),
                })
                .into(),
                // Each value is an integer.
                right: Node::Call(CallNode {
                    // Not the predicate function, but the outer function (IS_INTEGER).
                    callee: Node::FunctionSelf(1).into(),
                    with: Node::UnwrapOr(UnwrapOrNode {
                        set: Node::QueryValues(QueryValuesNode {
                            subject: Node::Parameter(0).into(),
                            tag: Abstract::STATEMENT_VALUE.into(),
                        })
                        .into(),
                        default: Composite::Empty.into(),
                    })
                    .into(),
                })
                .into(),
            })
            .into(),
        )
        .into(),
    })
    .into()
}

/// A function that checks whether the input object is an integer.
pub static IS_INTEGER: LazyLock<Object> = LazyLock::new(|| {
    Node::Function(
        Node::Or(BinaryNode {
            // is zero or...
            left: Node::Equal(BinaryNode {
                left: Node::Parameter(0).into(),
                right: Abstract::ZERO.into(),
            })
            .into(),
            right: Node::And(BinaryNode {
                // ...is not abstract and...
                left: Node::Not(Node::IsAbstract(Node::Parameter(0).into()).into()).into(),
                right: Node::And(BinaryNode {
                    // ...either has exactly one successor or exactly one predecessor and...
                    left: Node::Xor(BinaryNode {
                        left: Node::Equal(BinaryNode {
                            left: Node::Count(
                                Node::QueryValues(QueryValuesNode {
                                    subject: Node::Parameter(0).into(),
                                    tag: Abstract::SUCCESSOR_OF.into(),
                                })
                                .into(),
                            )
                            .into(),
                            right: Object::new_integer(1),
                        })
                        .into(),
                        right: Node::Equal(BinaryNode {
                            left: Node::Count(
                                Node::QueryValues(QueryValuesNode {
                                    subject: Node::Parameter(0).into(),
                                    tag: Abstract::PREDECESSOR_OF.into(),
                                })
                                .into(),
                            )
                            .into(),
                            right: Object::new_integer(1),
                        })
                        .into(),
                    })
                    .into(),
                    // ...every property's tag is either successor or predecessor
                    // (closed definition) and every associated tag is an integer.
                    right: inner_constraint(),
                })
                .into(),
            })
            .into(),
        })
        .into(),
    )
    .into()
});
