use std::collections::{HashMap, HashSet};

use crate::{
    PackedId,
    associations::Association,
    hash::{DataHash, StructureHash},
    structures::Structure,
};

pub struct Database {
    data: HashMap<DataHash, Box<[u8]>>,
    structures: HashMap<StructureHash, Structure>,
    stored: HashSet<Association>,
}

impl Database {
    pub fn structure(&self, hash: &StructureHash) -> &Structure {
        self.structure(hash)
    }

    pub fn validate(&self) {}

    pub fn query_stored_associations(&self) -> impl Iterator<Item = &Association> {
        self.stored.iter()
    }

    pub fn query_stored_values(
        &self,
        target: PackedId,
        tag: PackedId,
    ) -> impl Iterator<Item = PackedId> {
        let find_target = target;
        let find_tag = tag;

        self.query_stored_associations()
            .filter_map(move |Association { target, tag, value }| {
                (*target == find_target && *tag == find_tag).then_some(*value)
            })
    }
}
