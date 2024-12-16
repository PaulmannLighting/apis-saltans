use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveToHue {
    hue: u8,
    direction: Direction,
    transition_time: u16,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Direction {
    ShortestDistance = 0x00,
    LongestDistance = 0x01,
    Up = 0x02,
    Down = 0x03,
}
