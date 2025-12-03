use serde::{Deserialize, Serialize};
use zcl::lighting::color_control::MoveToColor;

use self::color::Rgb;
use crate::web_api::color_move::color::Xy;

mod color;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ColorMove {
    rgb: Rgb,
    rate: u16,
}

impl ColorMove {
    fn xy(self) -> (u16, u16) {
        let xy: Xy = self.rgb.into();
        (xy.x(), xy.y())
    }
}

impl From<ColorMove> for MoveToColor {
    fn from(color_move: ColorMove) -> Self {
        let (x, y) = color_move.xy();
        Self::new(x, y, color_move.rate, 0x00, 0x00)
    }
}
