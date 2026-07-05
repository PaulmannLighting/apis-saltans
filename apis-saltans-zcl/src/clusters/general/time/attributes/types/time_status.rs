use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// Time status attribute for the Time cluster.
    pub bitflags TimeStatus(u8) => Map8 {
        /// Indicates whether this is a master clock.
        const MASTER = 0b0000_0001;
        /// Indicates whether the time is synchronized.
        const SYNCHRONIZED = 0b0000_0010;
        /// Indicates whether this is a master clock for time zone and DST.
        const MASTER_ZONE_DST = 0b0000_0100;
        /// Indicates whether time synchronization should be superseded.
        const SUPERSEDING = 0b0000_1000;
    }
}
