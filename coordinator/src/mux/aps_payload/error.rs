use thiserror::Error;

/// An error that can occur when parsing an APS frame.
#[derive(Clone, Debug, Eq, Error, PartialEq, Hash)]
pub enum ParseApsFrameError {
    /// The ZCL frame is invalid.
    #[error("{0}")]
    ParseZclFrameError(
        #[from]
        #[source]
        zb_zcl::ParseFrameError,
    ),

    /// The ZDP frame is invalid.
    #[error("{0}")]
    ParseZdpFrameError(
        #[from]
        #[source]
        zb_zdp::ParseFrameError,
    ),

    /// The profile ID is invalid.
    #[error("Invalid profile ID: {0}")]
    InvalidProfile(u16),
}

#[cfg(test)]
mod tests {
    use std::error::Error as _;

    use super::ParseApsFrameError;

    #[test]
    fn converted_frame_error_is_retained_as_source() {
        let error = ParseApsFrameError::from(zb_zcl::ParseFrameError::MissingHeader);

        assert_eq!(error.to_string(), "Missing ZCL frame header");
        assert_eq!(
            error.source().map(ToString::to_string),
            Some("Missing ZCL frame header".to_owned())
        );
    }
}
