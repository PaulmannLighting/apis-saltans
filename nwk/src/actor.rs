use std::collections::BTreeMap;
use std::time::Duration;

use log::error;
use macaddr::MacAddr8;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;
use zcl::general::on_off;
use zcl::lighting::color_control;
use zcl::{Commands, general, lighting};

pub use self::message::Message;
use crate::device_proxy::DeviceProxy;
use crate::{Error, Nlme};

mod message;

/// Actor trait for handling NWK layer messages.
pub trait Actor<T> {
    /// Run the actor, processing incoming messages.
    fn run(self, rx: Receiver<Message>) -> impl Future<Output = ()>;
}

impl<T> Actor<T::Error> for T
where
    T: Nlme,
{
    async fn run(mut self, mut rx: Receiver<Message>) {
        while let Some(message) = rx.recv().await {
            match message {
                Message::AllowJoins { duration, response } => {
                    response
                        .send(self.allow_joins(duration).await)
                        .unwrap_or_else(|error| {
                            error!("Failed to send allow joins command response: {error:?}");
                        });
                }
                Message::GetNeighbors { response } => {
                    response
                        .send(self.get_neighbors().await)
                        .unwrap_or_else(|error| {
                            error!("Failed to send get neighbors command response: {error:?}");
                        });
                }
                Message::ZclCommand {
                    pan_id,
                    endpoint,
                    command,
                    response,
                } => {
                    response
                        .send(match command {
                            Commands::Lighting(lighting::Command::ColorControl(command)) => {
                                match command {
                                    color_control::Command::MoveToColor(move_to_color) => {
                                        self.unicast_command(pan_id, endpoint, move_to_color).await
                                    }
                                    _ => Err(Error::NotImplemented),
                                }
                            }
                            Commands::General(general::Command::OnOff(command)) => match command {
                                on_off::Command::On(on) => {
                                    self.unicast_command(pan_id, endpoint, on).await
                                }
                                on_off::Command::Off(off) => {
                                    self.unicast_command(pan_id, endpoint, off).await
                                }
                                _ => Err(Error::NotImplemented),
                            },
                            _ => Err(Error::NotImplemented),
                        })
                        .unwrap_or_else(|error| {
                            error!("Failed to send ZCL command response: {error:?}");
                        });
                }
            }
        }
    }
}

/// Proxy trait for sending NWK layer messages.
pub trait Proxy {
    /// Allow devices to join the network for the specified duration.
    fn allow_joins(&mut self, duration: Duration) -> impl Future<Output = Result<(), Error>>;

    /// Get the list of neighbor devices.
    fn get_neighbors(&mut self) -> impl Future<Output = Result<BTreeMap<MacAddr8, u16>, Error>>;

    /// Send a unicast ZCL command.
    fn unicast_command(
        &mut self,
        pan_id: u16,
        endpoint: zigbee::Endpoint,
        command: impl Into<Commands>,
    ) -> impl Future<Output = Result<(), Error>>;

    /// Get a device proxy for the specified PAN ID.
    fn device(&mut self, pan_id: u16) -> DeviceProxy<'_, Self>
    where
        Self: Sized,
    {
        DeviceProxy::new(self, pan_id)
    }
}

impl Proxy for Sender<Message> {
    async fn allow_joins(&mut self, duration: Duration) -> Result<(), Error> {
        let (sender, rx) = oneshot::channel();
        self.send(Message::AllowJoins {
            duration,
            response: sender,
        })
        .await
        .map_err(|_| Error::ActorSend)?;
        rx.await.map_err(|_| Error::ActorReceive)?
    }

    async fn get_neighbors(&mut self) -> Result<BTreeMap<MacAddr8, u16>, Error> {
        let (sender, rx) = oneshot::channel();
        self.send(Message::GetNeighbors { response: sender })
            .await
            .map_err(|_| Error::ActorSend)?;
        rx.await.map_err(|_| Error::ActorReceive)?
    }

    async fn unicast_command(
        &mut self,
        pan_id: u16,
        endpoint: zigbee::Endpoint,
        command: impl Into<Commands>,
    ) -> Result<(), Error> {
        let (sender, rx) = oneshot::channel();
        self.send(Message::ZclCommand {
            pan_id,
            endpoint,
            command: command.into(),
            response: sender,
        })
        .await
        .map_err(|_| Error::ActorSend)?;
        rx.await.map_err(|_| Error::ActorReceive)?
    }
}
