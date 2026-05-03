pub trait IteratorExtNextAndLast: Iterator {
    /// Returns the next item if it is also the last in this iterator.
    fn next_and_last(&mut self) -> Option<Self::Item> {
        let item = self.next()?;
        self.next().is_none().then_some(item)
    }
}

impl<I: Iterator> IteratorExtNextAndLast for I {}
