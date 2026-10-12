use crate::model::{Atom, Construction, Object, Set};

/// A node that has a left and a right hand side.
///
/// Used for multiple things, such as comparisons.
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryNode {
    pub left: Object,
    pub right: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MapNode {
    pub set: Object,
    pub mapper_function: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FilterNode {
    pub set: Object,
    pub filter_function: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfNode {
    pub condition: Object,
    pub then: Object,
    pub otherwise: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnwrapOrNode {
    pub set: Object,
    pub default: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallNode {
    /// The function/node getting called
    pub callee: Object,

    /// A node for the parameter.
    pub with: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct QueryExistsNode {
    pub subject: Object,
    pub tag: Object,
    pub value: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct QuerySubjectsNode {
    pub tag: Object,
    pub value: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct QueryTagsNode {
    pub subject: Object,
    pub value: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct QueryValuesNode {
    pub subject: Object,
    pub tag: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct QueryTagsAndValuesNode {
    pub subject: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct QuerySubjectsAndValuesNode {
    pub tag: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct QuerySubjectsAndTagsNode {
    pub value: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PredicateNode {
    pub set: Object,
    pub predicate: Object,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Function(Object),
    Literal(Object),
    And(BinaryNode),
    FunctionSelf(u32),
    Parameter(u32),
    Count(Object),
    QueryExists(QueryExistsNode),
    QuerySubjects(QuerySubjectsNode),
    QueryTags(QueryTagsNode),
    QueryValues(QueryValuesNode),
    QuerySubjectsAndTags(QuerySubjectsAndTagsNode),
    QuerySubjectsAndValues(QuerySubjectsAndValuesNode),
    QueryTagsAndValues(QueryTagsAndValuesNode),
    Equal(BinaryNode),
    Or(BinaryNode),
    Xor(BinaryNode),
    Not(Object),
    Add(BinaryNode),
    Union(BinaryNode),
    Map(MapNode),
    Filter(FilterNode),
    Less(BinaryNode),
    If(IfNode),
    UnwrapOr(UnwrapOrNode),
    Multiply(BinaryNode),
    Call(CallNode),
    IsAtom(Object),
    Every(PredicateNode),
    Any(PredicateNode),
    // Please also add new nodes to the array of nodes
    // in the tests (function `node_parsing()`).
}

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn new_node(node: Node) -> Object {
    match node {
        Node::IsAtom(inner) => {
            Set::new_move([Construction::new(Atom::NODE_IS_ATOM.into(), inner).into()]).into()
        }
        Node::Call(CallNode { callee, with }) => Set::new_move([
            Construction::new(Atom::NODE_CALL_CALLEE.into(), callee).into(),
            Construction::new(Atom::NODE_CALL_WITH.into(), with).into(),
        ])
        .into(),
        Node::Function(body) => {
            Set::new_move([Construction::new(Atom::NODE_FUNCTION.into(), body).into()]).into()
        }
        Node::Literal(literal) => {
            Set::new_move([Construction::new(Atom::NODE_LITERAL.into(), literal).into()]).into()
        }
        Node::And(BinaryNode { left, right }) => Set::new_move([
            Construction::new(Atom::NODE_AND_LEFT.into(), left).into(),
            Construction::new(Atom::NODE_AND_RIGHT.into(), right).into(),
        ])
        .into(),
        Node::FunctionSelf(depth) => Set::new_move([Construction::new(
            Atom::NODE_FUNCTION_SELF.into(),
            Object::new_integer(i128::from(depth)),
        )
        .into()])
        .into(),
        Node::Parameter(depth) => Set::new_move([Construction::new(
            Atom::NODE_PARAMETER.into(),
            Object::new_integer(i128::from(depth)),
        )
        .into()])
        .into(),
        Node::Count(object) => {
            Set::new_move([Construction::new(Atom::NODE_COUNT.into(), object).into()]).into()
        }
        Node::QueryExists(exists) => Set::new_move([
            Construction::new(Atom::NODE_QUERY_SUBJECT.into(), exists.subject).into(),
            Construction::new(Atom::NODE_QUERY_TAG.into(), exists.tag).into(),
            Construction::new(Atom::NODE_QUERY_VALUE.into(), exists.value).into(),
        ])
        .into(),
        Node::QuerySubjects(subjects) => Set::new_move([
            Construction::new(Atom::NODE_QUERY_TAG.into(), subjects.tag).into(),
            Construction::new(Atom::NODE_QUERY_VALUE.into(), subjects.value).into(),
        ])
        .into(),
        Node::QueryTags(query) => Set::new_move([
            Construction::new(Atom::NODE_QUERY_SUBJECT.into(), query.subject).into(),
            Construction::new(Atom::NODE_QUERY_VALUE.into(), query.value).into(),
        ])
        .into(),
        Node::QueryValues(query) => Set::new_move([
            Construction::new(Atom::NODE_QUERY_SUBJECT.into(), query.subject).into(),
            Construction::new(Atom::NODE_QUERY_TAG.into(), query.tag).into(),
        ])
        .into(),
        Node::QuerySubjectsAndTags(query) => {
            Set::new_move([Construction::new(Atom::NODE_QUERY_VALUE.into(), query.value).into()])
                .into()
        }
        Node::QueryTagsAndValues(query) => {
            Set::new_move([
                Construction::new(Atom::NODE_QUERY_SUBJECT.into(), query.subject).into(),
            ])
            .into()
        }
        Node::QuerySubjectsAndValues(query) => {
            Set::new_move([Construction::new(Atom::NODE_QUERY_TAG.into(), query.tag).into()]).into()
        }
        Node::Equal(BinaryNode { left, right }) => Set::new_move([
            Construction::new(Atom::NODE_EQUAL_LEFT.into(), left).into(),
            Construction::new(Atom::NODE_EQUAL_RIGHT.into(), right).into(),
        ])
        .into(),
        Node::Or(BinaryNode { left, right }) => Set::new_move([
            Construction::new(Atom::NODE_OR_LEFT.into(), left).into(),
            Construction::new(Atom::NODE_OR_RIGHT.into(), right).into(),
        ])
        .into(),
        Node::Xor(BinaryNode { left, right }) => Set::new_move([
            Construction::new(Atom::NODE_XOR_LEFT.into(), left).into(),
            Construction::new(Atom::NODE_XOR_RIGHT.into(), right).into(),
        ])
        .into(),
        Node::Not(node) => {
            Set::new_move([Construction::new(Atom::NODE_NOT.into(), node).into()]).into()
        }
        Node::Add(BinaryNode { left, right }) => Set::new_move([
            Construction::new(Atom::NODE_ADD_LEFT.into(), left).into(),
            Construction::new(Atom::NODE_ADD_RIGHT.into(), right).into(),
        ])
        .into(),
        Node::Union(BinaryNode { left, right }) => Set::new_move([
            Construction::new(Atom::NODE_UNION_LEFT.into(), left).into(),
            Construction::new(Atom::NODE_UNION_RIGHT.into(), right).into(),
        ])
        .into(),
        Node::Map(MapNode {
            set,
            mapper_function,
        }) => Set::new_move([
            Construction::new(Atom::NODE_MAP_SET.into(), set).into(),
            Construction::new(Atom::NODE_MAP_MAPPER.into(), mapper_function).into(),
        ])
        .into(),
        Node::Filter(FilterNode {
            set,
            filter_function,
        }) => Set::new_move([
            Construction::new(Atom::NODE_FILTER_SET.into(), set).into(),
            Construction::new(Atom::NODE_FILTER_FILTER.into(), filter_function).into(),
        ])
        .into(),
        Node::Every(PredicateNode { set, predicate }) => Set::new_move([
            Construction::new(Atom::NODE_EVERY_SET.into(), set).into(),
            Construction::new(Atom::NODE_EVERY_PREDICATE.into(), predicate).into(),
        ])
        .into(),
        Node::Any(PredicateNode { set, predicate }) => Set::new_move([
            Construction::new(Atom::NODE_ANY_SET.into(), set).into(),
            Construction::new(Atom::NODE_ANY_PREDICATE.into(), predicate).into(),
        ])
        .into(),
        Node::Less(BinaryNode { left, right }) => Set::new_move([
            Construction::new(Atom::NODE_LESS_LEFT.into(), left).into(),
            Construction::new(Atom::NODE_LESS_RIGHT.into(), right).into(),
        ])
        .into(),
        Node::If(IfNode {
            condition,
            otherwise,
            then,
        }) => Set::new_move([
            Construction::new(Atom::NODE_IF_CONDITION.into(), condition).into(),
            Construction::new(Atom::NODE_IF_THEN.into(), then).into(),
            Construction::new(Atom::NODE_IF_ELSE.into(), otherwise).into(),
        ])
        .into(),
        Node::UnwrapOr(UnwrapOrNode { set, default }) => Set::new_move([
            Construction::new(Atom::NODE_UNWRAP_OR_SET.into(), set).into(),
            Construction::new(Atom::NODE_UNWRAP_OR_DEFAULT.into(), default).into(),
        ])
        .into(),
        Node::Multiply(BinaryNode { left, right }) => Set::new_move([
            Construction::new(Atom::NODE_MULTIPLY_LEFT.into(), left).into(),
            Construction::new(Atom::NODE_MULTIPLY_RIGHT.into(), right).into(),
        ])
        .into(),
    }
}
