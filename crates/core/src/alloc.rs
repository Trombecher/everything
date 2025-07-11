use crate::pages::PageId;

pub struct Allocator {
    pub(crate) free_list_len: u64,
    pub(crate) free_list_head: Option<PageId>,
    pub(crate) free_list_tail: Option<PageId>,
}
