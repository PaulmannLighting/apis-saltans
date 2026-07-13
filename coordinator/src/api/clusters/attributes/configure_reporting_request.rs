use std::marker::PhantomData;

use le_stream::ToLeStream;
use zb_core::ExpectResponse;
use zb_core::types::Type;
use zb_zcl::global::configure_reporting;
use zb_zcl::{Cluster, Command, Reportable, Scoped};

use crate::transceiver::zcl::{Metadata, Payload};

/// Global Configure Reporting request scoped to one target cluster.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ConfigureReportingRequest<T> {
    configurations: Box<[configure_reporting::send::AttributeReportingConfiguration]>,
    attribute: PhantomData<T>,
}

impl<T> ConfigureReportingRequest<T>
where
    T: Reportable,
{
    pub(super) fn new<I>(
        attributes: I,
        minimum_reporting_interval: u16,
        maximum_reporting_interval: u16,
        reportable_change: Option<&Type>,
    ) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            configurations: attributes
                .into_iter()
                .map(|attribute| {
                    configure_reporting::send::AttributeReportingConfiguration::new(
                        attribute.attribute_id(),
                        attribute.type_id(),
                        minimum_reporting_interval,
                        maximum_reporting_interval,
                        reportable_change.cloned(),
                    )
                })
                .collect(),
            attribute: PhantomData,
        }
    }
}

impl<T> ExpectResponse<Cluster> for ConfigureReportingRequest<T> {
    type Response = configure_reporting::Response;
}

impl<T> From<ConfigureReportingRequest<T>> for Payload
where
    T: Reportable,
{
    fn from(request: ConfigureReportingRequest<T>) -> Self {
        Self::new(
            zb_hw::Metadata::new(T::PROFILE, T::ID),
            Metadata {
                scope: configure_reporting::Send::SCOPE,
                direction: <configure_reporting::Send as zb_zcl::Directed>::DIRECTION,
                disable_default_response: configure_reporting::Send::DISABLE_DEFAULT_RESPONSE,
                manufacturer_code: T::MANUFACTURER_CODE,
                command_id: configure_reporting::Send::ID,
            },
            configure_reporting::Send::new(request.configurations)
                .to_le_stream()
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use zb_core::types::{Bool, Type};
    use zb_core::{Cluster, Direction, Profile};
    use zb_zcl::on_off::Types;

    use super::ConfigureReportingRequest;
    use crate::transceiver::zcl::Payload;

    const ATTRIBUTE_ID: u16 = 0x0000;
    const TYPE_ID: u8 = 0x10;
    const MINIMUM_REPORTING_INTERVAL: u16 = 10;
    const MAXIMUM_REPORTING_INTERVAL: u16 = 60;

    #[test]
    fn derives_request_metadata_and_attribute_ids_from_reportable() {
        let request = ConfigureReportingRequest::new(
            [
                Types::OnOff(Type::Boolean(Bool::TRUE)),
                Types::OnOff(Type::Boolean(Bool::FALSE)),
            ],
            MINIMUM_REPORTING_INTERVAL,
            MAXIMUM_REPORTING_INTERVAL,
            None,
        );
        let configurations = &request.configurations;

        assert_eq!(configurations.len(), 2);
        for configuration in configurations {
            assert_eq!(configuration.attribute_id(), ATTRIBUTE_ID);
            assert_eq!(configuration.attribute_data_type(), TYPE_ID);
            assert_eq!(
                configuration.minimum_reporting_interval(),
                MINIMUM_REPORTING_INTERVAL
            );
            assert_eq!(
                configuration.maximum_reporting_interval(),
                MAXIMUM_REPORTING_INTERVAL
            );
        }

        let (aps, zcl, bytes) = Payload::from(request).into_parts();
        let mut record = vec![Direction::ClientToServer as u8];
        record.extend(ATTRIBUTE_ID.to_le_bytes());
        record.push(TYPE_ID);
        record.extend(MINIMUM_REPORTING_INTERVAL.to_le_bytes());
        record.extend(MAXIMUM_REPORTING_INTERVAL.to_le_bytes());
        let expected = [record.as_slice(), record.as_slice()].concat();

        assert_eq!(aps.profile(), Profile::ZigbeeHomeAutomation);
        assert_eq!(aps.cluster_id(), Cluster::OnOff.as_u16());
        assert_eq!(zcl.manufacturer_code, None);
        assert_eq!(bytes.as_ref(), expected);
    }
}
