pub trait Receive {
    fn receive_command(&mut self, frame: Frame);
    fn set_transport_state(&mut self, state: State);
    fn node_status_update(
        &mut self,
        node_status: NodeStatus,
        network_address: u32,
        ieee_address: u64,
    );
    fn receive_command_state(&mut self, message_tag: u32, progress_state: ProgressState);
}
