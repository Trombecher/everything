//! Color primitive.

/// A color represented in Oklch.
#[derive(PartialEq, Clone)]
pub struct Color {
    l: f32,
    c: f32,
    h: f32
}