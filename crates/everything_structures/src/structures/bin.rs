use std::{cmp::Ordering, fmt::Debug, hash::Hash, sync::Arc};

#[derive(Clone, Eq)]
pub struct BlobStructure {
    pub(super) data: Option<Arc<[u8]>>,
}

impl AsRef<[u8]> for BlobStructure {
    fn as_ref(&self) -> &[u8] {
        match &self.data {
            Some(data) => data,
            None => &[],
        }
    }
}

impl Debug for BlobStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl Hash for BlobStructure {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.as_ref().map(Arc::as_ptr).hash(state);
    }
}

impl PartialEq for BlobStructure {
    fn eq(&self, other: &Self) -> bool {
        match (&self.data, &other.data) {
            (None, None) => true,
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl PartialOrd for BlobStructure {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BlobStructure {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data
            .as_ref()
            .map(Arc::as_ptr)
            .cmp(&other.data.as_ref().map(Arc::as_ptr))
    }
}
