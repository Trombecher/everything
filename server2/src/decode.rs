//! This module handles (partial) decoding of values.

/// Items implementing this trait are decodable.
pub trait Decodable {
    type Output;

    fn decode(&self) -> Self::Output;
}

/// Items implementing this trait are partially decodable.
pub trait PartiallyDecodable {
    type PartialOutput: Decodable;

    fn decode_partial(&self) -> Self::PartialOutput;
}

impl<T: PartiallyDecodable + ?Sized> Decodable for T {
    type Output = <<T as PartiallyDecodable>::PartialOutput as Decodable>::Output;

    fn decode(&self) -> Self::Output {
        self.decode_partial().decode()
    }
}