use crate::StackPathBuf;

pub trait WritePathComponent<const CAPACITY: usize> {
    fn write_path_component(&self, buf: &mut StackPathBuf<CAPACITY>);
}