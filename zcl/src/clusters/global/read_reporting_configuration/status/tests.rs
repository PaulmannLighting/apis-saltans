use alloc::vec::Vec;

use le_stream::{FromLeStream, ToLeStream};
use zb_core::types::{Type, Uint8, Uint16, Uint64};

use super::{InvalidReportingRecord, ReportedConfiguration, Status};
use crate::global::read_reporting_configuration::Response;

const ATTRIBUTE_ID: u16 = 0x1234;
const MINIMUM: u16 = 10;
const MAXIMUM: u16 = 60;
const UINT16: u8 = 0x21;
const UINT64: u8 = 0x27;
const BOOL: u8 = 0x10;
const BITMAP64: u8 = 0x1f;
const FLOAT32: u8 = 0x39;
const FAILURE: u8 = 0x86;
const REPORTED: u8 = 0;
const RECEIVED: u8 = 1;
const ANALOG: [u8; 11] = [0, 0, 0x34, 0x12, UINT16, 10, 0, 60, 0, 2, 0];
const DISCRETE: [u8; 9] = [0, 0, 0x34, 0x12, BITMAP64, 10, 0, 60, 0];
const TIMEOUT: [u8; 6] = [0, 1, 0x34, 0x12, 0x2c, 1];
const FAILED: [u8; 4] = [FAILURE, RECEIVED, 0x34, 0x12];

fn configuration(datatype: u8, change: Option<Type>) -> ReportedConfiguration {
    ReportedConfiguration {
        attribute_data_type: datatype,
        minimum_reporting_interval: MINIMUM,
        maximum_reporting_interval: MAXIMUM,
        reportable_change: change,
    }
}

#[test]
fn analog_change_uses_the_preceding_datatype_without_repeating_it() {
    let expected = configuration(UINT16, Some(Type::Uint16(Uint16::new(2))));
    let analog = Status::from_le_stream_exact(ANALOG.into_iter()).unwrap();
    assert_eq!(analog.direction(), REPORTED);
    assert_eq!(analog.attribute_id(), ATTRIBUTE_ID);
    assert_eq!(analog.reported_configuration(), Some(expected.clone()));
    assert_eq!(analog.to_le_stream().collect::<Vec<_>>(), ANALOG);
    assert_eq!(
        Status::reported(ATTRIBUTE_ID, expected)
            .to_le_stream()
            .collect::<Vec<_>>(),
        ANALOG
    );
}

#[test]
fn longest_supported_analog_change_fits_the_record_buffer() {
    let configuration = configuration(UINT64, Some(Type::Uint64(Uint64::new(2))));
    let record = Status::try_reported(ATTRIBUTE_ID, configuration).unwrap();
    let wire = [
        0, 0, 0x34, 0x12, UINT64, 10, 0, 60, 0, 2, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(record.clone().to_le_stream().collect::<Vec<_>>(), wire);
    assert_eq!(
        Status::from_le_stream_exact(wire.into_iter()).unwrap(),
        record
    );
}

#[test]
fn discrete_bitmap_needs_only_a_datatype_id() {
    let record = Status::from_le_stream_exact(DISCRETE.into_iter()).unwrap();
    assert_eq!(
        record.reported_configuration(),
        Some(configuration(BITMAP64, None))
    );
    assert_eq!(record.to_le_stream().collect::<Vec<_>>(), DISCRETE);
}

#[test]
fn receiver_and_failure_records_round_trip_without_sender_fields() {
    let received = Status::from_le_stream_exact(TIMEOUT.into_iter()).unwrap();
    assert_eq!(received.direction(), RECEIVED);
    assert_eq!(received.timeout_period(), Some(300));
    assert_eq!(received.reported_configuration(), None);
    assert_eq!(received.to_le_stream().collect::<Vec<_>>(), TIMEOUT);

    let failed = Status::from_le_stream_exact(FAILED.into_iter()).unwrap();
    assert_eq!(failed.status(), FAILURE);
    assert_eq!(failed.timeout_period(), None);
    assert_eq!(failed.reported_configuration(), None);
    assert_eq!(failed, Status::failed(ATTRIBUTE_ID, FAILURE, RECEIVED));
    assert_eq!(failed.to_le_stream().collect::<Vec<_>>(), FAILED);
}

#[test]
fn validates_thresholds_and_status_before_encoding() {
    let wrong_type = Some(Type::Uint8(Uint8::new(2)));
    for change in [None, wrong_type.clone()] {
        assert_eq!(
            Status::try_reported(ATTRIBUTE_ID, configuration(UINT16, change)),
            Err(InvalidReportingRecord::InvalidReportableChange(UINT16))
        );
    }
    assert_eq!(
        Status::try_reported(ATTRIBUTE_ID, configuration(BOOL, wrong_type)),
        Err(InvalidReportingRecord::InvalidReportableChange(BOOL))
    );
    assert_eq!(
        Status::try_reported(ATTRIBUTE_ID, configuration(FLOAT32, None)),
        Err(InvalidReportingRecord::UnsupportedDatatype(FLOAT32))
    );
    assert!(Status::try_failed(ATTRIBUTE_ID, 0, RECEIVED).is_err());
    assert!(Status::try_failed(ATTRIBUTE_ID, FAILURE, 2).is_err());
}

#[test]
fn rejects_truncation_reserved_directions_and_unknown_threshold_layouts() {
    for record in [
        ANALOG.as_slice(),
        DISCRETE.as_slice(),
        TIMEOUT.as_slice(),
        FAILED.as_slice(),
    ] {
        for length in 0..record.len() {
            assert!(Status::from_le_stream(record[..length].iter().copied()).is_none());
        }
    }
    assert!(Status::from_le_stream([0, 2, 0, 0].into_iter()).is_none());
    for datatype in [FLOAT32, u8::MAX] {
        let mut unsupported = DISCRETE;
        unsupported[4] = datatype;
        assert!(Status::from_le_stream(unsupported.into_iter()).is_none());
    }
}

#[test]
fn keeps_mixed_record_boundaries_and_rejects_malformed_tails() {
    assert!(Response::from_le_stream(ANALOG.into_iter().chain([0, 0, 1])).is_none());
    let wire = FAILED
        .into_iter()
        .chain(ANALOG)
        .chain(TIMEOUT)
        .chain(DISCRETE);
    #[cfg(not(feature = "alloc"))]
    if crate::storage::RECORDS_CAPACITY < 4 {
        assert!(Response::from_le_stream(wire).is_none());
        return;
    }
    let response = Response::from_le_stream(wire.clone()).unwrap();
    let [failed, analog, received, discrete] = response.records() else {
        panic!("expected four reporting records");
    };
    assert_eq!(failed.status(), FAILURE);
    assert_eq!(
        analog.reported_configuration().unwrap().attribute_data_type,
        UINT16
    );
    assert_eq!(received.timeout_period(), Some(300));
    assert_eq!(
        discrete.reported_configuration(),
        Some(configuration(BITMAP64, None))
    );
    assert_eq!(
        response.to_le_stream().collect::<Vec<_>>(),
        wire.clone().collect::<Vec<_>>()
    );
    assert!(Response::from_le_stream(wire.chain([0, 0, 1])).is_none());
}

#[cfg(feature = "alloc")]
#[test]
fn allocating_response_rejects_more_than_255_records() {
    let records = core::iter::repeat_n(FAILED, usize::from(u8::MAX)).flatten();
    assert_eq!(
        Response::from_le_stream(records.clone())
            .unwrap()
            .records()
            .len(),
        usize::from(u8::MAX)
    );
    assert!(Response::from_le_stream(records.chain(FAILED)).is_none());
}

#[cfg(feature = "serde")]
#[test]
fn validates_deserialized_fields_before_allowing_wire_encoding() {
    let mut fields = Status::from_le_stream(ANALOG.into_iter()).unwrap().0;
    assert!(fields.validate().is_ok());
    fields.reportable_change = None;
    assert!(fields.validate().is_err());

    let mut fields = Status::received(ATTRIBUTE_ID, 300).0;
    assert!(fields.validate().is_ok());
    fields.minimum_reporting_interval = Some(MINIMUM);
    assert!(fields.validate().is_err());

    let mut fields = Status::failed(ATTRIBUTE_ID, FAILURE, RECEIVED).0;
    assert!(fields.validate().is_ok());
    fields.timeout_period = Some(300);
    assert!(fields.validate().is_err());
}
