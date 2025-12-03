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
        fn correct_channel(channel: u8) -> f32 {
            let channel_f = f32::from(channel) / 255.0;
            if channel_f <= 0.04045 {
                channel_f / 12.92
            } else {
                ((channel_f + 0.055) / 1.055).powf(2.4)
            }
        }

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

impl From<LinearRgb> for Xyz {
    fn from(gamma_rgb: LinearRgb) -> Self {
        Self {
            x: gamma_rgb.blue.mul_add(
                0.180_437_5,
                gamma_rgb
                    .red
                    .mul_add(0.412_456_4, gamma_rgb.green * 0.357_576_1),
            ),
            y: gamma_rgb.blue.mul_add(
                0.072_175_0,
                gamma_rgb
                    .red
                    .mul_add(0.212_672_9, gamma_rgb.green * 0.715_152_2),
            ),
            z: gamma_rgb.blue.mul_add(
                0.950_304_1,
                gamma_rgb
                    .red
                    .mul_add(0.019_333_9, gamma_rgb.green * 0.119_192),
            ),
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
    /// Create a new `Xy` color.
    #[must_use]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// Return the X value.
    #[must_use]
    pub const fn x(&self) -> u16 {
        self.x
    }

    /// Return the Y value.
    #[must_use]
    pub const fn y(&self) -> u16 {
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
                x: ((xyz.x / sum) * 65_536.0).round() as u16,
                y: ((xyz.y / sum) * 65_536.0).round() as u16,
            }
        }
    }
}

impl From<Rgb> for Xy {
    fn from(rgb: Rgb) -> Self {
        Self::from(Xyz::from(rgb))
    }
}
