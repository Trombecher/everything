use std::num::NonZero;

use crate::pages::{MetaPage, storage::InMemoryStorage};

use super::*;

#[test]
fn access() {
    let ms = ManagedStorage::new(InMemoryStorage::new(NonZero::new(10).unwrap()).unwrap());
    let page = ms.page(PageId::<MetaPage>::new(0)).unwrap();
}
