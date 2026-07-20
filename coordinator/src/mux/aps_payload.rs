use bytes::Bytes;
use zb_aps::data::Frame;
use zb_core::{Cluster, Profile};

pub use self::error::ParseApsFrameError;

mod error;

type ZdpFrame = zb_zdp::Frame<zb_zdp::Command>;
type ZclFrame = zb_zcl::Frame<zb_zcl::Cluster>;

/// Payloads received on the APS layer.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ApsPayload {
    /// A ZDP frame was received.
    Zdp(ZdpFrame),

    /// A ZCL command was received.
    Zcl(ZclFrame),

    /// A Keep-Alive packet was received.
    KeepAlive,
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
            | Profile::RemoteControl => match frame.header().cluster() {
                Ok(Cluster::KeepAlive) => Ok(Self::KeepAlive),
                _ => ZclFrame::try_from(frame)
                    .map(Self::Zcl)
                    .map_err(ParseApsFrameError::ParseZclFrameError),
            },
        }
    }
}
