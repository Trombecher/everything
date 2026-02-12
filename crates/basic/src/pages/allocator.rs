use std::sync::atomic::Ordering;

use tokio::sync::Mutex;

use crate::{
    Error,
    pages::{MetaPage, Page, PageProvider, kinds::FreePage},
};

pub struct PageAllocatorSubsystem<P: PageProvider> {
    provider: P,
}

impl<P: PageProvider> PageAllocatorSubsystem<P> {
    #[must_use]
    pub const fn new(provider: P) -> Self {
        Self { provider }
    }

    pub async fn meta_page(&self) -> Result<&MetaPage, Error> {
        self.provider.page(0).await.map(MetaPage::from_page)
    }

    pub async fn allocate(&self) -> Result<u64, Error> {
        let meta_page = self.meta_page().await?;

        let page_id = meta_page.free_list_pop.load(Ordering::Relaxed);

        if page_id == 0 {
            todo!("no free pages")
        }

        let page = FreePage::from_page(self.provider.page(page_id).await?);
        let next_page_in_free_list = page.next_free_page.load(Ordering::Relaxed);

        meta_page
            .free_list_pop
            .store(next_page_in_free_list, Ordering::Relaxed);

        Ok(page_id)
    }

    pub async fn free(&self, page_id: u64) -> Result<(), Error> {
        let meta_page = self.meta_page().await?;

        todo!()
    }
}
