use std::sync::Arc;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlobStructure {
    data: Option<Arc<[u8]>>,
}
