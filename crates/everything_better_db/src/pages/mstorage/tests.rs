use std::num::NonZero;

use crate::pages::{CurrentSuperBlock, MetaPage, storage::InMemoryStorage};

use super::*;

#[test]
pub fn access() {
    let ms = ManagedStorage::new(InMemoryStorage::new(NonZero::new(10).unwrap()).unwrap());
    let page = ms.page(PageId::<MetaPage>::new(0)).unwrap();

    page.current_super_block.set(CurrentSuperBlock::A);

    assert_eq!(page.current_super_block.get(), CurrentSuperBlock::A);
}
