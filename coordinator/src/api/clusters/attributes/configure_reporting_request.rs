use zb_aps::TxOptions;
use zb_aps::apsde::{DataRequest, IndividualEndpoint, NetworkDestination};
use zb_core::{ClusterSpecific, Profiled};
use zb_zcl::Reportable;
use zb_zcl::global::configure_reporting;

use crate::ZclRequest;

/// Construct a typed global request with the attributes' cluster and manufacturer context.
pub fn request<T>(
    destination: NetworkDestination,
    source_endpoint: IndividualEndpoint,
    attributes: T,
) -> ZclRequest<configure_reporting::Send>
where
    T: IntoIterator<Item: Reportable>,
{
    ZclRequest::new(
        DataRequest::new(
            destination.into(),
            <T::Item as Profiled>::PROFILE.as_u16(),
            <T::Item as ClusterSpecific>::ID,
            source_endpoint,
            configure_reporting::Send::new(attributes.into_iter().map(Into::into).collect()),
        )
        .with_tx_options(TxOptions::ACKNOWLEDGED_TRANSMISSION),
    )
    .with_manufacturer_code(<T::Item as Reportable>::MANUFACTURER_CODE)
}

#[cfg(test)]
mod tests {
    use zb_core::types::Bool;
    use zb_core::{Cluster, Direction, Profile};
    use zb_zcl::Discrete;
    use zb_zcl::on_off::SendReport;

    use super::request;

    const ENDPOINT: zb_core::Endpoint =
        zb_core::Endpoint::Application(zb_core::endpoint::Application::MIN);
    const ADDRESS: u16 = 0x1234;

    fn frame<T>(attributes: T) -> zb_zcl::UnsequencedFrame<bytes::Bytes>
    where
        T: IntoIterator<Item: zb_zcl::Reportable>,
    {
        let endpoint = zb_aps::apsde::IndividualEndpoint::new(ENDPOINT).unwrap();
        let destination = zb_aps::apsde::NetworkDestination::new(
            zb_aps::apsde::NetworkAddress::new(ADDRESS).unwrap(),
            endpoint,
        );
        request(destination, endpoint, attributes)
            .as_request()
            .asdu()
            .clone()
    }

    const ATTRIBUTE_ID: u16 = 0x0000;
    const TYPE_ID: u8 = 0x10;
    const MINIMUM_REPORTING_INTERVAL: u16 = 10;
    const MAXIMUM_REPORTING_INTERVAL: u16 = 60;

    #[test]
    fn derives_request_metadata_and_attribute_ids_from_reportable() {
        let frame = frame([
            SendReport::OnOff(Discrete::<Bool>::new(
                MINIMUM_REPORTING_INTERVAL,
                MAXIMUM_REPORTING_INTERVAL,
            )),
            SendReport::OnOff(Discrete::<Bool>::new(
                MINIMUM_REPORTING_INTERVAL,
                MAXIMUM_REPORTING_INTERVAL,
            )),
        ]);

        let manufacturer_code = frame.header().manufacturer_code();
        let bytes = frame.into_payload();
        let mut record = vec![Direction::ClientToServer as u8];
        record.extend(ATTRIBUTE_ID.to_le_bytes());
        record.push(TYPE_ID);
        record.extend(MINIMUM_REPORTING_INTERVAL.to_le_bytes());
        record.extend(MAXIMUM_REPORTING_INTERVAL.to_le_bytes());
        let expected = [record.as_slice(), record.as_slice()].concat();

        assert_eq!(
            <SendReport as zb_core::Profiled>::PROFILE,
            Profile::ZigbeeHomeAutomation
        );
        assert_eq!(
            <SendReport as zb_core::ClusterSpecific>::ID,
            Cluster::OnOff.as_u16()
        );
        assert_eq!(manufacturer_code, None);
        assert_eq!(bytes.as_ref(), expected);
    }

    #[test]
    fn configures_reporting_without_mandatory_reporting_access() {
        use zb_core::types::Uint16;
        use zb_zcl::Analog;
        use zb_zcl::level::SendReport;

        const CHANGE: Uint16 = Uint16::new(2);

        let frame = frame([SendReport::RemainingTime(Analog::new(
            MINIMUM_REPORTING_INTERVAL,
            MAXIMUM_REPORTING_INTERVAL,
            CHANGE,
        ))]);

        assert_eq!(
            frame.into_payload().as_ref(),
            [0x00, 0x01, 0x00, 0x21, 10, 0, 60, 0, 2, 0]
        );
    }
}
