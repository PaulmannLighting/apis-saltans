use bytes::Bytes;
use le_stream::ToLeStream;
use zb_zcl::global::configure_reporting;
use zb_zcl::{Command, Reportable, Scoped, UnsequencedFrame, UnsequencedHeader};

/// Construct a global Configure Reporting frame scoped to one target cluster.
pub fn frame<T>(attributes: T) -> UnsequencedFrame<Bytes>
where
    T: IntoIterator<Item: Reportable>,
{
    UnsequencedFrame::new(
        UnsequencedHeader::new(
            configure_reporting::Send::SCOPE,
            <configure_reporting::Send as zb_zcl::Directed>::DIRECTION,
            configure_reporting::Send::DISABLE_DEFAULT_RESPONSE,
            <T::Item as Reportable>::MANUFACTURER_CODE,
            configure_reporting::Send::ID,
        ),
        configure_reporting::Send::new(attributes.into_iter().map(Into::into).collect())
            .to_le_stream()
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use zb_core::types::Bool;
    use zb_core::{Cluster, Direction, Profile};
    use zb_zcl::Discrete;
    use zb_zcl::on_off::SendReport;

    use super::frame;

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
}
