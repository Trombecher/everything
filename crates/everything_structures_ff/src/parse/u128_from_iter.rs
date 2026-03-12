use crate::lex::Digit;

pub struct U128FromIterator(pub u128);

impl FromIterator<Digit> for U128FromIterator {
    fn from_iter<T: IntoIterator<Item = Digit>>(iter: T) -> Self {
        Self(iter.into_iter().fold(0_u128, |acc, item| {
            acc.saturating_mul(10).saturating_add(item as u8 as u128)
        }))
    }
}
