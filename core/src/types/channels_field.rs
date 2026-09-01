use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

const CHANNEL_PAGE_SHIFT: u32 = 27;
const MAX_CHANNEL_PAGE: u8 = 31;

/// A Zigbee channel page and its channel-selection bitmap.
///
/// Bits 27 through 31 contain a binary-encoded channel page. Bits 0 through 26 select the
/// corresponding channels on that page.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
#[repr(transparent)]
pub struct ChannelsField(u32);

bitflags! {
    impl ChannelsField: u32 {
        /// Binary-encoded channel page mask.
        const CHANNEL_PAGE = 0b1111_1000_0000_0000_0000_0000_0000_0000;
        /// Mask containing every channel-selection bit.
        const SCAN_CHANNELS = 0b0000_0111_1111_1111_1111_1111_1111_1111;
        /// Scan channel 0.
        const SCAN_CHANNEL_0 = 0b0000_0000_0000_0000_0000_0000_0000_0001;
        /// Scan channel 1.
        const SCAN_CHANNEL_1 = 0b0000_0000_0000_0000_0000_0000_0000_0010;
        /// Scan channel 2.
        const SCAN_CHANNEL_2 = 0b0000_0000_0000_0000_0000_0000_0000_0100;
        /// Scan channel 3.
        const SCAN_CHANNEL_3 = 0b0000_0000_0000_0000_0000_0000_0000_1000;
        /// Scan channel 4.
        const SCAN_CHANNEL_4 = 0b0000_0000_0000_0000_0000_0000_0001_0000;
        /// Scan channel 5.
        const SCAN_CHANNEL_5 = 0b0000_0000_0000_0000_0000_0000_0010_0000;
        /// Scan channel 6.
        const SCAN_CHANNEL_6 = 0b0000_0000_0000_0000_0000_0000_0100_0000;
        /// Scan channel 7.
        const SCAN_CHANNEL_7 = 0b0000_0000_0000_0000_0000_0000_1000_0000;
        /// Scan channel 8.
        const SCAN_CHANNEL_8 = 0b0000_0000_0000_0000_0000_0001_0000_0000;
        /// Scan channel 9.
        const SCAN_CHANNEL_9 = 0b0000_0000_0000_0000_0000_0010_0000_0000;
        /// Scan channel 10.
        const SCAN_CHANNEL_10 = 0b0000_0000_0000_0000_0000_0100_0000_0000;
        /// Scan channel 11.
        const SCAN_CHANNEL_11 = 0b0000_0000_0000_0000_0000_1000_0000_0000;
        /// Scan channel 12.
        const SCAN_CHANNEL_12 = 0b0000_0000_0000_0000_0001_0000_0000_0000;
        /// Scan channel 13.
        const SCAN_CHANNEL_13 = 0b0000_0000_0000_0000_0010_0000_0000_0000;
        /// Scan channel 14.
        const SCAN_CHANNEL_14 = 0b0000_0000_0000_0000_0100_0000_0000_0000;
        /// Scan channel 15.
        const SCAN_CHANNEL_15 = 0b0000_0000_0000_0000_1000_0000_0000_0000;
        /// Scan channel 16.
        const SCAN_CHANNEL_16 = 0b0000_0000_0000_0001_0000_0000_0000_0000;
        /// Scan channel 17.
        const SCAN_CHANNEL_17 = 0b0000_0000_0000_0010_0000_0000_0000_0000;
        /// Scan channel 18.
        const SCAN_CHANNEL_18 = 0b0000_0000_0000_0100_0000_0000_0000_0000;
        /// Scan channel 19.
        const SCAN_CHANNEL_19 = 0b0000_0000_0000_1000_0000_0000_0000_0000;
        /// Scan channel 20.
        const SCAN_CHANNEL_20 = 0b0000_0000_0001_0000_0000_0000_0000_0000;
        /// Scan channel 21.
        const SCAN_CHANNEL_21 = 0b0000_0000_0010_0000_0000_0000_0000_0000;
        /// Scan channel 22.
        const SCAN_CHANNEL_22 = 0b0000_0000_0100_0000_0000_0000_0000_0000;
        /// Scan channel 23.
        const SCAN_CHANNEL_23 = 0b0000_0000_1000_0000_0000_0000_0000_0000;
        /// Scan channel 24.
        const SCAN_CHANNEL_24 = 0b0000_0001_0000_0000_0000_0000_0000_0000;
        /// Scan channel 25.
        const SCAN_CHANNEL_25 = 0b0000_0010_0000_0000_0000_0000_0000_0000;
        /// Scan channel 26.
        const SCAN_CHANNEL_26 = 0b0000_0100_0000_0000_0000_0000_0000_0000;
    }
}

impl_bitflags_display_and_from_str!(ChannelsField);

impl ChannelsField {
    /// Deprecated encoded representation of channel page 1.
    #[deprecated(note = "use with_page")]
    pub const CHANNEL_PAGE_1: Self = Self(0b0000_1000_0000_0000_0000_0000_0000_0000);
    /// Deprecated encoded representation of channel page 2.
    #[deprecated(note = "use with_page")]
    pub const CHANNEL_PAGE_2: Self = Self(0b0001_0000_0000_0000_0000_0000_0000_0000);
    /// Deprecated encoded representation of channel page 3.
    #[deprecated(note = "use with_page")]
    pub const CHANNEL_PAGE_3: Self = Self(0b0001_1000_0000_0000_0000_0000_0000_0000);
    /// Deprecated encoded representation of channel page 4.
    #[deprecated(note = "use with_page")]
    pub const CHANNEL_PAGE_4: Self = Self(0b0010_0000_0000_0000_0000_0000_0000_0000);
    /// Deprecated encoded representation of channel page 5.
    #[deprecated(note = "use with_page")]
    pub const CHANNEL_PAGE_5: Self = Self(0b0010_1000_0000_0000_0000_0000_0000_0000);

    /// Returns the binary-encoded channel page.
    #[must_use]
    pub const fn page(self) -> u8 {
        ((self.bits() & Self::CHANNEL_PAGE.bits()) >> CHANNEL_PAGE_SHIFT) as u8
    }

    /// Returns this value with its channel page set to `page`.
    ///
    /// # Errors
    ///
    /// Returns `page` if it does not fit in the five-bit channel page field.
    pub fn with_page(self, page: u8) -> Result<Self, u8> {
        if page > MAX_CHANNEL_PAGE {
            return Err(page);
        }

        Ok(Self(
            (self.bits() & !Self::CHANNEL_PAGE.bits()) | (u32::from(page) << CHANNEL_PAGE_SHIFT),
        ))
    }

    /// Returns only the channel-selection bits, without the encoded page.
    #[must_use]
    pub const fn channels(self) -> Self {
        self.intersection(Self::SCAN_CHANNELS)
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

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::vec;
    use alloc::vec::Vec;

    use le_stream::ToLeStream;

    use super::ChannelsField;

    #[test]
    fn encodes_page_and_channels_at_their_wire_positions() {
        let channels = ChannelsField::SCAN_CHANNEL_0
            | ChannelsField::SCAN_CHANNEL_11
            | ChannelsField::SCAN_CHANNEL_26;
        let field = channels.with_page(28).expect("page fits in five bits");

        assert_eq!(field.page(), 28);
        assert_eq!(field.channels(), channels);
        assert_eq!(field.bits(), 0b1110_0100_0000_0000_0000_1000_0000_0001);

        let bytes: Vec<_> = field.to_le_stream().collect();
        assert_eq!(bytes, vec![0x01, 0x08, 0x00, 0xE4]);
    }

    #[test]
    fn rejects_page_that_does_not_fit() {
        assert_eq!(ChannelsField::empty().with_page(32), Err(32));
    }
}
