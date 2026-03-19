use std::ops::Range;

use crate::SourceIndex;

#[derive(Copy, Clone, PartialEq)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl TryFrom<char> for Digit {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            _ => Err(()),
        }
    }
}

impl From<Digit> for u8 {
    fn from(value: Digit) -> Self {
        value as u8
    }
}

pub trait AbstractConstructor: Default {
    fn push(&mut self, digit: Digit);
}

pub fn extract_abstract(source: &str, range: Range<SourceIndex>) -> impl Iterator<Item = Digit> {
    source[range.start as usize..range.end as usize]
        .chars()
        .skip(1)
        .map(|c| Digit::try_from(c).unwrap())
}
