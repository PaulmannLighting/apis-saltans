//! Discrete data types.

pub use bool::Bool;
pub use data8::Data8;
pub use data16::Data16;
pub use data24::Data24;
pub use data32::Data32;
pub use data40::Data40;
pub use data48::Data48;
pub use data56::Data56;
pub use data64::Data64;
pub use date::{Date, TryFromDateError, TryFromNaiveDateError};
pub use time_of_day::{Error as TimeOfDayError, TimeOfDay};
pub use utc_time::UtcTime;

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
