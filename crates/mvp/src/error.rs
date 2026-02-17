use crate::objects::Id;

pub enum Error {
    Missing(AssociationForm),
    Found(AssociationForm),
}

pub enum IdPattern {
    Specific(Id),
    Some,
}

pub struct AssociationForm {
    pub tag: IdPattern,
    pub target: IdPattern,
    pub value: IdPattern,
}
