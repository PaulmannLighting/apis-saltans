use zb_zcl::global::default_response::DefaultResponse;
use zb_zcl::{Cluster, global};

use crate::{Error, StatusExt};

/// Whether a request permits a Default Response as its application-level outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefaultResponsePolicy {
    /// Require a command-specific response. Device rejections still terminate the request.
    SpecificRequired,

    /// Permit a Default Response, preserving its success, failure, or unknown status.
    DefaultAllowed,
}

/// The response returned by a ZCL transaction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ZclOutcome<T> {
    /// A command-specific response.
    Specific(T),

    /// A permitted Default Response; inspect its status to determine device success.
    Default(DefaultResponse),
}

impl<T> ZclOutcome<T> {
    /// Extract a required command-specific response.
    ///
    /// # Errors
    ///
    /// Returns a device status error for a failed Default Response, or
    /// [`Error::UnexpectedDefaultResponse`] if it reports success without the required data.
    pub fn into_specific(self) -> Result<T, Error> {
        match self {
            Self::Specific(response) => Ok(response),
            Self::Default(response) => {
                zb_zcl::Status::try_from(response.status()).ensure_success()?;
                Err(Error::UnexpectedDefaultResponse {
                    command_id: response.command_id(),
                })
            }
        }
    }
}

impl<T> TryFrom<Cluster> for ZclOutcome<T>
where
    T: TryFrom<Cluster>,
{
    type Error = T::Error;

    fn try_from(response: Cluster) -> Result<Self, Self::Error> {
        match response {
            Cluster::Global(global::Command::DefaultResponse(response)) => {
                Ok(Self::Default(*response))
            }
            response => T::try_from(response).map(Self::Specific),
        }
    }
}

#[cfg(test)]
mod tests {
    use zb_zcl::global::read_attributes;
    use zb_zcl::{Frame, Status};

    use super::*;

    const CLUSTER_ID: u16 = 0x0006;
    const SEQUENCE: u8 = 7;
    const FRAME_CONTROL: u8 = 0x18;
    const DEFAULT_ID: u8 = 0x0b;
    const REQUEST_ID: u8 = 0x00;
    const UNKNOWN_STATUS: u8 = 0xff;

    #[test]
    fn default_outcomes_preserve_success_failure_and_unknown_status() {
        for status in [Status::Success as u8, Status::Failure as u8, UNKNOWN_STATUS] {
            let frame = Frame::parse(
                CLUSTER_ID,
                [FRAME_CONTROL, SEQUENCE, DEFAULT_ID, REQUEST_ID, status].into_iter(),
            )
            .unwrap();
            let outcome =
                ZclOutcome::<read_attributes::Response>::try_from(frame.into_payload()).unwrap();
            let ZclOutcome::Default(response) = &outcome else {
                panic!("expected Default outcome");
            };
            assert_eq!(response.status(), status);
            assert_eq!(response.command_id(), REQUEST_ID);
            match outcome.into_specific() {
                Err(Error::UnexpectedDefaultResponse { command_id }) => {
                    assert_eq!(status, Status::Success as u8);
                    assert_eq!(command_id, REQUEST_ID);
                }
                Err(Error::Zcl(actual)) => assert_eq!(actual, Status::try_from(status)),
                _ => panic!("Default Response cannot supply a specific response"),
            }
        }
    }

    #[test]
    fn specific_outcomes_preserve_response_data() {
        const RESPONSE: [u8; 8] = [0x18, SEQUENCE, 0x01, 0x00, 0x00, 0x00, 0x10, 0x01];
        let frame = Frame::parse(CLUSTER_ID, RESPONSE.into_iter()).unwrap();
        let expected = read_attributes::Response::try_from(frame.payload().clone()).unwrap();
        let actual =
            ZclOutcome::<read_attributes::Response>::try_from(frame.into_payload()).unwrap();
        assert_eq!(actual.into_specific().unwrap(), expected);
    }
}
