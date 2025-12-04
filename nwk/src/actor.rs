use std::collections::BTreeMap;
use std::error::Error;
use std::time::Duration;

use log::error;
use macaddr::MacAddr8;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;

pub use self::message::Message;
pub use self::zcl_command::ZclCommand;
use crate::Nlme;

mod message;
mod zcl_command;

/// Actor trait for handling NWK layer messages.
pub trait Actor<T> {
    /// Run the actor, processing incoming messages.
    fn run(self, rx: Receiver<Message<T>>) -> impl Future<Output = ()>;
}

impl<T> Actor<T::Error> for T
where
    T: Nlme,
{
    async fn run(mut self, mut rx: Receiver<Message<T::Error>>) {
        while let Some(message) = rx.recv().await {
            match message {
                Message::AllowJoins { duration, sender } => {
                    let result = self.allow_joins(duration).await;
                    sender.send(result).unwrap_or_else(|error| {
                        error!("Failed to send allow joins command response: {error:?}");
                    });
                }
                Message::GetNeighbors { sender } => {
                    let result = self.get_neighbors().await;
                    sender.send(result).unwrap_or_else(|error| {
                        error!("Failed to send get neighbors command response: {error:?}");
                    });
                }
                Message::ZclCommand {
                    pan_id,
                    endpoint,
                    command,
                    sender,
                } => {
                    let result = match command {
                        ZclCommand::On(cmd) => self.unicast_command(pan_id, endpoint, cmd).await,
                        ZclCommand::Off(cmd) => self.unicast_command(pan_id, endpoint, cmd).await,
                        ZclCommand::MoveToColor(cmd) => {
                            self.unicast_command(pan_id, endpoint, cmd).await
                        }
                    };
                    sender.send(result).unwrap_or_else(|error| {
                        error!("Failed to send ZCL command response: {error:?}");
                    });
                }
            }
        }
    }
}

/// Proxy trait for sending NWK layer messages.
pub trait Proxy<T> {
    /// Allow devices to join the network for the specified duration.
    fn allow_joins(
        &mut self,
        duration: Duration,
    ) -> impl Future<Output = Result<(), crate::Error<T>>>;

    /// Get the list of neighbor devices.
    fn get_neighbors(
        &mut self,
    ) -> impl Future<Output = Result<BTreeMap<MacAddr8, u16>, crate::Error<T>>>;

    /// Send a unicast ZCL command.
    fn unicast_command(
        &mut self,
        pan_id: u16,
        endpoint: zigbee::Endpoint,
        command: impl Into<ZclCommand>,
    ) -> impl Future<Output = Result<(), crate::Error<T>>>;
}

impl<T> Proxy<T> for Sender<Message<T>>
where
    T: Error,
{
    async fn allow_joins(&mut self, duration: Duration) -> Result<(), crate::Error<T>> {
        let (sender, rx) = oneshot::channel();
        self.send(Message::AllowJoins { duration, sender })
            .await
            .map_err(|_| crate::Error::ActorSend)?;
        rx.await.map_err(|_| crate::Error::ActorReceive)?
    }

    async fn get_neighbors(&mut self) -> Result<BTreeMap<MacAddr8, u16>, crate::Error<T>> {
        let (sender, rx) = oneshot::channel();
        self.send(Message::GetNeighbors { sender })
            .await
            .map_err(|_| crate::Error::ActorSend)?;
        rx.await.map_err(|_| crate::Error::ActorReceive)?
    }

    async fn unicast_command(
        &mut self,
        pan_id: u16,
        endpoint: zigbee::Endpoint,
        command: impl Into<ZclCommand>,
    ) -> Result<(), crate::Error<T>> {
        let (sender, rx) = oneshot::channel();
        self.send(Message::ZclCommand {
            pan_id,
            endpoint,
            command: command.into(),
            sender,
        })
        .await
        .map_err(|_| crate::Error::ActorSend)?;
        rx.await.map_err(|_| crate::Error::ActorReceive)?
    }
}
