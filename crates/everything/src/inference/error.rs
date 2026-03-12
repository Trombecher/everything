use everything_structures::{Object, Property};

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    FoundPropertyOn { subject: Object, property: Property },
    MissingPropertyOn { subject: Object, form: PropertyForm },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyForm {
    Exact(Property),
    SomeTagAndExactValue(Object),
    SomeValueAndExactTag(Object),
}
