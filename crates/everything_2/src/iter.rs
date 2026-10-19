//! Iterator helpers.

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

pub trait IteratorExtNextAndLast: Iterator {
    /// Returns the next item if it is also the last in this iterator.
    fn next_and_last(&mut self) -> Option<Self::Item> {
        let item = self.next()?;
        self.next().is_none().then_some(item)
    }
}

impl<I: Iterator> IteratorExtNextAndLast for I {}
