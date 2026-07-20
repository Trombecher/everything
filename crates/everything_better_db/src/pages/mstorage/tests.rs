use crate::pages::{MetaPage, storage::InMemoryStorage};

use super::*;

#[test]
fn access() {
    let ms = ManagedStorage::new(InMemoryStorage::new(10).unwrap());
    let _ = ms.page(PageId::<MetaPage>::new(0)).unwrap();
}
