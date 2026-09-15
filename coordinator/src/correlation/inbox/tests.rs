use std::time::Duration;

use tokio::runtime::Builder;
use tokio::sync::mpsc::channel;
use tokio::time::timeout;
use zb_core::Endpoint;

use super::{cancellation, schedule_timeout};
use crate::correlation::{Key, Token};

const CHANNEL_CAPACITY: usize = 1;
const TEST_TIMEOUT: Duration = Duration::from_secs(1);
const DISTANT_DEADLINE: Duration = Duration::from_secs(60);
const GENERATION: u64 = 7;
const SEQUENCE: u8 = 3;
const ADDRESS: u16 = 1;
const CLUSTER: u16 = 2;
const PROFILE: u16 = 3;

#[test]
fn cancellation_survives_a_full_inbox_with_its_original_token() {
    Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("test runtime is available")
        .block_on(async {
            let (sender, mut receiver) = channel(CHANNEL_CAPACITY);
            let token = Token::new(
                Key::new(ADDRESS, Endpoint::Data, CLUSTER, PROFILE, None, SEQUENCE),
                GENERATION,
            );
            sender.try_send(None).expect("inbox starts empty");
            let guard = cancellation(sender.downgrade(), token, Some, "test");

            drop(guard);
            assert_eq!(receiver.recv().await, Some(None));
            assert_eq!(
                timeout(TEST_TIMEOUT, receiver.recv()).await,
                Ok(Some(Some(token)))
            );
        });
}

#[test]
fn pending_timeout_does_not_keep_the_inbox_open() {
    Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("test runtime is available")
        .block_on(async {
            let (sender, mut receiver) = channel(CHANNEL_CAPACITY);
            schedule_timeout(sender.downgrade(), DISTANT_DEADLINE, (), "test timeout");
            drop(sender);

            assert_eq!(timeout(TEST_TIMEOUT, receiver.recv()).await, Ok(None));
        });
}
