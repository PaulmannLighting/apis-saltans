pub trait Listener {
    fn command_received(&mut self, command: Command);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Command {
    source_address: u64,
    destination_address: u64,
    cluster_id: u8,
    transaction_id: u8,
    aps_security: bool,
    ack_request: bool,
}
