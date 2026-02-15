use crate::objects::Id;

#[derive(Clone, PartialEq, Debug)]
pub struct Association {
    pub target: Id,
    pub tag: Id,
    pub value: Id,
}
