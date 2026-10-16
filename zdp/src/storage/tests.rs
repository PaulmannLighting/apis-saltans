use alloc::vec::Vec;
use core::fmt::Debug;

use le_stream::{FromLeStream, ToLeStream};
use zb_aps::data::{DataHeaderParameters, Header};
use zb_aps::{Data, Destination, Payload};
use zb_core::{Endpoint, Profile};

use crate::{
    Command, Frame, MgmtBindRsp, MgmtLqiRsp, MgmtNwkIeeeJoiningListRsp, MgmtRtgRsp, NodeDescReq,
    SecurityChallengeReq,
};

const NETWORK: u16 = 0x1234;
const SEQUENCE: u8 = 42;
// Global Fragmentation Parameters TLV: tag, length-minus-one, node ID, options.
const TLV: &[u8] = &[71, 2, 0x34, 0x12, 0];
const ERROR: &[u8] = &[0xff];
const TABLE: &[u8] = &[0, 3, 1, 2, 0xab, 0xcd];
const JOINING_EMPTY: &[u8] = &[0, 7, 0, 0];
const JOINING_ONE: &[u8] = &[0, 7, 1, 2, 1, 1, 1, 2, 3, 4, 5, 6, 7, 8];

fn roundtrip<T>(bytes: &[u8])
where
    T: FromLeStream + ToLeStream + Debug,
{
    let value = T::from_le_stream(bytes.iter().copied()).expect("valid fixture");
    assert_eq!(value.to_le_stream().collect::<Vec<_>>(), bytes);
}

#[test]
fn management_response_streams_preserve_conditional_fields() {
    for bytes in [ERROR, TABLE, &[0, 0, 0, 0]] {
        roundtrip::<MgmtBindRsp>(bytes);
        roundtrip::<MgmtLqiRsp>(bytes);
        roundtrip::<MgmtRtgRsp>(bytes);
    }
    for bytes in [ERROR, JOINING_EMPTY, JOINING_ONE] {
        roundtrip::<MgmtNwkIeeeJoiningListRsp>(bytes);
    }
    assert!(MgmtLqiRsp::from_le_stream(TABLE[..TABLE.len() - 1].iter().copied()).is_none());
    assert!(
        MgmtNwkIeeeJoiningListRsp::from_le_stream(
            JOINING_ONE[..JOINING_ONE.len() - 1].iter().copied()
        )
        .is_none()
    );
}

#[test]
fn tlv_lists_remain_unprefixed_and_reject_truncation() {
    roundtrip::<SecurityChallengeReq>(TLV);
    roundtrip::<SecurityChallengeReq>(&[]);
    let request: Vec<_> = NETWORK
        .to_le_bytes()
        .into_iter()
        .chain(TLV.iter().copied())
        .collect();
    roundtrip::<NodeDescReq>(&request);
    assert!(SecurityChallengeReq::from_le_stream(TLV[..TLV.len() - 1].iter().copied()).is_none());
}

#[test]
fn command_dispatch_and_conversion_preserve_payload() {
    let typed = SecurityChallengeReq::from_le_stream(TLV.iter().copied()).expect("valid TLV");
    let command: Command = typed.clone().into();
    assert_eq!(command.cluster_id(), SecurityChallengeReq::ID);
    assert_eq!(command.clone().to_le_stream().collect::<Vec<_>>(), TLV);
    assert_eq!(
        SecurityChallengeReq::try_from(command).expect("matching variant"),
        typed
    );
    let wire: Vec<_> = core::iter::once(SEQUENCE)
        .chain(TLV.iter().copied())
        .collect();
    let parsed =
        Frame::<Command>::parse_with_cluster_id(SecurityChallengeReq::ID, wire.iter().copied())
            .expect("known cluster")
            .expect("valid frame");
    assert_eq!(parsed.to_le_stream().collect::<Vec<_>>(), wire);
}

#[test]
fn aps_payload_conversion_works_with_unified_features() {
    let wire: Vec<_> = core::iter::once(SEQUENCE)
        .chain(TLV.iter().copied())
        .collect();
    // This works even when another dependency enables APS alloc independently.
    let payload = Payload::from_le_stream(wire.iter().copied()).expect("payload fits");
    let header = Header::new(DataHeaderParameters {
        destination: Destination::Unicast(Endpoint::Data),
        cluster_id: SecurityChallengeReq::ID,
        profile_id: Profile::Network.as_u16(),
        source_endpoint: Endpoint::Data,
        counter: SEQUENCE,
        extended: None,
    });
    let frame = Frame::<Command>::try_from(Data::new(header, payload)).expect("valid ZDP");
    assert_eq!(frame.to_le_stream().collect::<Vec<_>>(), wire);
}

#[cfg(not(feature = "alloc"))]
#[test]
fn bounded_tlvs_reject_overflow_without_truncating() {
    use super::{TLV_CAPACITY, Tlvs};
    let full: Vec<_> = TLV.repeat(TLV_CAPACITY);
    roundtrip::<SecurityChallengeReq>(&full);
    let excessive = TLV.repeat(TLV_CAPACITY + 1);
    assert!(SecurityChallengeReq::from_le_stream(excessive.iter().copied()).is_none());
    let tlv = zb_core::types::tlv::Tlv::from_le_stream(TLV.iter().copied()).expect("valid TLV");
    assert_eq!(
        Tlvs::try_from_iter(core::iter::repeat_n(tlv, TLV_CAPACITY + 1))
            .expect_err("too many")
            .capacity,
        TLV_CAPACITY
    );
}

#[cfg(feature = "alloc")]
#[test]
fn allocating_public_types_remain_compatible() {
    use alloc::boxed::Box;
    use alloc::vec;

    use zb_core::types::tlv::Tlv;
    let tlv = Tlv::from_le_stream(TLV.iter().copied()).expect("valid TLV");
    let list: Vec<Tlv> = vec![tlv.clone()];
    assert!(NodeDescReq::new(NETWORK, list).is_some());
    let slice: Box<[Tlv]> = vec![tlv].into_boxed_slice();
    let request = SecurityChallengeReq::new(slice);
    let group = crate::Security::SecurityChallengeReq(Box::new(request));
    let _: Box<_> = match group {
        crate::Security::SecurityChallengeReq(value) => value,
        _ => unreachable!(),
    };
}

#[test]
fn full_address_and_cluster_lists_remain_supported() {
    use zb_core::{ByteSizedVec, IeeeAddress};

    use crate::{AppFlags, NwkAddrRsp, NwkAddrRspResponse, SimpleDescriptor};
    let addresses: ByteSizedVec<u16> = (0..u16::from(u8::MAX)).collect();
    let response = NwkAddrRsp::new(Ok(NwkAddrRspResponse::Extended {
        ieee_addr_remote_dev: IeeeAddress::new(0, 0, 0, 0, 0, 0, 0, 0),
        nwk_addr_remote_dev: NETWORK,
        start_index: 0,
        nwk_addr_assoc_dev_list: super::own(addresses.clone()),
    }));
    let bytes: Vec<_> = response.to_le_stream().collect();
    roundtrip::<NwkAddrRsp>(&bytes);
    let descriptor = SimpleDescriptor::new(
        Endpoint::Data,
        Profile::Network,
        NETWORK,
        AppFlags::empty(),
        addresses.clone(),
        addresses.clone(),
    );
    let (_, _, _, _, input, output) = descriptor.into_parts();
    assert_eq!(&*input, &*addresses);
    assert_eq!(&*output, &*addresses);
}
