use everything_structures::{Object, Property, Structure};

use crate::{
    inference::{PropertyForm, ValidationError},
    objects,
};

pub fn cardinality_of_1(target: &Structure, tag: &Object) -> Result<(), ValidationError> {
    let mut values = target.values(tag);

    if values.next().is_none() {
        return Err(ValidationError::MissingPropertyOn {
            subject: target.clone().into(),
            form: PropertyForm::SomeValueAndExactTag(objects::STATEMENT_SUBJECT),
        });
    }

    if let Some(value) = values.next() {
        return Err(ValidationError::FoundPropertyOn {
            subject: target.clone().into(),
            property: Property {
                tag: tag.clone(),
                value: value.clone(),
            },
        });
    }

    Ok(())
}

pub fn statement(structure: &Structure) -> Result<(), ValidationError> {
    self::cardinality_of_1(structure, &objects::STATEMENT_SUBJECT)?;
    self::cardinality_of_1(structure, &objects::STATEMENT_TAG)?;
    self::cardinality_of_1(structure, &objects::STATEMENT_VALUE)
}

pub fn knowledge(root: &Structure) -> Result<(), ValidationError> {
    // First we validate that every object contained
    // in the root is a statement.

    for statement in root.values(&objects::CONTAINS) {
        let statement = match statement {
            Object::Abstract(o) => {
                return Err(ValidationError::FoundPropertyOn {
                    subject: root.clone().into(),
                    property: Property {
                        tag: objects::CONTAINS,
                        value: (*o).into(),
                    },
                });
            }
            Object::Structure(structure) => structure,
        };

        // Validate structure
        self::statement(statement)?;
    }

    Ok(())
}
