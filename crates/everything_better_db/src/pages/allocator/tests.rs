use std::num::NonZero;

use crate::pages::storage::InMemoryStorage;

use super::*;

#[test]
fn stuff() {
    let pa = PageAllocator::new(InMemoryStorage::new(NonZero::new(10).unwrap()).unwrap()).unwrap();

    let meta_page = pa.meta_page().unwrap();
    meta_page.init();
}
