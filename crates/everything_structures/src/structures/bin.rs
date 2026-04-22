use std::{
    cmp::Ordering,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    sync::{Arc, LazyLock},
};

use dashmap::DashMap;

static GLOBAL_BINARY_DATA: LazyLock<DashMap<u64, Arc<[u8]>>> = LazyLock::new(DashMap::new);

#[derive(Clone, Eq)]
pub struct BytesStructure {
    // An empty
    data: Arc<[u8]>,
}

impl BytesStructure {
    /// Creates a new binary structure. Returns `None` if the slice is empty
    /// (which is represented using [crate::Object::LIST_EMPTY])
    #[must_use]
    pub fn new(bytes: &[u8]) -> Option<Self> {
        Self::from_parts(bytes, &[])
    }

    pub fn from_parts(a: &[u8], b: &[u8]) -> Option<Self> {
        if a.is_empty() && b.is_empty() {
            return None;
        }

        let mut hasher = DefaultHasher::new();
        a.hash(&mut hasher);
        b.hash(&mut hasher);
        let hash_of_bytes = hasher.finish();

        if let Some(reference) = GLOBAL_BINARY_DATA.get(&hash_of_bytes) {
            Some(Self {
                data: Arc::clone(&reference),
            })
        } else {
            // Entry does not yet exist.
            let mut arc = Arc::new_uninit_slice(a.len().checked_add(b.len()).expect(":/"));

            {
                // Write data of both slices.

                let data = unsafe { Arc::get_mut(&mut arc).unwrap_unchecked() };
                data[..a.len()].write_copy_of_slice(a);
                data[a.len()..].write_copy_of_slice(b);
            }

            let arc = unsafe { arc.assume_init() };
            GLOBAL_BINARY_DATA.insert(hash_of_bytes, Arc::clone(&arc));

            Some(Self { data: arc })
        }
    }
}

impl AsRef<[u8]> for BytesStructure {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl Debug for BytesStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl Hash for BytesStructure {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.data).hash(state);
    }
}

impl PartialEq for BytesStructure {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.data, &other.data)
    }
}

impl PartialOrd for BytesStructure {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BytesStructure {
    fn cmp(&self, other: &Self) -> Ordering {
        Arc::as_ptr(&self.data).cmp(&Arc::as_ptr(&other.data))
    }
}

impl Drop for BytesStructure {
    fn drop(&mut self) {
        let ref_count = Arc::strong_count(&self.data);

        if ref_count == 2 {
            // We and the registry are the only ones holding it.

            // Calculate hash of bytes:
            let mut hasher = DefaultHasher::new();
            self.data.hash(&mut hasher);
            let hash_of_bytes = hasher.finish();

            // Remove from registry:
            GLOBAL_BINARY_DATA.remove(&hash_of_bytes);
        } else if ref_count == 1 {
            unreachable!("this blob was not registered")
        }
    }
}
