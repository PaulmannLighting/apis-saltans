use zb_aps::TxOptions;
use zb_aps::apsde::{DataRequest, IndividualEndpoint, NetworkDestination, RequestParameters};
use zb_core::{ClusterSpecific, Profiled};
use zb_zcl::Writable;
use zb_zcl::global::write_attributes::{Command, Record};

use crate::ZclRequest;

/// Construct a typed global request and retain its attribute IDs in wire order.
pub fn request<T>(
    destination: NetworkDestination,
    source_endpoint: IndividualEndpoint,
    attributes: T,
) -> (ZclRequest<Command>, Vec<u16>)
where
    T: IntoIterator<Item: Writable>,
{
    let command = Command::new(attributes.into_iter().map(Into::into).collect());
    let ids = command.iter().map(Record::id).collect();
    let request = ZclRequest::new(
        DataRequest::new(
            RequestParameters {
                destination: destination.into(),
                profile_id: <T::Item as Profiled>::PROFILE.as_u16(),
                cluster_id: <T::Item as ClusterSpecific>::ID,
                source_endpoint,
            },
            command,
        )
        .with_tx_options(TxOptions::ACKNOWLEDGED_TRANSMISSION),
    )
    .with_manufacturer_code(<T::Item as Writable>::MANUFACTURER_CODE);
    (request, ids)
}
