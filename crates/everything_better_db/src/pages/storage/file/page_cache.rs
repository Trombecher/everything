use std::io;

use crate::pages::storage::{
    PseudoReference,
    pages::{OpaquePage, Pages},
};

pub struct PageCache {
    pages: Pages,
}

impl PageCache {
    pub fn new(slot_count: u64) -> Result<Self, io::Error> {
        Pages::new(slot_count).map(|pages| Self { pages })
    }

    pub fn slot(&self, slot_index: u64) -> Option<PseudoReference<OpaquePage>> {
        self.pages.page(slot_index)
    }
}
