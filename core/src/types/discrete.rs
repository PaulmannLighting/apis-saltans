//! Discrete data types.

pub use self::bool::Bool;
pub use self::data8::Data8;
pub use self::data16::Data16;
pub use self::data24::Data24;
pub use self::data32::Data32;
pub use self::data40::Data40;
pub use self::data48::Data48;
pub use self::data56::Data56;
pub use self::data64::Data64;
pub use self::date::{Date, TryFromNaiveDateError, TryIntoNaiveDateError};
pub use self::time_of_day::{TimeOfDay, TryFromNaiveTimeError, TryIntoNaiveTimeError};
pub use self::utc_time::UtcTime;

mod bool;
mod data16;
mod data24;
mod data32;
mod data40;
mod data48;
mod data56;
mod data64;
mod data8;
mod date;
mod time_of_day;
mod utc_time;
