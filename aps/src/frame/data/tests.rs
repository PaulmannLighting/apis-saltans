use alloc::vec::Vec;
use core::num::NonZero;

use le_stream::{FromLeStream, ToLeStream};
use zb_core::Endpoint;

use super::{Assembler, AssemblyError, DataHeaderParameters, Frame, Header};
use crate::apsde::{
    DataIndication, IndicationMetadata, IndicationStatus, IndividualEndpoint, NetworkAddress,
    ReceivedDestination, Security, Source,
};
#[cfg(not(feature = "alloc"))]
use crate::{
    CapacityKind, FRAGMENT_CAPACITY, FRAGMENTS_CAPACITY, PAYLOAD_CAPACITY, TRANSACTIONS_CAPACITY,
};
use crate::{Destination, Extended, Payload};

const COUNTER: u8 = 42;
const CLUSTER: u16 = 6;
const PROFILE: u16 = 0x0104;
const SOURCE: u16 = 1;
const FIRST: &[u8] = &[1, 2];
const SECOND: &[u8] = &[3, 4];
const LAST: &[u8] = &[5, 6];
const CHUNK_SIZE: usize = 2;
const BLOCKS: u8 = 3;

fn payload(bytes: &[u8]) -> Payload {
    #[cfg(feature = "alloc")]
    {
        Payload::copy_from_slice(bytes)
    }
    #[cfg(not(feature = "alloc"))]
    {
        Payload::try_from(bytes).expect("test payload fits")
    }
}

fn frame(extended: Option<Extended>, bytes: &[u8]) -> Frame<Payload> {
    Frame::raw(
        Header::new(DataHeaderParameters {
            destination: Destination::Unicast(Endpoint::Data),
            cluster_id: CLUSTER,
            profile_id: PROFILE,
            source_endpoint: Endpoint::Data,
            counter: COUNTER,
            extended,
        }),
        payload(bytes),
    )
}

fn received(frame: Frame<Payload>, source: u16) -> DataIndication<Frame<Payload>, ()> {
    let endpoint = IndividualEndpoint::new(Endpoint::Data).expect("individual endpoint");
    DataIndication::new(
        IndicationMetadata {
            destination: ReceivedDestination::Network {
                address: NetworkAddress::new(0).expect("coordinator"),
                endpoint,
            },
            source: Source::Network {
                address: NetworkAddress::new(source).expect("source address"),
                endpoint,
            },
            profile_id: PROFILE,
            cluster_id: CLUSTER,
            status: IndicationStatus::success(),
            security: Security::Unsecured,
            link_quality: u8::MAX,
            rx_time: (),
        },
        frame,
    )
}

#[test]
fn payload_and_frame_wire_encodings_have_no_added_length_prefix() {
    let original = frame(None, FIRST);
    let header_bytes: Vec<_> = original.header().to_le_stream().collect();
    let bytes: Vec<_> = original.clone().to_le_stream().collect();
    assert_eq!(&bytes[..header_bytes.len()], header_bytes);
    assert_eq!(&bytes[header_bytes.len()..], FIRST);
    assert_eq!(
        Frame::<Payload>::from_le_stream(bytes.into_iter()),
        Some(original)
    );
}

#[test]
fn fragmented_payload_round_trips_and_preserves_header() {
    let original = frame(None, &[1, 2, 3, 4, 5, 6]);
    let mut assembler = Assembler::default();
    let fragments = original
        .clone()
        .fragment(NonZero::new(CHUNK_SIZE).expect("chunk size"))
        .expect("fragment count");
    let mut result = None;
    for fragment in fragments {
        result = assembler
            .try_add(received(fragment, SOURCE))
            .expect("valid fragment");
    }
    assert_eq!(result, Some(original));
}

#[test]
fn out_of_order_fragments_and_duplicates_preserve_block_order() {
    let mut assembler = Assembler::default();
    assert!(
        assembler
            .try_add(received(
                frame(Some(Extended::first_fragment(BLOCKS)), FIRST),
                SOURCE
            ))
            .expect("first")
            .is_none()
    );
    for _ in 0..2 {
        assert!(
            assembler
                .try_add(received(
                    frame(Some(Extended::followup_fragment(2)), LAST),
                    SOURCE
                ))
                .expect("last")
                .is_none()
        );
    }
    let result = assembler
        .try_add(received(
            frame(Some(Extended::followup_fragment(1)), SECOND),
            SOURCE,
        ))
        .expect("middle")
        .expect("complete");
    assert_eq!(result.payload().as_ref(), &[1, 2, 3, 4, 5, 6]);
    assert_eq!(result.header().extended(), None);
}

#[test]
fn invalid_followup_discards_only_its_matching_transaction() {
    let mut assembler = Assembler::default();
    let first = frame(Some(Extended::first_fragment(2)), FIRST);
    assert!(assembler.add(received(first, SOURCE)).is_none());
    assert_eq!(
        assembler.try_add(received(
            frame(Some(Extended::followup_fragment(2)), LAST),
            SOURCE
        )),
        Err(AssemblyError::InvalidFragment)
    );
    assert_eq!(
        assembler.try_add(received(
            frame(Some(Extended::followup_fragment(1)), LAST),
            SOURCE
        )),
        Err(AssemblyError::UnexpectedFragment)
    );
    assert_eq!(
        assembler.try_add(received(
            frame(Some(Extended::first_fragment(0)), FIRST),
            SOURCE
        )),
        Err(AssemblyError::InvalidFragment)
    );
}

#[cfg(not(feature = "alloc"))]
#[test]
fn payload_capacity_is_checked_in_construction_and_decoding() {
    let exact = [0; PAYLOAD_CAPACITY];
    assert_eq!(
        Payload::try_from(exact.as_slice()).expect("fits").len(),
        PAYLOAD_CAPACITY
    );
    let overflow = [0; PAYLOAD_CAPACITY + 1];
    let error = Payload::try_from(overflow.as_slice()).expect_err("must reject overflow");
    assert_eq!(error.kind, CapacityKind::Payload);
    assert_eq!(error.required, overflow.len());
    assert!(Payload::from_le_stream(overflow.into_iter()).is_none());
}

#[cfg(not(feature = "alloc"))]
#[test]
fn full_table_rejects_new_keys_but_allows_replacement_and_completion() {
    let mut assembler = Assembler::default();
    for source in 0..TRANSACTIONS_CAPACITY {
        assert!(
            assembler
                .try_add(received(
                    frame(Some(Extended::first_fragment(2)), FIRST),
                    u16::try_from(source).expect("source")
                ))
                .expect("slot available")
                .is_none()
        );
    }
    let rejected = assembler.try_add(received(
        frame(Some(Extended::first_fragment(2)), FIRST),
        u16::try_from(TRANSACTIONS_CAPACITY).expect("source"),
    ));
    assert!(
        matches!(rejected, Err(AssemblyError::Capacity(error)) if error.kind == CapacityKind::Transactions)
    );
    assert!(
        assembler
            .try_add(received(
                frame(Some(Extended::first_fragment(2)), SECOND),
                0
            ))
            .expect("replacement at capacity")
            .is_none()
    );
    let completed = assembler
        .try_add(received(
            frame(Some(Extended::followup_fragment(1)), LAST),
            0,
        ))
        .expect("complete")
        .expect("frame");
    assert_eq!(completed.payload().as_ref(), &[3, 4, 5, 6]);
    assert!(
        assembler
            .try_add(received(
                frame(Some(Extended::first_fragment(2)), FIRST),
                u16::try_from(TRANSACTIONS_CAPACITY).expect("source")
            ))
            .expect("freed slot")
            .is_none()
    );
}

#[cfg(not(feature = "alloc"))]
#[test]
fn oversized_fragment_or_count_does_not_replace_an_existing_transaction() {
    let mut assembler = Assembler::default();
    assembler
        .try_add(received(
            frame(Some(Extended::first_fragment(2)), FIRST),
            SOURCE,
        ))
        .expect("first");
    if FRAGMENT_CAPACITY < PAYLOAD_CAPACITY {
        let oversized = [0; FRAGMENT_CAPACITY + 1];
        let error = assembler.try_add(received(
            frame(Some(Extended::first_fragment(2)), &oversized),
            SOURCE,
        ));
        assert!(
            matches!(error, Err(AssemblyError::Capacity(error)) if error.kind == CapacityKind::Fragment)
        );
    }
    if let Ok(blocks) = u8::try_from(FRAGMENTS_CAPACITY + 1) {
        let error = assembler.try_add(received(
            frame(Some(Extended::first_fragment(blocks)), FIRST),
            SOURCE,
        ));
        assert!(
            matches!(error, Err(AssemblyError::Capacity(error)) if error.kind == CapacityKind::Fragments)
        );
    }
    let result = assembler
        .try_add(received(
            frame(Some(Extended::followup_fragment(1)), SECOND),
            SOURCE,
        ))
        .expect("original preserved")
        .expect("complete");
    assert_eq!(result.payload().as_ref(), &[1, 2, 3, 4]);
}

#[cfg(not(feature = "alloc"))]
#[test]
fn reassembly_checks_total_bytes_and_duplicate_replacement_size() {
    // Configurations whose fragment slots cannot exceed the payload limit need no overflow path.
    let blocks = PAYLOAD_CAPACITY / FRAGMENT_CAPACITY + 1;
    if blocks > FRAGMENTS_CAPACITY {
        return;
    }
    let blocks = u8::try_from(blocks).expect("configured block count");
    let bytes = [0; FRAGMENT_CAPACITY];
    let mut assembler = Assembler::default();
    assembler
        .try_add(received(
            frame(Some(Extended::first_fragment(blocks)), &bytes),
            SOURCE,
        ))
        .expect("first");
    for index in 1..blocks - 1 {
        assembler
            .try_add(received(
                frame(Some(Extended::followup_fragment(index)), &bytes),
                SOURCE,
            ))
            .expect("within limit");
        assembler
            .try_add(received(
                frame(Some(Extended::followup_fragment(index)), &bytes),
                SOURCE,
            ))
            .expect("duplicate is not counted twice");
    }
    let error = assembler.try_add(received(
        frame(Some(Extended::followup_fragment(blocks - 1)), &bytes),
        SOURCE,
    ));
    assert!(
        matches!(error, Err(AssemblyError::Capacity(error)) if error.kind == CapacityKind::Payload)
    );
    assert_eq!(
        assembler.try_add(received(
            frame(Some(Extended::followup_fragment(1)), FIRST),
            SOURCE
        )),
        Err(AssemblyError::UnexpectedFragment)
    );
}

#[cfg(not(feature = "alloc"))]
#[test]
fn fragmentation_rejects_excessive_chunks_and_block_counts() {
    use super::FragmentationError;

    if PAYLOAD_CAPACITY > FRAGMENTS_CAPACITY {
        let bytes = alloc::vec![0; FRAGMENTS_CAPACITY + 1];
        let result = frame(None, &bytes).fragment(NonZero::new(1).expect("nonzero"));
        if FRAGMENTS_CAPACITY < usize::from(u8::MAX) {
            assert!(
                matches!(result, Err(FragmentationError::Capacity(error)) if error.kind == CapacityKind::Fragments)
            );
        } else {
            assert!(matches!(result, Err(FragmentationError::BlockCount(_))));
        }
    }
    if PAYLOAD_CAPACITY > FRAGMENT_CAPACITY + 1 {
        let bytes = alloc::vec![0; FRAGMENT_CAPACITY + 2];
        let result =
            frame(None, &bytes).fragment(NonZero::new(FRAGMENT_CAPACITY + 1).expect("nonzero"));
        assert!(
            matches!(result, Err(FragmentationError::Capacity(error)) if error.kind == CapacityKind::Fragment)
        );
    }
}

#[cfg(not(feature = "alloc"))]
#[test]
fn maximum_fragment_slots_are_accepted() {
    let mut assembler = Assembler::default();
    let blocks = u8::try_from(FRAGMENTS_CAPACITY).expect("configured count");
    let first = assembler
        .try_add(received(
            frame(Some(Extended::first_fragment(blocks)), &[]),
            SOURCE,
        ))
        .expect("maximum slots fit");
    if blocks == 1 {
        assert!(first.is_some());
    } else {
        assert!(first.is_none());
        for index in 1..blocks {
            let result = assembler
                .try_add(received(
                    frame(Some(Extended::followup_fragment(index)), &[]),
                    SOURCE,
                ))
                .expect("valid slot");
            assert_eq!(result.is_some(), index == blocks - 1);
        }
    }
}
