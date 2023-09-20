use super::JoinMethod;

#[derive(Debug)]
pub struct NetworkParameters {
    extended_pan_id: u8,
    pan_id: u16,
    radio_tx_power: u8,
    radio_channel: u8,
    join_method: JoinMethod,
    nwk_manager_id: u16,
    nwk_update_id: u8,
    channels: u32,
}
