use std::fmt::Debug;

pub struct DebugIterator<I: Iterator + Clone>(pub I)
where
    I::Item: Debug;

impl<I: Iterator + Clone> Debug for DebugIterator<I>
where
    I::Item: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.clone()).finish()
    }
}
