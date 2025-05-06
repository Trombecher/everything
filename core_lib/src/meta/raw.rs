use crate::error::Error;
use memmap2::MmapMut;
use std::io;
use std::marker::PhantomData;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, MutexGuard};
use tokio::fs::File;

/// Any type implementing this trait can be used in [RawMeta]. But it needs to have some
/// properties:
///
/// * `repr(C, align(4096))`
/// * The type must be zero-initializeable.
pub unsafe trait MetaContent {}

pub struct RawMeta<T: MetaContent> {
    file_handle: File,
    map: Mutex<MmapMut>,
    _marker: PhantomData<T>,
}

impl<T: MetaContent> RawMeta<T> {
    #[inline]
    pub async fn new(file_handle: File) -> Result<Self, Error> {
        file_handle
            .set_len(size_of::<T>() as u64)
            .await
            .map_err(Error::from)?;

        Ok(Self {
            map: Mutex::new(unsafe {
                MmapMut::map_mut(&file_handle).map_err(Error::from)?
            }),
            file_handle,
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn lock(&self) -> MetaGuard<T> {
        MetaGuard {
            guard: self.map.lock().unwrap(),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn flush(&self) -> io::Result<()> {
        self.map.lock().unwrap().flush()
    }
}

pub struct MetaGuard<'a, T: MetaContent> {
    guard: MutexGuard<'a, MmapMut>,
    _marker: PhantomData<T>,
}

impl<'a, T: MetaContent> Deref for MetaGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self.guard.as_ptr()) }
    }
}

impl<'a, T: MetaContent> DerefMut for MetaGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(self.guard.as_mut_ptr()) }
    }
}
