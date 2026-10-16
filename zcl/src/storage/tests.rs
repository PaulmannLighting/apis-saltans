use alloc::vec::Vec;

use le_stream::{FromLeStream, ToLeStream};

use crate::global::read_attributes::Command as ReadAttributes;
use crate::global::report_attributes::Command as ReportAttributes;
use crate::ota_upgrade::{ImageBlock, ImageBlockResponsePayload, ImageId};
use crate::{Frame, UnsequencedFrame};

const IMAGE: ImageId = ImageId::new(0x1234, 0x5678, 0x90ab_cdef);
const OFFSET: u32 = 42;
// Attribute id, uint16 type, value; no collection length prefix.
const REPORT: &[u8] = &[0, 0, 0x21, 0x34, 0x12];
const ATTRIBUTE_IDS: &[u8] = &[0x34, 0x12, 0x78, 0x56];

#[test]
fn raw_attribute_lists_preserve_encoding_and_reject_truncation() {
    let command = ReadAttributes::from_le_stream(ATTRIBUTE_IDS.iter().copied()).expect("valid IDs");
    assert_eq!(command.attribute_ids(), [0x1234, 0x5678]);
    assert_eq!(command.to_le_stream().collect::<Vec<_>>(), ATTRIBUTE_IDS);
    assert!(ReadAttributes::from_le_stream([0x34].into_iter()).is_none());
    let report = ReportAttributes::from_le_stream(REPORT.iter().copied()).expect("valid report");
    assert_eq!(report.to_le_stream().collect::<Vec<_>>(), REPORT);
}

#[test]
fn maximum_ota_block_preserves_wire_format() {
    let data = super::test_image_data(core::iter::repeat_n(0xab, ImageBlock::MAX_DATA_SIZE));
    let block = ImageBlock::try_new(IMAGE, OFFSET, data).expect("maximum block fits");
    let response = ImageBlockResponsePayload::Success(block);
    let wire: Vec<_> = response.clone().to_le_stream().collect();
    let expected: Vec<_> = core::iter::once(0)
        .chain(IMAGE.to_le_stream())
        .chain(OFFSET.to_le_bytes())
        .chain(core::iter::once(u8::MAX))
        .chain(core::iter::repeat_n(0xab, ImageBlock::MAX_DATA_SIZE))
        .collect();
    assert_eq!(wire, expected);
    assert_eq!(
        ImageBlockResponsePayload::from_le_stream(wire.into_iter()),
        Some(response)
    );
}

#[test]
fn independent_aps_payload_feature_is_supported() {
    use zb_aps::data::{DataHeaderParameters, Header};
    use zb_aps::{Data, Destination, Payload};
    use zb_core::{Endpoint, Profile};
    const WIRE: &[u8] = &[0x01, 42, 0x01]; // On command.
    let payload = Payload::from_le_stream(WIRE.iter().copied()).expect("APS payload fits");
    let aps = Data::new(
        Header::new(DataHeaderParameters {
            destination: Destination::Unicast(Endpoint::Data),
            cluster_id: zb_core::Cluster::OnOff.as_u16(),
            profile_id: Profile::ZigbeeHomeAutomation.as_u16(),
            source_endpoint: Endpoint::Data,
            counter: 0,
            extended: None,
        }),
        payload,
    );
    let parsed = Frame::try_from(aps).expect("valid ZCL");
    assert!(crate::on_off::On::try_from(parsed.into_payload()).is_ok());
}

#[cfg(not(feature = "alloc"))]
mod bounded {
    use zb_core::TypeId;

    use super::super::{
        ATTRIBUTE_IDS_CAPACITY, IMAGE_DATA_CAPACITY, ImageData, PAYLOAD_CAPACITY, RECORDS_CAPACITY,
        REPORTABLE_CHANGE_CAPACITY,
    };
    use super::*;
    use crate::{Analog, Command, Directed, Direction, ReportingConfiguration, Scope, Scoped};

    struct ArbitraryBytes(usize);
    impl TypeId for ArbitraryBytes {
        const ID: u8 = 0x21;
    }
    impl Command for ArbitraryBytes {
        const ID: u8 = 0;
    }
    impl Directed for ArbitraryBytes {
        const DIRECTION: Direction = Direction::ClientToServer;
    }
    impl Scoped for ArbitraryBytes {
        const SCOPE: Scope = Scope::Global;
    }
    impl ToLeStream for ArbitraryBytes {
        type Iter = core::iter::RepeatN<u8>;
        fn to_le_stream(self) -> Self::Iter {
            core::iter::repeat_n(0, self.0)
        }
    }

    #[test]
    fn collections_reject_excess_elements() {
        let full = [0x34, 0x12].repeat(ATTRIBUTE_IDS_CAPACITY);
        assert!(ReadAttributes::from_le_stream(full.into_iter()).is_some());
        let excessive = [0x34, 0x12].repeat(ATTRIBUTE_IDS_CAPACITY + 1);
        assert!(ReadAttributes::from_le_stream(excessive.into_iter()).is_none());
        assert!(
            crate::global::configure_reporting::Response::from_le_stream(
                [0x86, 0, 0, 0].repeat(RECORDS_CAPACITY + 1).into_iter()
            )
            .is_none()
        );
        assert_eq!(
            ImageData::try_from_iter(core::iter::repeat_n(0, IMAGE_DATA_CAPACITY + 1))
                .expect_err("excess image data")
                .capacity,
            IMAGE_DATA_CAPACITY
        );
    }

    #[test]
    fn command_serialization_checks_payload_capacity() {
        let full = UnsequencedFrame::try_from_command(ArbitraryBytes(PAYLOAD_CAPACITY))
            .expect("exact capacity");
        assert_eq!(full.payload().len(), PAYLOAD_CAPACITY);
        assert_eq!(
            UnsequencedFrame::try_from_command(ArbitraryBytes(PAYLOAD_CAPACITY + 1))
                .expect_err("overflow")
                .capacity,
            PAYLOAD_CAPACITY
        );
    }

    #[test]
    fn generic_analog_configuration_reports_overflow() {
        assert!(
            Analog::new(0, 1, ArbitraryBytes(REPORTABLE_CHANGE_CAPACITY))
                .into_configuration(0)
                .is_ok()
        );
        assert_eq!(
            Analog::new(0, 1, ArbitraryBytes(REPORTABLE_CHANGE_CAPACITY + 1))
                .into_configuration(0)
                .expect_err("overflow")
                .capacity,
            REPORTABLE_CHANGE_CAPACITY
        );
    }
}

#[cfg(feature = "alloc")]
#[test]
fn allocating_types_and_infallible_conversions_remain_available() {
    use alloc::boxed::Box;

    use bytes::Bytes;
    use zb_core::types::Uint16;

    use crate::global::configure_reporting::send::AttributeReportingConfiguration;
    use crate::{Analog, basic};
    let _: AttributeReportingConfiguration =
        basic::SendReport::ClusterRevision(Analog::new(0, 1, Uint16::new(1))).into();
    let command = ReadAttributes::new(Box::new([0x1234]));
    let frame: UnsequencedFrame<Bytes> = UnsequencedFrame::from_command(command);
    assert_eq!(frame.payload().as_ref(), [0x34, 0x12]);
}
