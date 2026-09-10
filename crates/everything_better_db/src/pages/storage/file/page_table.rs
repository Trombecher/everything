use dashmap::DashMap;

use crate::{convert::safe_u64_to_usize, pages::RawPageId};

pub struct PageTable {
    /// Mapping from raw page ids to page cache slots.
    mapping: DashMap<RawPageId, u64>,
}

impl PageTable {
    pub fn new(slot_count: u64) -> Self {
        Self {
            mapping: DashMap::with_capacity(safe_u64_to_usize(slot_count)),
        }
    }

    pub fn cache_slot_index(&self, page: RawPageId) -> Option<u64> {
        self.mapping.get(&page).map(|r| *r)
    }
}
