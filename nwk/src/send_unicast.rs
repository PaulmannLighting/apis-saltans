use le_stream::ToLeStream;
use zigbee::frame::Frame;

/// Trait for sending unicast messages.
pub trait SendUnicast {
    /// The error type.
    type Error;

    /// Send a raw unicast message to the specified address.
    fn send_raw_unicast<T>(
        &mut self,
        address: u16,
        payload: T,
    ) -> impl Future<Output = Result<(), Self::Error>>
    where
        T: IntoIterator<Item = u8>;

    /// Send a unicast message to the specified address.
    fn send_unicast<T>(
        &mut self,
        address: u16,
        frame: Frame<T>,
    ) -> impl Future<Output = Result<(), Self::Error>>
    where
        T: ToLeStream,
    {
        self.send_raw_unicast(address, frame.to_le_stream())
    }
}
