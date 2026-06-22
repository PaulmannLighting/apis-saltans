use log::warn;
use macaddr::MacAddr8;
use smarthomelib::command::{Dimming, OnOff, Timing};
use smarthomelib::{Command, Event, EventReceiver};
use tokio::sync::mpsc::Receiver;
use zcl::Cluster;
use zcl::general::level::Mode;
use zcl::general::on_off::OnOffControl;
use zcl::general::{level, on_off};
use zigbee::Endpoint;

impl EventReceiver<MacAddr8, Endpoint> for crate::EventReceiver {
    async fn recv(&mut self) -> Option<Event<MacAddr8, Endpoint>> {
        let (address, endpoint, cluster) = Receiver::recv(self).await?.into_parts();

        match cluster {
            Cluster::OnOff(on_off) => match on_off {
                on_off::Command::On(_) => Some(Event::new(
                    address.ieee_address(),
                    endpoint,
                    Command::OnOff(OnOff::On {
                        recall_scene: false,
                        timing: None,
                    }),
                )),
                on_off::Command::Off(_) => Some(Event::new(
                    address.ieee_address(),
                    endpoint,
                    Command::OnOff(OnOff::Off { effect: None }),
                )),
                on_off::Command::OffWithEffect(_params) => Some(Event::new(
                    address.ieee_address(),
                    endpoint,
                    // TODO: Handle effects
                    Command::OnOff(OnOff::Off { effect: None }),
                )),
                on_off::Command::OnWithTimedOff(params) => Some(Event::new(
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
                on_off::Command::OnWithRecallGlobalScene(_) => Some(Event::new(
                    address.ieee_address(),
                    endpoint,
                    Command::OnOff(OnOff::On {
                        recall_scene: true,
                        timing: None,
                    }),
                )),
                on_off::Command::Toggle(_) => Some(Event::new(
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
                        _ => return None,
                    };

                    Some(Event::new(address.ieee_address(), endpoint, command))
                }
                level::Command::MoveWithOnOff(mv) => {
                    let command = match mv.mode() {
                        Ok(Mode::Up) => Command::Dimming(Dimming::Up { rate: mv.rate() }),
                        Ok(Mode::Down) => Command::Dimming(Dimming::Down { rate: mv.rate() }),
                        _ => return None,
                    };

                    Some(Event::new(address.ieee_address(), endpoint, command))
                }
                other => {
                    warn!("Unhandled level command: {other:?}");
                    None
                }
            },
            other => {
                warn!("Unhandled level cluster: {other:?}");
                None
            }
        }
    }
}
