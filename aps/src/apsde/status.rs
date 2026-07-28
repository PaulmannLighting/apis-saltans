use core::fmt::{Formatter, LowerHex, UpperHex};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

/// APS sublayer status value.
#[derive(
    Clone, Copy, Debug, Eq, Error, Hash, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive,
)]
#[num_enum(error_type(name = u8, constructor = core::convert::identity))]
#[repr(u8)]
pub enum Status {
    /// The request completed successfully.
    #[error("SUCCESS")]
    Success = 0x00,

    /// The ASDU is too large and cannot be fragmented.
    #[error("ASDU_TOO_LONG")]
    AsduTooLong = 0xa0,

    /// Defragmentation cannot proceed at the current time.
    #[error("DEFRAG_DEFERRED")]
    DefragmentationDeferred = 0xa1,

    /// The receiving device does not support defragmentation.
    #[error("DEFRAG_UNSUPPORTED")]
    DefragmentationUnsupported = 0xa2,

    /// A requested operation or parameter is outside the permitted range.
    #[error("ILLEGAL_REQUEST")]
    IllegalRequest = 0xa3,

    /// The requested binding does not exist.
    #[error("INVALID_BINDING")]
    InvalidBinding = 0xa4,

    /// The requested group does not exist.
    #[error("INVALID_GROUP")]
    InvalidGroup = 0xa5,

    /// A parameter is invalid or outside its range.
    #[error("INVALID_PARAMETER")]
    InvalidParameter = 0xa6,

    /// An acknowledged transmission received no APS acknowledgement.
    #[error("NO_ACK")]
    NoAcknowledgement = 0xa7,

    /// Binding-table destination resolution found no peer.
    #[error("NO_BOUND_DEVICE")]
    NoBoundDevice = 0xa8,

    /// An IEEE destination could not be mapped to a short address.
    #[error("NO_SHORT_ADDRESS")]
    NoShortAddress = 0xa9,

    /// The requested operation is not supported.
    #[error("NOT_SUPPORTED")]
    NotSupported = 0xaa,

    /// The received ASDU was secured using an APS link key.
    #[error("SECURED_LINK_KEY")]
    SecuredLinkKey = 0xab,

    /// The received ASDU was secured using the NWK key.
    #[error("SECURED_NWK_KEY")]
    SecuredNetworkKey = 0xac,

    /// APS security processing failed.
    #[error("SECURITY_FAIL")]
    SecurityFailure = 0xad,

    /// An APS management table has no free entry.
    #[error("TABLE_FULL")]
    TableFull = 0xae,

    /// The received ASDU was not secured.
    #[error("UNSECURED")]
    Unsecured = 0xaf,

    /// The requested APS information-base attribute is unknown.
    #[error("UNSUPPORTED_ATTRIBUTE")]
    UnsupportedAttribute = 0xb0,

    /// The target cannot accept a required fragmented transmission.
    #[error("PEER_CANNOT_FRAGMENT")]
    PeerCannotFragment = 0xb1,

    /// The target did not report its fragmentation support.
    #[error("UNKNOWN_FRAGMENT_SUPPORT")]
    UnknownFragmentSupport = 0xb2,
}

impl Status {
    /// Return whether this status indicates success.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Success)
    }
}

impl UpperHex for Status {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        UpperHex::fmt(&u8::from(*self), formatter)
    }
}

impl LowerHex for Status {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        LowerHex::fmt(&u8::from(*self), formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::Status;

    #[test]
    fn status_values_round_trip_through_their_protocol_codes() {
        assert_eq!(u8::from(Status::NoAcknowledgement), 0xa7);
        assert_eq!(Status::try_from(0xb2), Ok(Status::UnknownFragmentSupport));
        assert_eq!(Status::try_from(0xff), Err(0xff));
    }
}
