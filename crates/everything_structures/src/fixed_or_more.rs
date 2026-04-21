/// An iterator that can yield nothing, one item, two items,
/// or more items.
#[repr(u8)]
pub enum FixedOrMore<More: Iterator> {
    None = 0,
    One(More::Item) = 1,
    Two(More::Item, More::Item) = 2,
    More(More),
}

impl<More: Iterator> Iterator for FixedOrMore<More> {
    type Item = More::Item;

    fn next(&mut self) -> Option<Self::Item> {
        // This method could be written with `std::mem::replace`
        // but it would generate suboptimal assembly with
        // reallocations and stuff. Therefore we dabble a bit
        // into unsafe. Here is the safe version
        //
        // ```rust
        // match std::mem::replace(self, Self::None) {
        //     Self::None => None,
        //     Self::One(one) => Some(one),
        //     Self::Two(first, second) => {
        //         *self = Self::One(first);
        //         Some(second)
        //     }
        //     Self::More(mut more) => {
        //         let next = more.next();
        //         *self = Self::More(more);
        //         next
        //     }
        // }
        // ```

        match self {
            Self::More(more) => more.next(),
            Self::None => None,
            Self::One(item) | Self::Two(_, item) => unsafe {
                let data_ptr: *const More::Item = item;

                let disc_ptr = self as *mut Self as *mut u8;
                *disc_ptr -= 1;

                Some(std::ptr::read(data_ptr))
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            FixedOrMore::None => (0, Some(0)),
            FixedOrMore::One(_) => (1, Some(1)),
            FixedOrMore::Two(_, _) => (2, Some(2)),
            FixedOrMore::More(more) => more.size_hint(),
        }
    }
}
