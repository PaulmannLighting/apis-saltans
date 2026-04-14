use macaddr::MacAddr8;

pub use self::command::Command;

mod command;

/// Events that can occur in the network module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    /// The network is up and running.
    NetworkUp,
    /// The network is down.
    NetworkDown,
    /// The network has been opened for new joins.
    NetworkOpened,
    /// The network has been closed for new joins.
    NetworkClosed,
    /// A new device has joined the network.
    DeviceJoined {
        /// The IEEE address of the joined device.
        ieee_address: MacAddr8,
        /// The PAN ID of the joined device.
        pan_id: u16,
    },
    /// A device has rejoined the network.
    DeviceRejoined {
        /// The IEEE address of the joined device.
        ieee_address: MacAddr8,
        /// The PAN ID of the joined device.
        pan_id: u16,
        /// Whether the rejoin was secured.
        secured: bool,
    },
    /// A device has left the network.
    DeviceLeft {
        /// The IEEE address of the left device.
        ieee_address: MacAddr8,
        /// The PAN ID of the left device.
        pan_id: u16,
    },
    /// A new device has been discovered.
    DeviceDiscovered {
        /// The IEEE address of the discovered device.
        ieee_address: MacAddr8,
        /// The PAN ID of the discovered device.
        pan_id: u16,
    },
    /// Message received from a device.
    MessageReceived {
        /// The PAN ID of the sender.
        src_address: u16,
        /// The APS frame.
        aps_frame: aps::Data<Command>,
    },
}

#[cfg(feature = "smarthomelib")]
mod zigbee {
    use aps::data::Header;
    use log::{info, warn};
    use smarthomelib::Command;
    use zcl::Cluster;
    use zcl::general::{level, on_off};
    use zigbee::Endpoint;

    use super::Event;
    use crate::smarthomelib::Source;

    impl From<Event> for smarthomelib::Event<Source> {
        fn from(event: Event) -> Self {
            match event {
                Event::MessageReceived {
                    src_address,
                    aps_frame,
                } => {
                    let (header, payload) = aps_frame.into_parts();

                    match payload {
                        crate::Command::Zdp(command) => {
                            warn!("ZDP command handling not yet implemented: {command:?}");
                            Self::Unhandled
                        }
                        crate::Command::Zcl(command) => {
                            translate_zcl_command(src_address, header, command)
                        }
                    }
                }
                other => {
                    warn!("Unhandled event: {other:?}");
                    Self::Unhandled
                }
            }
        }
    }

    fn translate_zcl_command(
        pan_id: u16,
        header: Header,
        command: zcl::Frame<Cluster>,
    ) -> smarthomelib::Event<Source> {
        let endpoint: Endpoint = header.source_endpoint().into();
        info!("Received ZCL command from {pan_id:#06X}/{endpoint}: {command:?}");
        let (_header, payload) = command.into_parts();

        match payload {
            Cluster::OnOff(on_off) => match on_off {
                on_off::Command::On(_) => smarthomelib::Event::Command {
                    sender: Source::new(pan_id, endpoint),
                    command: Command::On,
                },
                on_off::Command::Off(_) => smarthomelib::Event::Command {
                    sender: Source::new(pan_id, endpoint),
                    command: Command::Off,
                },
                other => {
                    warn!("Received unhandled On/Off command: {other:?}");
                    smarthomelib::Event::Unhandled
                }
            },
            Cluster::Level(level) => match level {
                level::Command::Move(r#move) => match r#move.try_into() {
                    Ok(dimming) => smarthomelib::Event::Command {
                        sender: Source::new(pan_id, endpoint),
                        command: Command::Dimming(Some(dimming)),
                    },
                    Err(err) => {
                        warn!("Failed to translate Move command: {err}");
                        smarthomelib::Event::Unhandled
                    }
                },
                level::Command::MoveWithOnOff(move_with_on_off) => {
                    match move_with_on_off.try_into() {
                        Ok(dimming) => smarthomelib::Event::Command {
                            sender: Source::new(pan_id, endpoint),
                            command: Command::Dimming(Some(dimming)),
                        },
                        Err(err) => {
                            warn!("Failed to translate MoveWithOnOff command: {err}");
                            smarthomelib::Event::Unhandled
                        }
                    }
                }
                level::Command::Stop(_) | level::Command::StopWithOnOff(_) => {
                    smarthomelib::Event::Command {
                        sender: Source::new(pan_id, endpoint),
                        command: Command::Dimming(None),
                    }
                }
                other => {
                    warn!("Received unhandled Level command: {other:?}");
                    smarthomelib::Event::Unhandled
                }
            },
            other => {
                warn!("Received unhandled ZCL command: {other:?}");
                smarthomelib::Event::Unhandled
            }
        }
    }
}
