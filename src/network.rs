use crate::node::Node;
use crate::IeeeAddress;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::AtomicU8;

mod node;
mod state;

const DEFAULT_LOCAL_ENDPOINT_ID: u8 = 1;
const BROADCAST_ENDPOINT_ID: u8 = u8::MAX;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Manager {
    network_nodes: HashMap<IeeeAddress, Node>,
    group_manager: GroupManager,
    node_listeners: Vec<NodeListener>,
    announce_listeners: Vec<AnnounceListerner>,
    aps_counter: AtomicU8,
    database_manager: DatabaseManager,
    notification_service: NotificationService,
    executor_service: ExecutorService,
    transport: TransportTransmit,
    transaction_manager: TransactionManager,
    local_endpoint_id: u8,
    aps_data_entity: ApsDataEntity,
    command_notifier: CommandNotifier,
    state_listeners: Vec<StateListener>,
    node_discovery_complete: HashSet<StateListener>,
    // TODO: implement further attributes
}
