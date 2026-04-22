#[cfg(test)]
mod tests;

use std::{
    cmp::Ordering,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    hint::unreachable_unchecked,
    sync::{Arc, LazyLock},
};

use dashmap::DashMap;

use crate::{Abstract, Byte, Object, Property, Structure};

static GLOBAL_BINARY_DATA: LazyLock<DashMap<u64, Arc<[u8]>>> = LazyLock::new(DashMap::new);

#[derive(Clone, Eq)]
pub struct BytesStructure {
    /// This will not be empty.
    data: Arc<[u8]>,
}

impl BytesStructure {
    /// Creates a new binary structure. Returns `None` if the slice is empty
    /// (which is represented using [crate::Structure::Empty])
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

    #[must_use]
    pub fn parts(&self) -> (Byte, &[u8]) {
        let (item, tail) = unsafe { self.data.split_first().unwrap_unchecked() };
        (Byte(*item), tail)
    }

    #[must_use]
    pub fn has(&self, tag: &Object, value: &Object) -> bool {
        match tag {
            Object::Abstract(Abstract::LIST_ITEM) => {
                value == &Object::Structure(Structure::Byte(self.parts().0))
            }
            Object::Abstract(Abstract::LIST_TAIL)
                if let Object::Structure(Structure::Empty) = value =>
            {
                // Tail is empty
                self.as_ref().len() == 1
            }
            Object::Abstract(Abstract::LIST_TAIL)
                if let Object::Structure(Structure::Bytes(tail)) = &value =>
            {
                // Tail is non empty
                tail.as_ref() == self.parts().1
            }
            _ => false,
        }
    }

    pub fn properties<'properties>(&'properties self) -> BytesStructureProperties<'properties> {
        let (item, tail) = self.parts();

        BytesStructureProperties {
            item,
            tail,
            index: 0,
        }
    }

    pub fn values<'properties>(
        &'properties self,
        tag: Object,
    ) -> BytesStructureValues<'properties> {
        match tag {
            Object::Abstract(Abstract::LIST_ITEM) => BytesStructureValues::ListItem(self.parts().0),
            Object::Abstract(Abstract::LIST_TAIL) => BytesStructureValues::Tail(self.parts().1),
            _ => BytesStructureValues::None,
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
        Arc::as_ptr(&self.data)
            .cast::<()>()
            .cmp(&Arc::as_ptr(&other.data).cast::<()>())
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

#[derive(Clone)]
pub struct BytesStructureProperties<'bytes> {
    item: Byte,
    tail: &'bytes [u8],
    index: u8,
}

impl Iterator for BytesStructureProperties<'_> {
    type Item = Property;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: test the order of this

        match self.index {
            0 => {
                self.index += 1;
                Some(Property::list_tail(Object::from(Structure::from(
                    self.tail,
                ))))
            }
            1 => {
                self.index += 1;
                Some(Property::list_item(Object::from(Structure::from(
                    self.item,
                ))))
            }
            2 => None,
            _ => unsafe {
                // SAFETY: this is ok
                unreachable_unchecked()
            },
        }
    }
}

#[derive(Clone)]
pub enum BytesStructureValues<'bytes> {
    None,
    ListItem(Byte),
    Tail(&'bytes [u8]),
}

impl Iterator for BytesStructureValues<'_> {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::None => None,
            Self::ListItem(byte) => {
                let byte = *byte;
                *self = Self::None;

                Some(Object::from(Structure::from(byte)))
            }
            Self::Tail(tail) => {
                let tail = *tail;
                *self = Self::None;

                Some(Object::Structure(Structure::from(tail)))
            }
        }
    }
}
