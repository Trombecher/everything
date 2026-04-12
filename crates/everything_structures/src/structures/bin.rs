use std::{hash::Hash, sync::Arc};

use crate::Registry;

#[derive(Clone, Debug)]
pub struct BlobStructure<R: Registry> {
    pub(super) data: Option<Arc<[u8]>>,
    pub(super) registry: R,
}

impl<R: Registry> AsRef<[u8]> for BlobStructure<R> {
    fn as_ref(&self) -> &[u8] {
        match &self.data {
            Some(data) => &data,
            None => &[],
        }
    }
}

impl<R: Registry> Hash for BlobStructure<R> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<R: Registry> PartialEq for BlobStructure<R> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.data, &other.data) {
            (None, None) => true,
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl<R: Registry> Eq for BlobStructure<R> {}

impl<R: Registry> PartialOrd for BlobStructure<R> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<R: Registry> Ord for BlobStructure<R> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}
