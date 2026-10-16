use alloc::vec::Vec;
use core::iter::once;

use le_stream::{FromLeStream, ToLeStream};

use super::{
    ImageId, UpgradeComplete, UpgradeEndRequest, UpgradeIncomplete, UpgradeIncompleteReason,
};

const IMAGE: ImageId = ImageId::new(0x1234, 0x5678, 0x90ab_cdef);
const SUCCESS: [u8; 9] = [0x00, 0x34, 0x12, 0x78, 0x56, 0xef, 0xcd, 0xab, 0x90];
const INCOMPLETE: [(u8, UpgradeIncompleteReason); 3] = [
    (0x95, UpgradeIncompleteReason::Abort),
    (0x96, UpgradeIncompleteReason::InvalidImage),
    (0x99, UpgradeIncompleteReason::RequireMoreImage),
];
const UNKNOWN_STATUS: u8 = 0xff;

#[test]
fn success_selects_complete_and_preserves_wire_bytes() {
    let complete = UpgradeComplete::new(IMAGE);
    assert_eq!(
        UpgradeEndRequest::from_le_stream(SUCCESS.into_iter()),
        Some(UpgradeEndRequest::Complete(complete))
    );
    assert_eq!(complete.to_le_stream().collect::<Vec<_>>(), SUCCESS);
    assert_eq!(
        UpgradeComplete::from_le_stream(SUCCESS.into_iter()),
        Some(complete)
    );
    assert!(UpgradeIncomplete::from_le_stream(SUCCESS.into_iter()).is_none());
    let request = UpgradeEndRequest::Complete(complete);
    assert_eq!(UpgradeIncomplete::try_from(request), Err(request));
}

#[test]
fn incomplete_statuses_select_reason_and_preserve_wire_bytes() {
    for (status, reason) in INCOMPLETE {
        let mut bytes = SUCCESS;
        bytes[0] = status;
        let incomplete = UpgradeIncomplete::new(reason, IMAGE);
        assert_eq!(
            UpgradeEndRequest::from_le_stream(bytes.into_iter()),
            Some(UpgradeEndRequest::Incomplete(incomplete))
        );
        assert_eq!(incomplete.to_le_stream().collect::<Vec<_>>(), bytes);
        assert_eq!(
            UpgradeIncomplete::from_le_stream(bytes.into_iter()),
            Some(incomplete)
        );
        assert!(UpgradeComplete::from_le_stream(bytes.into_iter()).is_none());
        let request = UpgradeEndRequest::Incomplete(incomplete);
        assert_eq!(UpgradeComplete::try_from(request), Err(request));
    }
}

#[test]
fn unknown_status_and_truncated_payloads_are_rejected() {
    let mut bytes = SUCCESS;
    bytes[0] = UNKNOWN_STATUS;
    assert!(UpgradeEndRequest::from_le_stream(bytes.into_iter()).is_none());
    for status in once(SUCCESS[0]).chain(INCOMPLETE.map(|(status, _)| status)) {
        bytes[0] = status;
        for length in 0..bytes.len() {
            assert!(UpgradeEndRequest::from_le_stream(bytes[..length].iter().copied()).is_none());
        }
    }
}
