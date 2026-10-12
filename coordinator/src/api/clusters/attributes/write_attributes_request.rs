use zb_aps::TxOptions;
use zb_aps::apsde::{DataRequest, IndividualEndpoint, NetworkDestination};
use zb_core::{ClusterSpecific, Profiled};
use zb_zcl::Writable;
use zb_zcl::global::write_attributes::Command;

use crate::ZclRequest;

/// Construct a typed global request with the attributes' cluster and manufacturer context.
pub fn request<T>(
    destination: NetworkDestination,
    source_endpoint: IndividualEndpoint,
    attributes: T,
) -> ZclRequest<Command>
where
    T: IntoIterator<Item: Writable>,
{
    ZclRequest::new(
        DataRequest::new(
            destination.into(),
            <T::Item as Profiled>::PROFILE.as_u16(),
            <T::Item as ClusterSpecific>::ID,
            source_endpoint,
            Command::new(attributes.into_iter().map(Into::into).collect()),
        )
        .with_tx_options(TxOptions::ACKNOWLEDGED_TRANSMISSION),
    )
    .with_manufacturer_code(<T::Item as Writable>::MANUFACTURER_CODE)
}
