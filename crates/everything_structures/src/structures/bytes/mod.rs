#[cfg(test)]
mod tests;

use std::{
    cmp::Ordering,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    num::NonZeroUsize,
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

    /// Creates a new [`BytesStructure`] from an iterator and a length.
    /// This function will extract `length` items from the iterator and
    /// write them to a preallocated [`Arc`].
    ///
    /// # Panics
    ///
    /// This function panics iff the iterator is not able to yield `length`
    /// items.
    pub fn from_iter(mut iter: impl Iterator<Item = u8>, len: NonZeroUsize) -> Self {
        let mut data = Arc::new_uninit_slice(len.get());

        {
            let arc_ref = Arc::get_mut(&mut data).unwrap();

            for i in 0..len.get() {
                let value = iter.next().expect("iterator ended too early");

                arc_ref.get_mut(i).unwrap().write(value);
            }
        }

        let data = unsafe { data.assume_init() };

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        let hash_of_bytes = hasher.finish();

        if let Some(reference) = GLOBAL_BINARY_DATA.get(&hash_of_bytes) {
            // Return reference and drop allocated data.
            Self {
                data: Arc::clone(&reference),
            }
        } else {
            GLOBAL_BINARY_DATA.insert(hash_of_bytes, Arc::clone(&data));

            Self { data }
        }
    }

    pub fn from_parts(a: &[u8], b: &[u8]) -> Option<Self> {
        let total_length = a.len().checked_add(b.len()).expect(":/");

        if total_length == 0 {
            return None;
        }

        let mut hasher = DefaultHasher::new();
        hasher.write_usize(total_length);
        hasher.write(a);
        hasher.write(b);
        let hash_of_bytes = hasher.finish();

        if let Some(reference) = GLOBAL_BINARY_DATA.get(&hash_of_bytes) {
            Some(Self {
                data: Arc::clone(&reference),
            })
        } else {
            // Entry does not yet exist.
            let mut arc = Arc::new_uninit_slice(total_length);

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

    pub fn from_raw(data: Arc<[u8]>) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        let mut hasher = DefaultHasher::new();
        data.as_ref().hash(&mut hasher);
        let hash_of_bytes = hasher.finish();

        GLOBAL_BINARY_DATA.insert(hash_of_bytes, Arc::clone(&data));

        Some(Self { data })
    }

    /// Returns the number of strong Arc references
    /// to the data allocation.
    #[must_use]
    pub fn ref_count(&self) -> usize {
        Arc::strong_count(&self.data)
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
        BytesStructureProperties::TailAndItem(tail, item)
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
        // TODO: maybe move this into something like `.as_bytes()`.
        &self.data
    }
}

impl Debug for BytesStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("X")?;

        for byte in self.as_ref() {
            write!(f, "{:02X}", byte)?;
        }

        Ok(())
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

/// An iterator over the (virtual) properties of
/// a [BytesStructure]. You can obtain an instance
/// of this iterator by calling [BytesStructure::properties].
///
/// This iterator is guaranteed to return items in
/// lexicographical [Ord]er.
#[derive(Clone)]
pub enum BytesStructureProperties<'bytes> {
    TailAndItem(&'bytes [u8], Byte),
    Tail(&'bytes [u8]),
    None,
}

impl Iterator for BytesStructureProperties<'_> {
    type Item = Property;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::TailAndItem(tail, item) => {
                let tail = *tail;
                let item = *item;
                *self = Self::Tail(tail);

                Some(Property::new_list_item(Object::from(Structure::from(item))))
            }
            Self::Tail(tail) => {
                let tail = *tail;
                *self = Self::None;

                Some(Property::new_list_tail(Object::from(Structure::from(tail))))
            }
            Self::None => None,
        }
    }
}

/// An iterator over the (virtual) values of
/// a [BytesStructure] which are associated with a
/// tag. You can obtain an instance of this
/// iterator by calling [BytesStructure::values].
///
/// This iterator is guaranteed to return items in
/// lexicographical [Ord]er. Also the iterator will
/// yield exactly one [Object].
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaybeEmptyBytesStructure(pub Option<BytesStructure>);

impl AsRef<[u8]> for MaybeEmptyBytesStructure {
    fn as_ref(&self) -> &[u8] {
        match &self.0 {
            None => &[],
            Some(bytes) => bytes.as_ref(),
        }
    }
}

impl TryFrom<&Structure> for MaybeEmptyBytesStructure {
    type Error = ();

    fn try_from(value: &Structure) -> Result<Self, Self::Error> {
        match value {
            Structure::Empty => Ok(Self(None)),
            Structure::Bytes(bytes) => Ok(Self(Some(bytes.clone()))),
            _ => Err(()),
        }
    }
}
