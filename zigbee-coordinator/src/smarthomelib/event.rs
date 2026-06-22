use log::warn;
use macaddr::MacAddr8;
use smarthomelib::command::{Dimming, OnOff, Timing};
use smarthomelib::{Command, Event};
use zcl::Cluster;
use zcl::general::level::Mode;
use zcl::general::on_off::OnOffControl;
use zcl::general::{level, on_off};
use zigbee::Endpoint;

impl TryFrom<crate::Event> for Event<MacAddr8, Endpoint> {
    type Error = crate::Event;

    fn try_from(event: crate::Event) -> Result<Self, Self::Error> {
        let (address, endpoint, cluster) = event.into_parts();

        match cluster {
            Cluster::OnOff(on_off) => match on_off {
                on_off::Command::On(_) => Ok(Self::new(
                    address.ieee_address(),
                    endpoint,
                    Command::OnOff(OnOff::On {
                        recall_scene: false,
                        timing: None,
                    }),
                )),
                on_off::Command::Off(_) => Ok(Self::new(
                    address.ieee_address(),
                    endpoint,
                    Command::OnOff(OnOff::Off { effect: None }),
                )),
                on_off::Command::OffWithEffect(_params) => Ok(Self::new(
                    address.ieee_address(),
                    endpoint,
                    // TODO: Handle effects
                    Command::OnOff(OnOff::Off { effect: None }),
                )),
                on_off::Command::OnWithTimedOff(params) => Ok(Self::new(
                    address.ieee_address(),
                    endpoint,
                    Command::OnOff(OnOff::On {
                        recall_scene: false,
                        timing: Some(Timing::new(
                            params
                                .on_off_control()
                                .contains(OnOffControl::ACCEPT_ONLY_WHEN_ON),
                            params.on_time(),
                            params.off_wait_time(),
                        )),
                    }),
                )),
                on_off::Command::OnWithRecallGlobalScene(_) => Ok(Self::new(
                    address.ieee_address(),
                    endpoint,
                    Command::OnOff(OnOff::On {
                        recall_scene: true,
                        timing: None,
                    }),
                )),
                on_off::Command::Toggle(_) => Ok(Self::new(
                    address.ieee_address(),
                    endpoint,
                    Command::OnOff(OnOff::Toggle),
                )),
            },
            Cluster::Level(level) => match level {
                level::Command::Move(mv) => {
                    let command = match mv.mode() {
                        Ok(Mode::Up) => Command::Dimming(Dimming::Up { rate: mv.rate() }),
                        Ok(Mode::Down) => Command::Dimming(Dimming::Down { rate: mv.rate() }),
                        _ => {
                            return Err(crate::Event::new(
                                address,
                                endpoint,
                                Cluster::Level(level),
                            ));
                        }
                    };

                    Ok(Self::new(address.ieee_address(), endpoint, command))
                }
                level::Command::MoveWithOnOff(mv) => {
                    let command = match mv.mode() {
                        Ok(Mode::Up) => Command::Dimming(Dimming::Up { rate: mv.rate() }),
                        Ok(Mode::Down) => Command::Dimming(Dimming::Down { rate: mv.rate() }),
                        _ => {
                            return Err(crate::Event::new(
                                address,
                                endpoint,
                                Cluster::Level(level),
                            ));
                        }
                    };

                    Ok(Self::new(address.ieee_address(), endpoint, command))
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
