use crate::model::{Construction, Object, SetValues};

#[derive(Clone)]
pub struct QueryConstructionValues {
    pub properties: SetValues,
    pub property_constructor: Object,
}

impl Iterator for QueryConstructionValues {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.properties.find_map(|property| match property {
            Object::Construction(Construction { constructor, value })
                if constructor.as_ref() == &self.property_constructor =>
            {
                Some(*value)
            }
            _ => None,
        })
    }
}
