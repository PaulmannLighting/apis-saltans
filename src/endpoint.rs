pub trait Listener {
    fn device_added(&mut self, device: Endpoint);
    fn device_updated(&mut self, device: Endpoint);
    fn device_removed(&mut self, device: Endpoint);
}
