use ulid::Ulid;

use crate::Digit;

pub struct UlidFromIterator(pub Ulid);

impl FromIterator<Digit> for UlidFromIterator {
    fn from_iter<T: IntoIterator<Item = Digit>>(iter: T) -> Self {
        let n = iter.into_iter().fold(0_u128, |acc, item| {
            acc.saturating_mul(10).saturating_add(item as u8 as u128)
        });

        Self(Ulid(n))
    }
}
