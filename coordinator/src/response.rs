//! Deferred transmission and protocol response futures.

pub use self::communication_response::CommunicationResponse;
pub use self::internal_communication_response::InternalCommunicationResponse;

mod communication_response;
mod internal_communication_response;
