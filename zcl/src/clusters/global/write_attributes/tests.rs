use alloc::vec::Vec;

use le_stream::ToLeStream;
use zb_core::Cluster;

use super::{Response, Status};
use crate::{Frame, ParseFrameError};

const CLUSTER_ID: u16 = Cluster::IasZone.as_u16();
const HEADER: [u8; 3] = [0x18, 0x42, 0x04];
const CIE_ADDRESS_ID: u16 = 0x0010;
const OTHER_ID: u16 = 0x1234;
const FAILURE: [u8; 3] = [0x86, 0x10, 0x00];
const OTHER_FAILURE: [u8; 3] = [0xff, 0x34, 0x12];

#[test]
fn round_trips_request_wide_success() {
    let wire = [0x18, 0x42, 0x04, 0x00];
    let frame = Frame::parse(CLUSTER_ID, wire.into_iter()).unwrap();
    let (header, payload) = frame.into_parts();
    let response = Response::try_from(payload).unwrap();

    assert_eq!(response, Response::success());
    assert!(response.is_success());
    assert_eq!(response.failures(), []);
    assert_eq!(
        Frame::new(header, response)
            .to_le_stream()
            .collect::<Vec<_>>(),
        wire
    );
}

#[test]
fn round_trips_failure_records() {
    let wire = HEADER.into_iter().chain(FAILURE);
    let frame = Frame::parse(CLUSTER_ID, wire.clone()).unwrap();
    let (header, payload) = frame.into_parts();
    let response = Response::try_from(payload).unwrap();

    assert!(!response.is_success());
    assert_eq!(
        response.failures(),
        [Status::new(
            crate::Status::UnsupportedAttribute,
            CIE_ADDRESS_ID
        )]
    );
    assert!(Frame::new(header, response).to_le_stream().eq(wire.clone()));

    let wire = wire.chain(OTHER_FAILURE);
    #[cfg(not(feature = "alloc"))]
    if crate::storage::RECORDS_CAPACITY == 1 {
        assert_eq!(
            Frame::parse(CLUSTER_ID, wire),
            Err(ParseFrameError::InsufficientPayload)
        );
        return;
    }

    let frame = Frame::parse(CLUSTER_ID, wire.clone()).unwrap();
    let (header, payload) = frame.into_parts();
    let response = Response::try_from(payload).unwrap();
    let [first, second] = response.failures() else {
        panic!("expected two failure records");
    };
    assert_eq!(
        *first,
        Status::new(crate::Status::UnsupportedAttribute, CIE_ADDRESS_ID)
    );
    assert_eq!(second.id(), OTHER_ID);
    assert_eq!(second.status(), Err(OTHER_FAILURE[0]));
    assert!(Frame::new(header, response).to_le_stream().eq(wire));
}

#[test]
fn rejects_empty_truncated_and_mixed_payloads() {
    const INVALID: &[&[u8]] = &[
        &[],
        &[0x86],
        &[0x86, 0x10],
        &[0x86, 0x10, 0x00, 0x86],
        &[0x86, 0x10, 0x00, 0x86, 0x34],
        &[0x00, 0x10, 0x00],
        &[0x86, 0x10, 0x00, 0x00, 0x34, 0x12],
    ];
    for payload in INVALID {
        assert_eq!(
            Frame::parse(
                CLUSTER_ID,
                HEADER.into_iter().chain(payload.iter().copied())
            ),
            Err(ParseFrameError::InsufficientPayload),
        );
    }
}

#[test]
fn constructor_omits_success_records() {
    let success = Status::new(crate::Status::Success, CIE_ADDRESS_ID);
    assert_eq!(
        Response::new(crate::storage::test_records([success])),
        Response::success()
    );

    #[cfg(not(feature = "alloc"))]
    if crate::storage::RECORDS_CAPACITY == 1 {
        return;
    }

    let failure = Status::new(crate::Status::UnsupportedAttribute, OTHER_ID);
    let response = Response::new(crate::storage::test_records([success, failure]));
    assert_eq!(response.failures(), [failure]);
}
