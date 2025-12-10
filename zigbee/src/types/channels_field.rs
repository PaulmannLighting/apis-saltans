use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
#[repr(transparent)]
pub struct ChannelsField(u32);

bitflags! {
    impl ChannelsField: u32 {
        /// Channel page no. 1
        const CHANNEL_PAGE_1 = 0b0000_0000_0000_0000_0000_0000_0001_0000;
        /// Channel page no. 2
        const CHANNEL_PAGE_2 = 0b0000_0000_0000_0000_0000_0000_0000_1000;
        /// Channel page no. 3
        const CHANNEL_PAGE_3 = 0b0000_0000_0000_0000_0000_0000_0000_0100;
        /// Channel page no. 4
        const CHANNEL_PAGE_4 = 0b0000_0000_0000_0000_0000_0000_0000_0010;
        /// Channel page no. 5
        const CHANNEL_PAGE_5 = 0b0000_0000_0000_0000_0000_0000_0000_0001;
        /// Scan channel no. 1
        const SCAN_CHANNEL_1 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
        /// Scan channel no. 2
        const SCAN_CHANNEL_2 = 0b0100_0000_0000_0000_0000_0000_0000_0000;
        /// Scan channel no. 3
        const SCAN_CHANNEL_3 = 0b0010_0000_0000_0000_0000_0000_0000_0000;
        /// Scan channel no. 4
        const SCAN_CHANNEL_4 = 0b0001_0000_0000_0000_0000_0000_0000_0000;
        /// Scan channel no. 5
        const SCAN_CHANNEL_5 = 0b0000_1000_0000_0000_0000_0000_0000_0000;
        /// Scan channel no. 6
        const SCAN_CHANNEL_6 = 0b0000_0100_0000_0000_0000_0000_0000_0000;
        /// Scan channel no. 7
        const SCAN_CHANNEL_7 = 0b0000_0010_0000_0000_0000_0000_0000_0000;
        /// Scan channel no. 8
        const SCAN_CHANNEL_8 = 0b0000_0001_0000_0000_0000_0000_0000_0000;
        /// Scan channel no. 9
        const SCAN_CHANNEL_9 = 0b0000_0000_1000_0000_0000_0000_0000_0000;
        /// Scan channel no. 10
        const SCAN_CHANNEL_10 = 0b0000_0000_0100_0000_0000_0000_0000_0000;
        /// Scan channel no. 11
        const SCAN_CHANNEL_11 = 0b0000_0000_0010_0000_0000_0000_0000_0000;
        /// Scan channel no. 12
        const SCAN_CHANNEL_12 = 0b0000_0000_0001_0000_0000_0000_0000_0000;
        /// Scan channel no. 13
        const SCAN_CHANNEL_13 = 0b0000_0000_0000_1000_0000_0000_0000_0000;
        /// Scan channel no. 14
        const SCAN_CHANNEL_14 = 0b0000_0000_0000_0100_0000_0000_0000_0000;
        /// Scan channel no. 15
        const SCAN_CHANNEL_15 = 0b0000_0000_0000_0010_0000_0000_0000_0000;
        /// Scan channel no. 16
        const SCAN_CHANNEL_16 = 0b0000_0000_0000_0001_0000_0000_0000_0000;
        /// Scan channel no. 17
        const SCAN_CHANNEL_17 = 0b0000_0000_0000_0000_1000_0000_0000_0000;
        /// Scan channel no. 18
        const SCAN_CHANNEL_18 = 0b0000_0000_0000_0000_0100_0000_0000_0000;
        /// Scan channel no. 19
        const SCAN_CHANNEL_19 = 0b0000_0000_0000_0000_0010_0000_0000_0000;
        /// Scan channel no. 20
        const SCAN_CHANNEL_20 = 0b0000_0000_0000_0000_0001_0000_0000_0000;
        /// Scan channel no. 21
        const SCAN_CHANNEL_21 = 0b0000_0000_0000_0000_0000_1000_0000_0000;
        /// Scan channel no. 22
        const SCAN_CHANNEL_22 = 0b0000_0000_0000_0000_0000_0100_0000_0000;
        /// Scan channel no. 23
        const SCAN_CHANNEL_23 = 0b0000_0000_0000_0000_0000_0010_0000_0000;
        /// Scan channel no. 24
        const SCAN_CHANNEL_24 = 0b0000_0000_0000_0000_0000_0001_0000_0000;
        /// Scan channel no. 25
        const SCAN_CHANNEL_25 = 0b0000_0000_0000_0000_0000_0000_1000_0000;
        /// Scan channel no. 26
        const SCAN_CHANNEL_26 = 0b0000_0000_0000_0000_0000_0000_0100_0000;
        /// Scan channel no. 27
        const SCAN_CHANNEL_27 = 0b0000_0000_0000_0000_0000_0000_0010_0000;
    }
}

impl FromLeStream for ChannelsField {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u32::from_le_stream(&mut bytes).map(Self::from_bits_retain)
    }
}
