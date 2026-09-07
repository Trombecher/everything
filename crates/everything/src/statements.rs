use everything_objects::{Abstract, Composite, Object, Property};
use rpds::HashTrieMap;

use crate::ext::PropertyExt;

pub struct StatementProperty {
    tag: Object,
    value: Object,
    additional_properties: Composite,
}

impl StatementProperty {
    pub fn to_composite(&self) -> Composite {
        self.additional_properties.add(&mut [
            Property::new_statement_tag(self.tag.clone()),
            Property::new_statement_value(self.value.clone()),
        ])
    }
}

pub struct Statements {
    abstract_properties: HashTrieMap<Abstract, Vec<StatementProperty>>,
}

impl Statements {
    pub fn change(&self) {
        // self.abstract_properties.remove_mut(key)
    }
}
