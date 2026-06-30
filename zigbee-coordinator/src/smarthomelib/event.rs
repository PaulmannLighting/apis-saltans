use std::time::Duration;

use log::warn;
use macaddr::MacAddr8;
use smarthomelib::command::{Dimming, OnOff, OpenClosed, Timing};
use smarthomelib::{Command, Event};
use zcl::general::level::Mode;
use zcl::general::on_off::OnOffControl;
use zcl::general::{level, on_off};
use zcl::{Cluster, ias};
use zigbee::{Application, Endpoint};

impl TryFrom<crate::Event> for Event<MacAddr8, Application> {
    type Error = crate::Event;

    fn try_from(event: crate::Event) -> Result<Self, Self::Error> {
        let (address, endpoint, cluster) = event.into_parts();

        let Endpoint::Application(application) = endpoint else {
            return Err(crate::Event::new(address, endpoint, cluster));
        };

        match cluster {
            Cluster::OnOff(on_off) => match on_off {
                on_off::Command::On(_) => Ok(Self::new(
                    address.ieee_address(),
                    application,
                    Command::OnOff(OnOff::On {
                        recall_scene: false,
                        timing: None,
                    }),
                )),
                on_off::Command::Off(_) => Ok(Self::new(
                    address.ieee_address(),
                    application,
                    Command::OnOff(OnOff::Off { effect: None }),
                )),
                on_off::Command::OffWithEffect(_params) => Ok(Self::new(
                    address.ieee_address(),
                    application,
                    // TODO: Handle effects
                    Command::OnOff(OnOff::Off { effect: None }),
                )),
                on_off::Command::OnWithTimedOff(params) => Ok(Self::new(
                    address.ieee_address(),
                    application,
                    Command::OnOff(OnOff::On {
                        recall_scene: false,
                        timing: Some(Timing::new(
                            params
                                .on_off_control()
                                .contains(OnOffControl::ACCEPT_ONLY_WHEN_ON),
                            Duration::from(params.on_time().unwrap_or_default()),
                            Duration::from(params.off_wait_time().unwrap_or_default()),
                        )),
                    }),
                )),
                on_off::Command::OnWithRecallGlobalScene(_) => Ok(Self::new(
                    address.ieee_address(),
                    application,
                    Command::OnOff(OnOff::On {
                        recall_scene: true,
                        timing: None,
                    }),
                )),
                on_off::Command::Toggle(_) => Ok(Self::new(
                    address.ieee_address(),
                    application,
                    Command::OnOff(OnOff::Toggle),
                )),
            },
            Cluster::Level(level) => match translate_level_command(level) {
                Ok(command) => Ok(Self::new(address.ieee_address(), application, command)),
                Err(command) => Err(crate::Event::new(
                    address,
                    endpoint,
                    Cluster::Level(command),
                )),
            },
            Cluster::IasZone(ias_zone) => match ias_zone {
                ias::zone::Command::StatusChange(status_change) => Ok(Self::new(
                    address.ieee_address(),
                    application,
                    Command::OpenClosed(
                        if status_change.status().contains(ias::zone::Status::ALARM_1) {
                            OpenClosed::Open
                        } else {
                            OpenClosed::Closed
                        },
                    ),
                )),
            },
            other => {
                warn!("Unhandled level cluster: {other:?}");
                Err(crate::Event::new(address, application.into(), other))
            }
        }
    }
}

fn translate_level_command(command: level::Command) -> Result<Command, level::Command> {
    match command {
        level::Command::Move(mv) => {
            let rate = if let Some(rate) = mv.rate() {
                rate.into()
            } else {
                return Err(command);
            };

            match mv.mode() {
                Ok(Mode::Up) => Ok(Command::Dimming(Dimming::Up { rate })),
                Ok(Mode::Down) => Ok(Command::Dimming(Dimming::Down { rate })),
                _ => Err(command),
            }
        }
        level::Command::MoveWithOnOff(mv) => {
            let rate = if let Some(rate) = mv.rate() {
                rate.into()
            } else {
                return Err(command);
            };

            match mv.mode() {
                Ok(Mode::Up) => Ok(Command::Dimming(Dimming::Up { rate })),
                Ok(Mode::Down) => Ok(Command::Dimming(Dimming::Down { rate })),
                _ => Err(command),
            }
        }
        other => {
            warn!("Unhandled level command: {other:?}");
            Err(other)
        }
    }
}
