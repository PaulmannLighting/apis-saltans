use thiserror::Error;
use zb_aps::apsde::{ReceivedDestination, Source};
use zb_core::Endpoint;

/// An error that can occur when parsing an APSDE indication payload.
#[derive(Clone, Debug, Eq, Error, PartialEq, Hash)]
pub enum ParseApsPayloadError {
    /// The ZCL frame is invalid.
    #[error("{0}")]
    ParseZclFrameError(#[from] zb_zcl::ParseFrameError),

    /// The profile ID is invalid.
    #[error("Invalid profile ID: {0}")]
    InvalidProfile(u16),

    /// The received ZDP source did not include an endpoint.
    #[error("ZDP source addressing does not include an endpoint: {0:?}")]
    ZdpSourceAddressing(Source),

    /// The received ZDP source endpoint was not the data endpoint.
    #[error("ZDP source endpoint must be the data endpoint, got {0}")]
    ZdpSourceEndpoint(Endpoint),

    /// The received ZDP destination did not identify one endpoint.
    #[error("ZDP destination addressing does not identify one endpoint: {0:?}")]
    ZdpDestinationAddressing(ReceivedDestination),

    /// The received ZDP destination endpoint was not the data endpoint.
    #[error("ZDP destination endpoint must be the data endpoint, got {0}")]
    ZdpDestinationEndpoint(Endpoint),

    /// The cluster identifier did not select a supported ZDP command.
    #[error("Invalid cluster ID for ZDP frame: {0:#06x}")]
    ZdpClusterId(u16),

    /// The ZDP ASDU did not contain a complete frame.
    #[error("Invalid ZDP frame")]
    InvalidZdpFrame,
}

#[cfg(test)]
mod tests {
    use std::error::Error as _;

    use super::ParseApsPayloadError;

    #[test]
    fn converted_frame_error_is_retained_as_source() {
        let error = ParseApsPayloadError::from(zb_zcl::ParseFrameError::MissingHeader);

        assert_eq!(error.to_string(), "Missing ZCL frame header");
        assert_eq!(
            error.source().map(ToString::to_string),
            Some("Missing ZCL frame header".to_owned())
        );
    }
}
