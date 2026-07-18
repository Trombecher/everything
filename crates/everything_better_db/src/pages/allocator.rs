//! Pages the allocator uses.

use crate::{pages::PageIdLocation, unsafe_declare_page};

#[repr(C, align(4096))]
pub struct FreePage {
    pub next_page: PageIdLocation<FreePage>,
    // TODO: maybe duplicate or save free page otherwise...
    _rest: [u8; 4088],
}

unsafe_declare_page!(FreePage);
