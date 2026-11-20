//! Independent wire fixtures for management table response codecs.

use alloc::vec::Vec;

use le_stream::{FromLeStream, ToLeStream};
use zb_core::ByteSizedVec;

use crate::{Frame, MgmtBindRsp, MgmtBindRspPayload, MgmtLqiRsp, MgmtRtgRsp, Status};

const GROUP_BINDING: [u8; 14] = [1, 0, 0, 0, 0, 0, 0, 0, 1, 6, 0, 1, 42, 0];
const DIRECT_BINDING: [u8; 21] = [
    1, 0, 0, 0, 0, 0, 0, 0, 1, 6, 0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 1,
];
const NEIGHBOR: [u8; 22] = [
    1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0x34, 0x12, 1, 0, 1, 200,
];
const ROUTE: [u8; 5] = [0x34, 0x12, 0, 0x78, 0x56];
const SUCCESSFUL_EMPTY_PAGE: [u8; 4] = [0, 0, 0, 0];
const UNKNOWN_STATUS: [u8; 1] = [0xff];
const BOUNDED_GROUP_COUNT: u8 = 18;
const SEQUENCE: u8 = 0x42;

#[test]
fn binding_pages_preserve_mixed_entries_and_wire_count() {
    let header = [0, 4, 1, 2];
    let entries: Vec<_> = GROUP_BINDING.into_iter().chain(DIRECT_BINDING).collect();
    let wire: Vec<_> = header.into_iter().chain(entries.iter().copied()).collect();

    let page = MgmtBindRsp::from_le_stream_exact(wire.iter().copied()).unwrap();
    assert_eq!(page.status(), Ok(Status::Success));
    assert_eq!(page.table_entries(), 4);
    assert_eq!(page.start_index(), 1);
    assert_eq!(page.list_count(), 2);
    assert_eq!(page.list_bytes(), entries);
    assert_eq!(page.clone().to_le_stream().collect::<Vec<_>>(), wire);

    let constructed = MgmtBindRsp::new(Ok(MgmtBindRspPayload {
        binding_table_entries: 4,
        start_index: 1,
        binding_table_list: ByteSizedVec::from_slice(&entries).unwrap(),
    }));
    assert_eq!(constructed, page);

    let frame = Frame::parse_with_cluster_id(
        MgmtBindRsp::ID,
        core::iter::once(SEQUENCE).chain(wire.iter().copied()),
    )
    .unwrap()
    .unwrap();
    assert_eq!(frame.seq(), SEQUENCE);
    assert_eq!(MgmtBindRsp::try_from(frame.into_parts().1).unwrap(), page);
}

#[test]
fn neighbor_and_route_counts_describe_records() {
    let neighbor_wire: Vec<_> = [0, 4, 2, 2]
        .into_iter()
        .chain(NEIGHBOR)
        .chain(NEIGHBOR)
        .collect();
    let neighbors = MgmtLqiRsp::from_le_stream_exact(neighbor_wire.iter().copied()).unwrap();
    assert_eq!(neighbors.table_entries(), 4);
    assert_eq!(neighbors.start_index(), 2);
    assert_eq!(neighbors.list_count(), 2);
    assert_eq!(neighbors.list_bytes(), [NEIGHBOR, NEIGHBOR].concat());
    assert_eq!(neighbors.to_le_stream().collect::<Vec<_>>(), neighbor_wire);

    let route_wire: Vec<_> = [0, 4, 2, 1].into_iter().chain(ROUTE).collect();
    let routes = MgmtRtgRsp::from_le_stream_exact(route_wire.iter().copied()).unwrap();
    assert_eq!(routes.table_entries(), 4);
    assert_eq!(routes.start_index(), 2);
    assert_eq!(routes.list_count(), 1);
    assert_eq!(routes.list_bytes(), ROUTE);
    assert_eq!(routes.to_le_stream().collect::<Vec<_>>(), route_wire);
}

#[test]
fn rejects_truncation_trailing_bytes_and_declared_count_mismatches() {
    assert_rejects_malformed::<MgmtBindRsp>(&GROUP_BINDING);
    assert_rejects_malformed::<MgmtLqiRsp>(&NEIGHBOR);
    assert_rejects_malformed::<MgmtRtgRsp>(&ROUTE);
}

fn assert_rejects_malformed<T>(record: &[u8])
where
    T: FromLeStream,
{
    let mut wire: Vec<_> = [0, 3, 0, 1]
        .into_iter()
        .chain(record.iter().copied())
        .collect();
    for length in 0..wire.len() {
        assert!(T::from_le_stream(wire[..length].iter().copied()).is_none());
    }
    wire.push(0);
    assert!(T::from_le_stream(wire.iter().copied()).is_none());

    let wrong_count = [0, 3, 0, 2].into_iter().chain(record.iter().copied());
    assert!(T::from_le_stream(wrong_count).is_none());
}

#[test]
fn rejects_unsupported_binding_address_mode() {
    const ADDRESS_MODE_OFFSET: usize = 11;
    let mut invalid_entry = GROUP_BINDING;
    invalid_entry[ADDRESS_MODE_OFFSET] = 0x02;
    let wire = [0, 1, 0, 1].into_iter().chain(invalid_entry);
    assert!(MgmtBindRsp::from_le_stream(wire).is_none());

    let response = MgmtBindRsp::new(Ok(MgmtBindRspPayload {
        binding_table_entries: 1,
        start_index: 0,
        binding_table_list: ByteSizedVec::from_slice(&invalid_entry).unwrap(),
    }));
    assert_eq!(response.status(), Ok(Status::InvalidRequestType));
    assert_eq!(
        response.to_le_stream().collect::<Vec<_>>(),
        [u8::from(Status::InvalidRequestType)]
    );
}

#[test]
fn preserves_empty_pages_and_unknown_status() {
    let page = MgmtLqiRsp::from_le_stream_exact(SUCCESSFUL_EMPTY_PAGE.into_iter()).unwrap();
    assert_eq!(page.list_count(), 0);
    assert!(page.list_bytes().is_empty());
    assert_eq!(
        page.to_le_stream().collect::<Vec<_>>(),
        SUCCESSFUL_EMPTY_PAGE
    );

    let page = MgmtRtgRsp::from_le_stream_exact(UNKNOWN_STATUS.into_iter()).unwrap();
    assert_eq!(page.status(), Err(u8::MAX));
    assert!(page.list_bytes().is_empty());
    assert_eq!(page.to_le_stream().collect::<Vec<_>>(), UNKNOWN_STATUS);
}

#[test]
fn binding_pages_respect_storage_capacity_and_the_wire_count_limit() {
    let input = |count: u8| {
        [0, count, 0, count].into_iter().chain(
            GROUP_BINDING
                .into_iter()
                .cycle()
                .take(usize::from(count) * GROUP_BINDING.len()),
        )
    };
    assert!(MgmtBindRsp::from_le_stream(input(BOUNDED_GROUP_COUNT)).is_some());

    let beyond_bounded_capacity = MgmtBindRsp::from_le_stream(input(BOUNDED_GROUP_COUNT + 1));
    #[cfg(not(feature = "alloc"))]
    assert!(beyond_bounded_capacity.is_none());
    #[cfg(feature = "alloc")]
    {
        assert_eq!(
            beyond_bounded_capacity.unwrap().list_count(),
            BOUNDED_GROUP_COUNT + 1
        );
        let full = MgmtBindRsp::from_le_stream(input(u8::MAX)).unwrap();
        assert_eq!(full.list_count(), u8::MAX);
        assert_eq!(
            full.list_bytes().len(),
            usize::from(u8::MAX) * GROUP_BINDING.len()
        );
        assert!(MgmtBindRsp::from_le_stream(input(u8::MAX).chain(GROUP_BINDING)).is_none());
    }
}
