use apis_saltans_core::ClusterId;

pub use self::enroll_request::EnrollRequest;
pub use self::enroll_response::EnrollResponse;
pub use self::initiate_normal_operation_mode::InitiateNormalOperationMode;
pub use self::initiate_test_mode::InitiateTestMode;
pub use self::status_change::StatusChange;
use crate::macros::zcl_command_enum;

mod enroll_request;
mod enroll_response;
mod initiate_normal_operation_mode;
mod initiate_test_mode;
mod status_change;

// IAS Zone cluster commands.
zcl_command_enum! {
    { ClusterId::IasZone } => IasZone;
    StatusChange(StatusChange),
    EnrollRequest(EnrollRequest),
    EnrollResponse(EnrollResponse),
    InitiateNormalOperationMode(InitiateNormalOperationMode),
    InitiateTestMode(InitiateTestMode),
}
