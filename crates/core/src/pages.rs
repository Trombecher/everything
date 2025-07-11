use crate::btree::*;
use crate::validation::{ValidationId, ValidationIdStatus, ValidationIdStore};
use crate::{Error, ff};
use crc::{CRC_64_REDIS, Crc};
use static_assertions::const_assert;
use std::num::NonZeroU64;

pub type PageId = NonZeroU64;

#[repr(align(4096), C)]
pub struct UnknownPage {
    /// CRC-64 checksum for the rest of the page content, including the validation id.
    crc: u64,
    /// The validation id for this page.
    validation_id: ValidationIdStore,
    /// Raw, maybe unvalidated data.
    data: [u8; 4080],
}

const_assert!(size_of::<UnknownPage>() == 4096);

impl UnknownPage {
    #[inline]
    fn crc_data(&self) -> &[u8; 4088] {
        unsafe { &*(self.validation_id.as_ptr() as *const _) }
    }

    #[inline]
    pub fn check_integrity(&self) -> bool {
        Crc::<u64>::new(&CRC_64_REDIS).checksum(self.crc_data()) == self.crc
    }

    #[inline]
    unsafe fn assume_page_content<P: PageContent>(&self) -> &P {
        unsafe { &*(self.data.as_ptr() as *const _) }
    }

    fn validate_page_kind(&self, page_id: PageId) -> Result<PageKind, Error> {
        match self.data[0] {
            ff::PAGE_KIND_FREE_LIST => Ok(PageKind::FreeListNode(unsafe {
                self.assume_page_content::<FreeListNode>()
            })),
            ff::PAGE_KIND_SPILL => todo!(),
            ff::PAGE_KIND_B_TREE_NODE_O_TV => todo!(),
            ff::PAGE_KIND_B_TREE_NODE_T_V => todo!(),
            ff::PAGE_KIND_B_TREE_NODE_V_T => todo!(),
            ff::PAGE_KIND_B_TREE_NODE_T_OV => todo!(),
            ff::PAGE_KIND_B_TREE_NODE_V_O => todo!(),
            ff::PAGE_KIND_B_TREE_NODE_V_OT => todo!(),
            ff::PAGE_KIND_WAL => todo!(),
            byte => Err(Error::InvalidPageKind(page_id, byte)),
        }
    }

    /// Constructs a page kind from the data without checking validity.
    #[inline]
    unsafe fn assume_page_kind(&self) -> PageKind {
        unsafe {
            match self.data[0] {
                ff::PAGE_KIND_FREE_LIST => {
                    PageKind::FreeListNode(self.assume_page_content::<FreeListNode>())
                }
                ff::PAGE_KIND_SPILL => PageKind::SpillNode(self.assume_page_content::<SpillNode>()),
                ff::PAGE_KIND_B_TREE_NODE_O_TV => {
                    PageKind::BTreeNodeOxTV(self.assume_page_content::<BTreeNodeOxTV>())
                }
                ff::PAGE_KIND_B_TREE_NODE_T_V => {
                    PageKind::BTreeNodeTxV(self.assume_page_content::<BTreeNodeTxV>())
                }
                ff::PAGE_KIND_B_TREE_NODE_V_T => {
                    PageKind::BTreeNodeVxT(self.assume_page_content::<BTreeNodeVxT>())
                }
                ff::PAGE_KIND_B_TREE_NODE_T_OV => {
                    PageKind::BTreeNodeTxOV(self.assume_page_content::<BTreeNodeTxOV>())
                }
                ff::PAGE_KIND_B_TREE_NODE_V_O => {
                    PageKind::BTreeNodeVxO(self.assume_page_content::<BTreeNodeVxO>())
                }
                ff::PAGE_KIND_B_TREE_NODE_V_OT => {
                    PageKind::BTreeNodeVxOT(self.assume_page_content::<BTreeNodeVxOT>())
                }
                ff::PAGE_KIND_WAL => PageKind::WalNode(self.assume_page_content::<WalNode>()),
                // TODO: may be converted to unchecked
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    pub fn page_kind(&self, vid: ValidationId, page_id: PageId) -> Result<PageKind, Error> {
        match self.validation_id.status(vid) {
            ValidationIdStatus::Validated => Ok(unsafe { self.assume_page_kind() }),
            ValidationIdStatus::NotValidated(guard) => {
                if !self.check_integrity() {
                    guard.discard();
                    return Err(Error::PageCorrupted(page_id));
                }

                self.validate_page_kind(page_id)
                    .inspect_err(|_| guard.discard())
            }
        }
    }
}

#[repr(u8)]
pub enum PageKind<'a> {
    FreeListNode(&'a FreeListNode) = ff::PAGE_KIND_FREE_LIST,
    SpillNode(&'a SpillNode) = ff::PAGE_KIND_SPILL,
    BTreeNodeOxTV(&'a BTreeNodeOxTV) = ff::PAGE_KIND_B_TREE_NODE_O_TV,
    BTreeNodeTxV(&'a BTreeNodeTxV) = ff::PAGE_KIND_B_TREE_NODE_T_V,
    BTreeNodeVxT(&'a BTreeNodeVxT) = ff::PAGE_KIND_B_TREE_NODE_V_T,
    BTreeNodeTxOV(&'a BTreeNodeTxOV) = ff::PAGE_KIND_B_TREE_NODE_T_OV,
    BTreeNodeVxO(&'a BTreeNodeVxO) = ff::PAGE_KIND_B_TREE_NODE_V_O,
    BTreeNodeVxOT(&'a BTreeNodeVxOT) = ff::PAGE_KIND_B_TREE_NODE_V_OT,
    WalNode(&'a WalNode) = ff::PAGE_KIND_WAL,
}

pub(crate) unsafe trait PageContent {}

// TODO

pub struct SpillNode {}
unsafe impl PageContent for SpillNode {}

pub struct WalNode {}
unsafe impl PageContent for WalNode {}

pub struct FreeListNode {
    pub next: Option<PageId>,
    /* random bytes */
}

unsafe impl PageContent for FreeListNode {}
