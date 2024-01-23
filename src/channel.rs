use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

pub const MASK_NONE: u32 = 0x0000_0000;
pub const MASK_ALL: u32 = 0x07FF_FFFF;
pub const MASK_2GHZ: u32 = 0x07FF_F800;

#[derive(Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Channel {
    Channel00 = 0x0000_0001,
    Channel01 = 0x0000_0002,
    Channel02 = 0x0000_0004,
    Channel03 = 0x0000_0008,
    Channel04 = 0x0000_0010,
    Channel05 = 0x0000_0020,
    Channel06 = 0x0000_0040,
    Channel07 = 0x0000_0080,
    Channel08 = 0x0000_0100,
    Channel09 = 0x0000_0200,
    Channel10 = 0x0000_0400,
    Channel11 = 0x0000_0800,
    Channel12 = 0x0000_1000,
    Channel13 = 0x0000_2000,
    Channel14 = 0x0000_4000,
    Channel15 = 0x0000_8000,
    Channel16 = 0x0001_0000,
    Channel17 = 0x0002_0000,
    Channel18 = 0x0004_0000,
    Channel19 = 0x0008_0000,
    Channel20 = 0x0010_0000,
    Channel21 = 0x0020_0000,
    Channel22 = 0x0040_0000,
    Channel23 = 0x0080_0000,
    Channel24 = 0x0100_0000,
    Channel25 = 0x0200_0000,
    Channel26 = 0x0400_0000,
}

/// Return the channel number.
impl From<Channel> for u8 {
    fn from(channel: Channel) -> Self {
        match channel {
            Channel::Channel00 => 0,
            Channel::Channel01 => 1,
            Channel::Channel02 => 2,
            Channel::Channel03 => 3,
            Channel::Channel04 => 4,
            Channel::Channel05 => 5,
            Channel::Channel06 => 6,
            Channel::Channel07 => 7,
            Channel::Channel08 => 8,
            Channel::Channel09 => 9,
            Channel::Channel10 => 10,
            Channel::Channel11 => 11,
            Channel::Channel12 => 12,
            Channel::Channel13 => 13,
            Channel::Channel14 => 14,
            Channel::Channel15 => 15,
            Channel::Channel16 => 16,
            Channel::Channel17 => 17,
            Channel::Channel18 => 18,
            Channel::Channel19 => 19,
            Channel::Channel20 => 20,
            Channel::Channel21 => 21,
            Channel::Channel22 => 22,
            Channel::Channel23 => 23,
            Channel::Channel24 => 24,
            Channel::Channel25 => 25,
            Channel::Channel26 => 26,
        }
    }
}

/// Return the channel mask.
impl From<Channel> for u32 {
    fn from(channel: Channel) -> Self {
        channel.to_u32().expect("Could not convert Channel to u32")
    }
}

/// Attempt to get a channel from a channel number.
impl TryFrom<u8> for Channel {
    type Error = u8;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(Self::Channel00),
            1 => Ok(Self::Channel01),
            2 => Ok(Self::Channel02),
            3 => Ok(Self::Channel03),
            4 => Ok(Self::Channel04),
            5 => Ok(Self::Channel05),
            6 => Ok(Self::Channel06),
            7 => Ok(Self::Channel07),
            8 => Ok(Self::Channel08),
            9 => Ok(Self::Channel09),
            10 => Ok(Self::Channel10),
            11 => Ok(Self::Channel11),
            12 => Ok(Self::Channel12),
            13 => Ok(Self::Channel13),
            14 => Ok(Self::Channel14),
            15 => Ok(Self::Channel15),
            16 => Ok(Self::Channel16),
            17 => Ok(Self::Channel17),
            18 => Ok(Self::Channel18),
            19 => Ok(Self::Channel19),
            20 => Ok(Self::Channel20),
            21 => Ok(Self::Channel21),
            22 => Ok(Self::Channel22),
            23 => Ok(Self::Channel23),
            24 => Ok(Self::Channel24),
            25 => Ok(Self::Channel25),
            26 => Ok(Self::Channel26),
            n => Err(n),
        }
    }
}

/// Attempt to get a channel from a channel mask.
impl TryFrom<u32> for Channel {
    type Error = u32;

    fn try_from(n: u32) -> Result<Self, Self::Error> {
        Self::from_u32(n).ok_or(n)
    }
}
