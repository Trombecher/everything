use crate::associations::Association;

#[derive(Clone, PartialEq, Debug)]
pub struct Structure(pub Vec<Association>);
