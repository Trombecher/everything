//! Color primitive.

/// A color represented in Oklab.
#[derive(PartialEq, Clone, Debug, Copy)]
#[repr(C, packed)]
pub struct Color {
    _padding: [u8; 3],
    pub l: f32,
    pub a: f32,
    pub b: f32,
}
