use everything_structures::Object;

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

#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Computed(Object),
    Literal(Object),
    And(BinaryNode),
    FunctionSelf(u128),
    Parameter(u128),
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
}

#[derive(Debug)]
pub enum Task {
    Eval(Object),
    PartialAnd { right: Object },
    ToBoolean,
    Count,
    QueryValues,
    QuerySubjectsAxiomatically,
    QuerySubjectsAndValuesAxiomatically,
    QueryExists,
    Equal,
    PartialOr { right: Object },
    Xor,
    Not,
    Add,
    Union,
    Map,
    Filter,
    Less,
    PartialIf { then: Object, otherwise: Object },
    PopContext,
}
