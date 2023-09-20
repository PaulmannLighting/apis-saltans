pub trait FrameHandler {
    fn start(&mut self, port: Port);
    fn set_closing(&mut self);
    fn close(&mut self);
    fn is_alive(&self) -> bool;
    fn enqueue(&mut self, frame_request: FrameRequest);
    fn connect(&mut self);
    fn send(&mut self, transaction: Transaction); // TODO: shall return a future.
}
