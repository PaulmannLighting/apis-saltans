use apis_saltans_aps::data::Frame;
use apis_saltans_core::Profile;
use bytes::Bytes;

pub use self::error::ParseApsFrameError;

mod error;

type ZdpFrame = apis_saltans_zdp::Frame<apis_saltans_zdp::Command>;
type ZclFrame = apis_saltans_zcl::Frame<apis_saltans_zcl::Cluster>;

/// Commands received on the APS layer.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ApsPayload {
    /// A ZDP frame was received.
    Zdp(ZdpFrame),

    /// A ZCL command was received.
    Zcl(ZclFrame),
}

impl TryFrom<Frame<Bytes>> for ApsPayload {
    type Error = ParseApsFrameError;

    fn try_from(frame: Frame<Bytes>) -> Result<Self, Self::Error> {
        let profile = match frame.header().profile() {
            Ok(profile) => profile,
            Err(profile_id) => return Err(ParseApsFrameError::InvalidProfile(profile_id)),
        };

        match profile {
            Profile::Network => ZdpFrame::try_from(frame)
                .map(Self::Zdp)
                .map_err(ParseApsFrameError::ParseZdpFrameError),
            Profile::ZigbeeHomeAutomation
            | Profile::SmartEnergy
            | Profile::TouchLink
            | Profile::BuildingAutomation
            | Profile::HealthCare
            | Profile::RemoteControl => ZclFrame::try_from(frame)
                .map(Self::Zcl)
                .map_err(ParseApsFrameError::ParseZclFrameError),
        }
    }
}
