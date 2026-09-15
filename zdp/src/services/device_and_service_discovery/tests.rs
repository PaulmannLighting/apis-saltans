use le_stream::{FromLeStream, ToLeStream};
use zb_core::{ByteSizedVec, IeeeAddress};

use super::{IeeeAddrRsp, IeeeAddrRspResponse, NwkAddrRsp, NwkAddrRspResponse};
use crate::Status;

const IEEE_ADDRESS: IeeeAddress = IeeeAddress::new(1, 1, 1, 1, 1, 1, 1, 1);
const NETWORK_ADDRESS: u16 = 0x1234;
const START_INDEX: u8 = 3;
const ASSOCIATED_ADDRESS: u16 = 0x5678;
const SINGLE: &[u8] = &[0, 1, 1, 1, 1, 1, 1, 1, 1, 0x34, 0x12];
const EMPTY_EXTENDED: &[u8] = &[0, 1, 1, 1, 1, 1, 1, 1, 1, 0x34, 0x12, 0];
const EXTENDED: &[u8] = &[0, 1, 1, 1, 1, 1, 1, 1, 1, 0x34, 0x12, 1, 3, 0x78, 0x56];
const MISSING_BYTES: usize = 1;
const UNKNOWN_STATUS: u8 = 0xff;

#[test]
fn address_response_codecs_preserve_wire_examples() {
    for bytes in [SINGLE, EMPTY_EXTENDED, EXTENDED, &[UNKNOWN_STATUS]] {
        let nwk = NwkAddrRsp::from_le_stream(bytes.iter().copied()).expect("valid response");
        let ieee = IeeeAddrRsp::from_le_stream(bytes.iter().copied()).expect("valid response");
        assert_eq!(nwk.clone().to_le_stream().collect::<Vec<_>>(), bytes);
        assert_eq!(ieee.clone().to_le_stream().collect::<Vec<_>>(), bytes);
        if bytes == [UNKNOWN_STATUS] {
            assert_eq!(nwk.status(), Err(UNKNOWN_STATUS));
            assert_eq!(ieee.status(), Err(UNKNOWN_STATUS));
        } else {
            assert_eq!(nwk.status(), Ok(Status::Success));
            assert_eq!(ieee.status(), Ok(Status::Success));
        }
    }
}

#[test]
fn address_response_codecs_reject_incomplete_required_fields() {
    for bytes in [
        &[][..],
        &SINGLE[..SINGLE.len() - MISSING_BYTES],
        &EXTENDED[..EXTENDED.len() - MISSING_BYTES],
    ] {
        assert!(NwkAddrRsp::from_le_stream(bytes.iter().copied()).is_none());
        assert!(IeeeAddrRsp::from_le_stream(bytes.iter().copied()).is_none());
    }
}

#[test]
fn single_and_error_constructors_preserve_wire_format() {
    let nwk = NwkAddrRsp::new(Ok(NwkAddrRspResponse::Single {
        ieee_addr_remote_dev: IEEE_ADDRESS,
        nwk_addr_remote_dev: NETWORK_ADDRESS,
    }));
    let ieee = IeeeAddrRsp::new(Ok(IeeeAddrRspResponse::Single {
        ieee_addr_remote_dev: IEEE_ADDRESS,
        nwk_addr_remote_dev: NETWORK_ADDRESS,
    }));
    assert_eq!(nwk.to_le_stream().collect::<Vec<_>>(), SINGLE);
    assert_eq!(ieee.to_le_stream().collect::<Vec<_>>(), SINGLE);

    let status = Status::InvalidRequestType;
    assert_eq!(
        NwkAddrRsp::new(Err(status))
            .to_le_stream()
            .collect::<Vec<_>>(),
        [u8::from(status)]
    );
    assert_eq!(
        IeeeAddrRsp::new(Err(status))
            .to_le_stream()
            .collect::<Vec<_>>(),
        [u8::from(status)]
    );
}

#[test]
fn extended_constructors_preserve_empty_and_nonempty_lists() {
    for (addresses, expected) in [
        (&[][..], EMPTY_EXTENDED),
        (&[ASSOCIATED_ADDRESS][..], EXTENDED),
    ] {
        let list: ByteSizedVec<u16> = addresses.iter().copied().collect();
        let nwk = NwkAddrRsp::new(Ok(NwkAddrRspResponse::Extended {
            ieee_addr_remote_dev: IEEE_ADDRESS,
            nwk_addr_remote_dev: NETWORK_ADDRESS,
            start_index: START_INDEX,
            nwk_addr_assoc_dev_list: Box::new(list.clone()),
        }));
        let ieee = IeeeAddrRsp::new(Ok(IeeeAddrRspResponse::Extended {
            ieee_addr_remote_dev: IEEE_ADDRESS,
            nwk_addr_remote_dev: NETWORK_ADDRESS,
            start_index: START_INDEX,
            nwk_addr_assoc_dev_list: Box::new(list),
        }));
        assert_eq!(nwk.to_le_stream().collect::<Vec<_>>(), expected);
        assert_eq!(ieee.to_le_stream().collect::<Vec<_>>(), expected);
    }
}
