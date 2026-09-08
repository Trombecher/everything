use everything_objects::Object;

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
    Statements,
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
    IsAbstract(Object),
    Every(PredicateNode),
    Any(PredicateNode),
    // Please also add new nodes to the array of nodes
    // in the tests (function `node_parsing()`).
}

#[derive(Debug)]
pub enum Task {
    Eval(Object),
    PartialAnd { right: Object },
    ToBoolean,
    Count,
    QueryValues,
    QuerySubjects,
    QuerySubjectsAndValues,
    QueryTagsAndValues,
    QuerySubjectsAndTags,
    QueryTags,
    QueryExists,
    Equal,
    PartialOr { right: Object },
    Xor,
    Not,
    Add,
    Multiply,
    Union,
    Map,
    Filter,
    Every,
    Any,
    Less,
    PartialIf { then: Object, otherwise: Object },
    Call,
    PopContext,
    PartialUnwrapOr { default: Object },
    IsAbstract,
}
