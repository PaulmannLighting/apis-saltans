use aps::data::Frame;
use zigbee::Profile;

pub use self::error::ParseApsFrameError;

mod error;

/// Commands received on the APS layer.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ApsPayload {
    /// A ZDP frame was received.
    Zdp(zdp::Frame<zdp::Command>),

    /// A ZCL command was received.
    Zcl(zcl::Frame<zcl::Cluster>),
}

impl TryFrom<Frame<Vec<u8>>> for ApsPayload {
    type Error = ParseApsFrameError;

    fn try_from(frame: Frame<Vec<u8>>) -> Result<Self, Self::Error> {
        let profile = match frame.header().profile() {
            Ok(profile) => profile,
            Err(profile_id) => return Err(ParseApsFrameError::InvalidProfile(profile_id)),
        };

        match profile {
            Profile::Network => zdp::Frame::<zdp::Command>::try_from(frame)
                .map(Self::Zdp)
                .map_err(ParseApsFrameError::ParseZdpFrameError),
            Profile::ZigbeeHomeAutomation
            | Profile::SmartEnergy
            | Profile::TouchLink
            | Profile::BuildingAutomation
            | Profile::HealthCare
            | Profile::RemoteControl => zcl::Frame::<zcl::Cluster>::try_from(frame)
                .map(Self::Zcl)
                .map_err(ParseApsFrameError::ParseZclFrameError),
        }
    }
}
