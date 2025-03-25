use memmap2::MmapMut;
use std::fs::File;
use std::io;
use std::marker::PhantomData;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, MutexGuard};

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
    #[must_use]
    pub fn new(file_handle: File) -> Self {
        file_handle.set_len(size_of::<T>() as u64).unwrap();
        
        Self {
            map: Mutex::new(unsafe { MmapMut::map_mut(&file_handle).unwrap() }),
            file_handle,
            _marker: PhantomData,
        }
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