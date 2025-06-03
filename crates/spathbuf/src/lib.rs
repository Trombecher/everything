mod write;

use std::ffi::OsStr;
use std::mem::transmute;
use std::path::Path;
use arrayvec::ArrayVec;

#[derive(Clone)]
pub struct StackPathBuf<const CAPACITY: usize> {
    #[cfg(unix)]
    vec: ArrayVec<u8, CAPACITY>,
    #[cfg(windows)]
    vec: ArrayVec<u16, CAPACITY>,
}

impl<const CAPACITY: usize> StackPathBuf<CAPACITY> {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            vec: ArrayVec::new_const()
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.vec.clear();
    }

    #[inline]
    #[cfg(windows)]
    pub unsafe fn unsafe_push_unit(&mut self, unit: u16) {
        self.vec.push(unit);
    }
}

impl<const CAPACITY: usize> AsRef<Path> for StackPathBuf<CAPACITY> {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl<const CAPACITY: usize> AsRef<OsStr> for StackPathBuf<CAPACITY> {
    fn as_ref(&self) -> &OsStr {
        unsafe { transmute(self.vec.as_ref()) }
    }
}