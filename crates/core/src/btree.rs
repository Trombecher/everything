use crate::ff;
use crate::objects::ObjectId;
use crate::pages::{PageContent, PageId};
use std::mem::MaybeUninit;

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum BTreeNodeKind {
    OxTV = ff::PAGE_KIND_B_TREE_NODE_O_TV,
    TxV = ff::PAGE_KIND_B_TREE_NODE_T_V,
    VxT = ff::PAGE_KIND_B_TREE_NODE_V_T,
    TxOV = ff::PAGE_KIND_B_TREE_NODE_T_OV,
    VxO = ff::PAGE_KIND_B_TREE_NODE_V_O,
    VxOT = ff::PAGE_KIND_B_TREE_NODE_V_OT,
}

#[repr(C, align(16))]
pub struct BTreeNode<const I_COUNT: usize, K: Ord + Clone, V: Clone> {
    /// This is the byte from [PageKind].
    kind: BTreeNodeKind,
    node_len: u8,
    _padding: [u8; 6],
    keys: [MaybeUninit<K>; I_COUNT],
    values: [MaybeUninit<V>; I_COUNT],
    first_child: PageId,
    other_children: [PageId; I_COUNT],
}

const fn calculate_optimal_b_tree_order(key_size: usize, value_size: usize) -> usize {
    4064 / (key_size + value_size + 8)
}

macro_rules! define_b_tree_variant {
    ($id:ident, $k:ty, $v:ty) => {
        pub type $id =
            BTreeNode<{ calculate_optimal_b_tree_order(size_of::<$k>(), size_of::<$v>()) }, $k, $v>;
        ::static_assertions::const_assert!(size_of::<$id>() <= 4080);
    };
}

// TODO
pub type InlineValue = (u64, u64);

define_b_tree_variant!(BTreeNodeOxTV, ObjectId, (PageId, PageId));
define_b_tree_variant!(BTreeNodeTxV, ObjectId, InlineValue);
define_b_tree_variant!(BTreeNodeVxT, InlineValue, ObjectId);
define_b_tree_variant!(BTreeNodeTxOV, ObjectId, (ObjectId, PageId));
define_b_tree_variant!(BTreeNodeVxO, InlineValue, ObjectId);
define_b_tree_variant!(BTreeNodeVxOT, InlineValue, (ObjectId, ObjectId));

unsafe impl PageContent for BTreeNodeOxTV {}
unsafe impl PageContent for BTreeNodeTxV {}
unsafe impl PageContent for BTreeNodeVxT {}
unsafe impl PageContent for BTreeNodeTxOV {}
unsafe impl PageContent for BTreeNodeVxOT {}
