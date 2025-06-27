use crate::ff;
use crate::objects::ObjectId;
use crate::values::{ConstValue, Duration, Schema, Value, I120};
use memmap2::MmapMut;
use std::mem::transmute;
use std::num::NonZeroU64;
use std::ops::{Deref, DerefMut};
use tokio::sync::RwLock;

/// A wrapper around
pub struct ContentsMemMap(MmapMut);

impl ContentsMemMap {
    #[inline]
    #[must_use]
    pub const unsafe fn new(mmap: MmapMut) -> Self {
        Self(mmap)
    }
}

impl Deref for ContentsMemMap {
    type Target = Contents;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(&self.0[..] as *const [u8] as *const Self::Target) }
    }
}

impl DerefMut for ContentsMemMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(&mut self.0[..] as *mut [u8] as *mut Self::Target) }
    }
}

#[repr(C, align(4096))]
pub struct Contents {
    /// Magic bytes, EVERYTHINGDB
    pub magic_bytes: [u8; 12],
    /// The version of the database.
    pub version: u32,
    /// The root validation ID. Incremented on open.
    pub validation_id: u64,

    pub blocks_list_meta: RwLock<BlockListMeta>,

    // /// The metadata for the _associations_ free list.
    // pub bfl_associations: RwLock<ChainedListMeta>,
    // /// The metadata for the 2048 bytes slot free list.
    // pub bfl_2048: RwLock<ChainedListMeta>,
    /// The blocks.
    pub blocks: [Block],
}

pub struct BlockListMeta {
    pub free_block_count: u64,
    pub used_block_count: u64,
    pub free_list_head: NonZeroU64,
}

pub struct ChainedListMeta {
    /// The index of the first item in the free list.
    pub head: u64,
    /// The index of the last item in the free list. If `Some`, it contains data.
    pub free_list_head: u64,
    /// The number of items in the free list.
    pub len: u64,
}

/// An (Everything) _Block_, contiguous block of 2^14 = 16,384 bytes (spanning four OS pages
/// á 4096 bytes), aligned at OS pages.
#[repr(C, align(4096), u32)]
pub enum Block {
    Associations {
        validation_id: u64,
        next_block_index: Option<NonZeroU64>,
        _unused_0: u64,
        rows: [Row; 511],
    },
    VariableBinary {
        next_block_index: Option<NonZeroU64>,
        remaining_len: u64,
        _unused_0: u64,
        content: [u8; 511 * 32],
    },
    /// Indicates that this block is used for variable text (UTF-8).
    VariableText {
        next_block_index: Option<NonZeroU64>,
        remaining_len: u64,
        _unused_0: u64,
        content: [u8; 511 * 32],
    },
    BinaryStorage64 {
        /// The number of used slots in this block. When zero, unchain this block from the freelist
        used_slot_count: NonZeroU64,
        slots: [(u8, [u8; 63]); 127],
    },
    /// Indicates that this block is free. `Some(block)` if there is a next block,
    /// `None` if this is the end of the free list.
    FreeList(Option<NonZeroU64>),
}

impl Contents {}

#[repr(C)]
#[derive(Clone, PartialEq)]
pub struct Row([u64; 4]);

impl Row {
    pub fn encode(row: DecodedRow) -> Self {
        todo!()
    }

    pub const fn const_encode(row: ConstDecodedRow) -> Self {
        match row {
            ConstDecodedRow::Association(target_id, tag_id, value) => {
                let (a, b) = match value {
                    ConstValue::Unit => (ff::UNIT as u64, 0),
                    ConstValue::ObjectReference(id) => (ff::OBJECT_REFERENCE as u64, id.get()),
                    ConstValue::Schema(schema) => {
                        let (byte, other) = match schema {
                            Schema::Unit => (ff::UNIT, 0),
                            Schema::Integer => (ff::INTEGER, 0),
                            Schema::Float => (ff::FLOAT, 0),
                            Schema::Character => (ff::CHARACTER, 0),
                            Schema::Duration => (ff::DURATION, 0),
                            Schema::DateTime => (ff::DATE_TIME, 0),
                            Schema::ObjectReference(id) => (
                                ff::OBJECT_REFERENCE,
                                match id {
                                    None => 0,
                                    Some(id) => id.get(),
                                },
                            ),
                            Schema::Schema => (ff::SCHEMA, 0),
                            Schema::Language => (ff::LANGUAGE, 0),
                            Schema::Url => (ff::URL, 0),
                            Schema::Color => (ff::COLOR, 0),
                            Schema::Email => (ff::EMAIL, 0),
                            Schema::Text => (ff::TEXT, 0),
                            Schema::Binary => (ff::BINARY, 0),
                            Schema::Encrypted => (ff::ENCRYPTED, 0),
                        };

                        (ff::SCHEMA as u64 | ((byte as u64) << 8), other)
                    }
                };

                Self([target_id.get(), tag_id.get(), a, b])
            }
            ConstDecodedRow::FreeListNextFreeIndex(index) => Self([0, index.get(), 0, 0]),
            ConstDecodedRow::FreeListEnd => Self([0; 4]),
        }
    }

    pub const fn decode(self) -> Result<DecodedRow, ()> {
        match self.0 {
            [0, 0, 0, 0] => Ok(DecodedRow::FreeListEnd),
            [0, index, 0, 0] => Ok(DecodedRow::FreeListNextFreeIndex(
                NonZeroU64::new(index).unwrap(),
            )),
            [object_id, tag_id, a, b] if let Some(tag_id) = NonZeroU64::new(tag_id) => {
                let object_id = NonZeroU64::new(object_id).unwrap();

                let value = match a.to_le_bytes() {
                    [ff::UNIT, ..] => Value::Unit,
                    [ff::INTEGER, ..] => Value::Integer(b as _),
                    [ff::FLOAT, ..] => Value::Float(f64::from_bits(b)),
                    [ff::CHARACTER, _, _, _, remaining @ ..] => {
                        Value::Character(match char::from_u32(u32::from_le_bytes(remaining)) {
                            Some(char) => char,
                            None => return Err(()),
                        })
                    }
                    [ff::DURATION, bytes @ ..] => {
                        Value::Duration(Duration::from_nanos(I120::from_le_bytes(unsafe {
                            transmute((bytes, b.to_le_bytes()))
                        })))
                    }
                    [ff::OBJECT_REFERENCE, ..] if b != 0 => {
                        Value::ObjectReference(NonZeroU64::new(b).unwrap())
                    }
                    _ => todo!(),
                };

                Ok(DecodedRow::Association(object_id, tag_id, value))
            }
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum DecodedRow {
    Association(ObjectId, ObjectId, Value),

    /// Row matches `[0, index != 0, 0, 0]`
    FreeListNextFreeIndex(NonZeroU64),

    /// Row matches `[0, 0, 0, 0]`
    FreeListEnd,
}

#[derive(PartialEq, Clone)]
pub enum ConstDecodedRow {
    Association(ObjectId, ObjectId, ConstValue),
    FreeListNextFreeIndex(NonZeroU64),
    FreeListEnd,
}

impl ConstDecodedRow {
    pub const fn const_into_decoded_row(self) -> DecodedRow {
        match self {
            ConstDecodedRow::Association(o, t, v) => {
                DecodedRow::Association(o, t, v.const_into_value())
            }
            ConstDecodedRow::FreeListNextFreeIndex(i) => DecodedRow::FreeListNextFreeIndex(i),
            ConstDecodedRow::FreeListEnd => DecodedRow::FreeListEnd,
        }
    }
}

impl Into<DecodedRow> for ConstDecodedRow {
    fn into(self) -> DecodedRow {
        self.const_into_decoded_row()
    }
}
