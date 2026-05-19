use everything_structures::{
    Abstract, AnyStructureProperties, Object, Structure, StructureProperties,
};

use crate::ext::AbstractExt;

/// An iterator over all set values of a given structure.
#[derive(Clone)]
pub struct StructureSetValues {
    properties: StructureProperties,
}

impl StructureSetValues {
    pub fn new(structure: &Structure) -> Self {
        Self {
            properties: match structure {
                Structure::Empty
                | Structure::Integer(_)
                | Structure::Bytes(_)
                | Structure::Text(_)
                | Structure::Byte(_)
                | Structure::Character(_) => {
                    // These do not have set values.
                    Structure::Empty.properties()
                }
                Structure::Any(any_structure) => {
                    StructureProperties::Any(AnyStructureProperties::new_starting_from_tag(
                        any_structure.clone(),
                        Abstract::CONTAINS.into(),
                    ))
                }
            },
        }
    }
}

impl Iterator for StructureSetValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        // We don't use find_map here because we
        // don't want to continue if the tag does not match
        // CONTAINS (iterator is sorted).

        self.properties.next().and_then(|property| {
            (property.tag == Abstract::CONTAINS.into()).then_some(property.value)
        })
    }
}

impl std::fmt::Debug for StructureSetValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(&mut self.clone()).finish()
    }
}
