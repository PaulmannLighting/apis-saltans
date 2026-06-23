use log::warn;
use macaddr::MacAddr8;
use smarthomelib::{Event, InboundProtocolCommand, LevelMoveDirection};
use zcl::Cluster;
use zcl::general::level::Mode;
use zcl::general::{level, on_off};
use zigbee::Endpoint;

impl TryFrom<crate::Event> for Event<MacAddr8, Endpoint> {
    type Error = crate::Event;

    fn try_from(event: crate::Event) -> Result<Self, Self::Error> {
        let (address, endpoint, cluster) = event.into_parts();

        match cluster {
            Cluster::OnOff(on_off) => match on_off {
                on_off::Command::On(_)
                | on_off::Command::OnWithTimedOff(_)
                | on_off::Command::OnWithRecallGlobalScene(_) => Ok(Self::new(
                    address.ieee_address(),
                    endpoint,
                    InboundProtocolCommand::On,
                )),
                on_off::Command::Off(_) | on_off::Command::OffWithEffect(_) => Ok(Self::new(
                    address.ieee_address(),
                    endpoint,
                    InboundProtocolCommand::Off,
                )),
                on_off::Command::Toggle(_) => Ok(Self::new(
                    address.ieee_address(),
                    endpoint,
                    InboundProtocolCommand::Toggle,
                )),
            },
            Cluster::Level(level) => match level {
                level::Command::Move(mv) => {
                    let direction = match mv.mode() {
                        Ok(Mode::Up) => LevelMoveDirection::Up,
                        Ok(Mode::Down) => LevelMoveDirection::Down,
                        _ => {
                            return Err(crate::Event::new(
                                address,
                                endpoint,
                                Cluster::Level(level),
                            ));
                        }
                    };

                    Ok(Self::new(
                        address.ieee_address(),
                        endpoint,
                        InboundProtocolCommand::LevelMove {
                            direction,
                            rate: mv.rate(),
                        },
                    ))
                }
                level::Command::MoveWithOnOff(mv) => {
                    let direction = match mv.mode() {
                        Ok(Mode::Up) => LevelMoveDirection::Up,
                        Ok(Mode::Down) => LevelMoveDirection::Down,
                        _ => {
                            return Err(crate::Event::new(
                                address,
                                endpoint,
                                Cluster::Level(level),
                            ));
                        }
                    };

                    Ok(Self::new(
                        address.ieee_address(),
                        endpoint,
                        InboundProtocolCommand::LevelMove {
                            direction,
                            rate: mv.rate(),
                        },
                    ))
                }
                other => {
                    warn!("Unhandled level command: {other:?}");
                    Err(crate::Event::new(address, endpoint, Cluster::Level(other)))
                }
            },
            other => {
                warn!("Unhandled level cluster: {other:?}");
                Err(crate::Event::new(address, endpoint, other))
            }
        }
    }
}
