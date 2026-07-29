//! Conversion helpers for normalized APSDE indications.

use zb_aps::apsde::{
    IndicationMetadata, ReceivedDestination, SecurityStatus, Source as ApsdeSource,
};
use zb_aps::data::Header;
use zb_aps::{Data, Destination};
use zb_nwk::Source;

// APSDE-DATA.indication does not expose the received wire-frame counter. Legacy coordinator event
// and subscription representations do not use that counter for routing or response correlation.
const UNAVAILABLE_APS_COUNTER: u8 = 0;

/// Derive legacy NWK source information and an APS header from indication metadata.
pub fn legacy_context<T, K>(metadata: &IndicationMetadata<T, K>) -> Option<(Source, Header)> {
    let (source, source_endpoint) = match metadata.source() {
        ApsdeSource::Network { address, endpoint } => {
            (Source::new(address.as_u16(), None), endpoint.get())
        }
        ApsdeSource::Extended { .. } | ApsdeSource::ExtendedWithoutEndpoint(_) => return None,
    };
    let destination = match metadata.destination() {
        ReceivedDestination::Group(address) => Destination::Group(address.as_u16()),
        ReceivedDestination::Network { endpoint, .. }
        | ReceivedDestination::Extended { endpoint, .. } => Destination::Unicast(endpoint.get()),
        ReceivedDestination::ExtendedWithoutEndpoint(_) => return None,
    };
    let mut header = Header::new(
        destination,
        metadata.cluster_id(),
        metadata.profile_id(),
        source_endpoint,
        UNAVAILABLE_APS_COUNTER,
        None,
    );
    header.set_security(metadata.security().status() != SecurityStatus::Unsecured);

    Some((source, header))
}

/// Convert a normalized indication into the coordinator's legacy APS data representation.
pub fn into_legacy_data<A>(
    indication: zb_aps::apsde::DataIndication<A, (), ()>,
) -> Option<(Source, Data<A>)> {
    let (source, header) = legacy_context(indication.metadata())?;
    let (_, asdu) = indication.into_parts();
    Some((source, Data::new(header, asdu)))
}
