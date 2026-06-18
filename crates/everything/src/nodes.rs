use everything_objects::Object;

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

#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Function(Object),
    Literal(Object),
    And(BinaryNode),
    FunctionSelf(u64),
    Parameter(u64),
    Count(Object),
    Query(Object),
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
    Knowledge,
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
    Less,
    PartialIf { then: Object, otherwise: Object },
    Call,
    PopContext,
    PartialUnwrapOr { default: Object },
}
