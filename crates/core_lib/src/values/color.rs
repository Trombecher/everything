#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color(ColorType, f32, f32, f32);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ColorType {
    Cie1931Xyz,
    Cie1931Xyy,
    Oklab,
    Oklch,
    Cielab,
    Cielch,
    LinearSrgb,
    LinearAdobeRgb,
    LinearDisplayP3,
    Srgb,
    AdobeRgb,
    DisplayP3,
}