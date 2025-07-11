//! Color primitive.

/// A color represented in Oklab.
#[derive(PartialEq, Clone, Debug, Copy)]
#[repr(C, packed)]
pub struct Color {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}
