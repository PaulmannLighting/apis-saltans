use half::f16;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum FloatingPoint {
    Semi(f16),
    Single(f32),
    Double(f64),
}
