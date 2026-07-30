use bytes::Bytes;
use zb_aps::apsde::{IndicationMetadata, ReceivedDestination};
use zb_core::{Cluster, Endpoint, Profile};

pub use self::error::ParseApsPayloadError;

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

impl ApsPayload {
    /// Parse one APSDE indication ASDU using its protocol metadata.
    ///
    /// # Errors
    ///
    /// Returns an error when the profile, ZDP addressing, cluster identifier, or encoded protocol
    /// frame is invalid.
    pub fn parse<T, K>(
        metadata: &IndicationMetadata<T, K>,
        asdu: Bytes,
    ) -> Result<Self, ParseApsPayloadError> {
        let profile = match metadata.profile() {
            Ok(profile) => profile,
            Err(profile_id) => return Err(ParseApsPayloadError::InvalidProfile(profile_id)),
        };

        match profile {
            Profile::Network => Self::parse_zdp(metadata, asdu),
            Profile::ZigbeeHomeAutomation
            | Profile::SmartEnergy
            | Profile::TouchLink
            | Profile::BuildingAutomation
            | Profile::HealthCare
            | Profile::RemoteControl => match metadata.cluster() {
                Ok(Cluster::KeepAlive) => Ok(Self::KeepAlive),
                _ => ZclFrame::parse(metadata.cluster_id(), asdu.into_iter())
                    .map(Self::Zcl)
                    .map_err(ParseApsPayloadError::ParseZclFrameError),
            },
        }
    }

    fn parse_zdp<T, K>(
        metadata: &IndicationMetadata<T, K>,
        asdu: Bytes,
    ) -> Result<Self, ParseApsPayloadError> {
        let source_endpoint = metadata
            .source()
            .endpoint()
            .ok_or_else(|| ParseApsPayloadError::ZdpSourceAddressing(metadata.source()))?
            .get();
        if source_endpoint != Endpoint::Data {
            return Err(ParseApsPayloadError::ZdpSourceEndpoint(source_endpoint));
        }

        let destination = metadata.destination();
        let destination_endpoint = match destination {
            ReceivedDestination::Broadcast { endpoint, .. } => endpoint,
            ReceivedDestination::Network { endpoint, .. }
            | ReceivedDestination::Extended { endpoint, .. } => endpoint.get(),
            ReceivedDestination::Group(_) | ReceivedDestination::ExtendedWithoutEndpoint(_) => {
                return Err(ParseApsPayloadError::ZdpDestinationAddressing(destination));
            }
        };
        if destination_endpoint != Endpoint::Data {
            return Err(ParseApsPayloadError::ZdpDestinationEndpoint(
                destination_endpoint,
            ));
        }

        ZdpFrame::parse_with_cluster_id(metadata.cluster_id(), asdu.into_iter())
            .map_err(ParseApsPayloadError::ZdpClusterId)?
            .map(Self::Zdp)
            .ok_or(ParseApsPayloadError::InvalidZdpFrame)
    }
}
