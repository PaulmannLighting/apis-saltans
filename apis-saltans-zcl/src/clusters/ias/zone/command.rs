use apis_saltans_core::ClusterId;

pub use self::status_change::StatusChange;
use crate::macros::zcl_command_enum;

mod status_change;

// IAS Zone cluster commands.
zcl_command_enum! {
    { ClusterId::IasZone } => IasZone;
    StatusChange(StatusChange),
}
