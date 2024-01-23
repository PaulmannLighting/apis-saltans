use crate::node::Status;
use crate::IeeeAddress;

pub trait Listener {
    fn device_status_update(
        &mut self,
        _device_status: Status,
        _network_address: u64,
        _ieee_address: IeeeAddress,
    ) {
    }

    fn announce_unknown_device(&mut self, _network_address: u64) {}
}
