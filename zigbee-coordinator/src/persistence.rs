use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot::channel;

pub use self::error::Error;
pub use self::message::Message;
use crate::network_manager::Device;

mod error;
mod message;

/// The persistence server.
pub type Server = Receiver<Message>;

/// The persistence server.
pub type Client = Sender<Message>;

/// The persistence client trait.
pub trait Persistence {
    /// Save devices.
    fn save(&self, devices: Box<[Device]>) -> impl Future<Output = Result<(), Error>> + Send;

    /// Load devices.
    fn load(&self) -> impl Future<Output = Result<Box<[Device]>, Error>> + Send;
}

impl Persistence for Client {
    async fn save(&self, state: Box<[Device]>) -> Result<(), Error> {
        let (tx, rx) = channel();
        self.send(Message::Save {
            state,
            response: tx,
        })
        .await
        .map_err(|_| Error::Send)?;
        rx.await.map_err(|_| Error::Receive)?
    }

    async fn load(&self) -> Result<Box<[Device]>, Error> {
        let (tx, rx) = channel();
        self.send(Message::Load { response: tx })
            .await
            .map_err(|_| Error::Send)?;
        rx.await.map_err(|_| Error::Receive)?
    }
}
