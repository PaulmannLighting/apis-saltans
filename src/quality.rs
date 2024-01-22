pub trait Statistics {
    fn last_received_lqi(&self) -> u8;
    fn last_received_rssi(&self) -> i8;
}
