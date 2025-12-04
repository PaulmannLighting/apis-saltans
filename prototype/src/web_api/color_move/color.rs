use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    /// Create a new `Rgb` color.
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct LinearRgb {
    red: f32,
    green: f32,
    blue: f32,
}

impl From<Rgb> for LinearRgb {
    fn from(rgb: Rgb) -> Self {
        Self {
            red: correct_channel(rgb.red),
            green: correct_channel(rgb.green),
            blue: correct_channel(rgb.blue),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct Xyz {
    x: f32,
    y: f32,
    z: f32,
}

impl Xyz {
    const X_COEFFICIENTS: LinearRgbToXyzCoefficients =
        LinearRgbToXyzCoefficients::new(0.664_511, 0.154_324, 0.162_028);
    const Y_COEFFICIENTS: LinearRgbToXyzCoefficients =
        LinearRgbToXyzCoefficients::new(0.283_881, 0.668_433, 0.047_685);
    const Z_COEFFICIENTS: LinearRgbToXyzCoefficients =
        LinearRgbToXyzCoefficients::new(0.000_088, 0.072_310, 0.986_039);
}

impl From<LinearRgb> for Xyz {
    fn from(linear_rgb: LinearRgb) -> Self {
        Self {
            x: Self::X_COEFFICIENTS.apply(linear_rgb),
            y: Self::Y_COEFFICIENTS.apply(linear_rgb),
            z: Self::Z_COEFFICIENTS.apply(linear_rgb),
        }
    }
}

impl From<Rgb> for Xyz {
    fn from(rgb: Rgb) -> Self {
        Self::from(LinearRgb::from(rgb))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Xy {
    x: u16,
    y: u16,
}

impl Xy {
    const MULTIPLIER: f32 = 65_535.0;

    /// Return the X value.
    #[must_use]
    pub const fn x(self) -> u16 {
        self.x
    }

    /// Return the Y value.
    #[must_use]
    pub const fn y(self) -> u16 {
        self.y
    }
}

impl From<Xyz> for Xy {
    fn from(xyz: Xyz) -> Self {
        let sum = xyz.x + xyz.y + xyz.z;

        if sum == 0.0 {
            Self { x: 0, y: 0 }
        } else {
            #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            Self {
                x: ((xyz.x / sum) * Self::MULTIPLIER).round() as u16,
                y: ((xyz.y / sum) * Self::MULTIPLIER).round() as u16,
            }
        }
    }
}

impl From<Rgb> for Xy {
    fn from(rgb: Rgb) -> Self {
        Self::from(Xyz::from(rgb))
    }
}

/// Coefficients for converting Linear RGB to a single channel of XYZ.
#[derive(Debug)]
struct LinearRgbToXyzCoefficients {
    red: f32,
    green: f32,
    blue: f32,
}

impl LinearRgbToXyzCoefficients {
    /// Create new coefficients.
    const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    /// Apply the coefficients to a Linear RGB color.
    pub fn apply(&self, linear_rgb: LinearRgb) -> f32 {
        self.apply_rgb(linear_rgb.red, linear_rgb.green, linear_rgb.blue)
    }

    /// Apply the coefficients to individual RGB components.
    fn apply_rgb(&self, red: f32, green: f32, blue: f32) -> f32 {
        red.mul_add(self.red, green.mul_add(self.green, blue * self.blue))
    }
}

#[inline]
fn correct_channel(channel: u8) -> f32 {
    let channel_f = f32::from(channel) / 255.0;
    if channel_f <= 0.04045 {
        channel_f / 12.92
    } else {
        ((channel_f + 0.055) / 1.055).powf(2.4)
    }
}
