//! Common types used across the protocol.

pub use self::analog::{
    Int8, Int16, Int24, Int32, Int40, Int48, Int56, Int64, Uint8, Uint16, Uint24, Uint32, Uint40,
    Uint48, Uint56, Uint64,
};
pub use self::channel_list::ChannelList;
pub use self::channels_field::ChannelsField;
pub use self::composite::{OctStr, String};
pub use self::discrete::{
    Bool, Data8, Data16, Data24, Data32, Data40, Data48, Data56, Data64, Date, TimeOfDay,
    TryFromNaiveDateError, TryFromNaiveTimeError, TryIntoNaiveDateError, TryIntoNaiveTimeError,
    UtcTime,
};
pub use self::null::{NoData, Unknown};

mod analog;
mod channel_list;
mod channels_field;
mod composite;
mod discrete;
mod null;
pub mod tlv;
