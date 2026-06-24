use std::time::Duration;

use log::warn;
use macaddr::MacAddr8;
use smarthomelib::command::{Dimming, OnOff, Timing};
use smarthomelib::{Command, Deciseconds, Event};
use zcl::Cluster;
use zcl::general::level::Mode;
use zcl::general::on_off::OnOffControl;
use zcl::general::{level, on_off};
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
                            Duration::from_deci_secs(params.on_time().unwrap_or_default().into()),
                            Duration::from_deci_secs(
                                params.off_wait_time().unwrap_or_default().into(),
                            ),
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
            Cluster::Level(level) => match level {
                level::Command::Move(mv) => {
                    let command = match mv.mode() {
                        Ok(Mode::Up) => Command::Dimming(Dimming::Up { rate: mv.rate() }),
                        Ok(Mode::Down) => Command::Dimming(Dimming::Down { rate: mv.rate() }),
                        _ => {
                            return Err(crate::Event::new(
                                address,
                                application.into(),
                                Cluster::Level(level),
                            ));
                        }
                    };

                    Ok(Self::new(address.ieee_address(), application, command))
                }
                level::Command::MoveWithOnOff(mv) => {
                    let command = match mv.mode() {
                        Ok(Mode::Up) => Command::Dimming(Dimming::Up { rate: mv.rate() }),
                        Ok(Mode::Down) => Command::Dimming(Dimming::Down { rate: mv.rate() }),
                        _ => {
                            return Err(crate::Event::new(
                                address,
                                application.into(),
                                Cluster::Level(level),
                            ));
                        }
                    };

                    Ok(Self::new(address.ieee_address(), application, command))
                }
                other => {
                    warn!("Unhandled level command: {other:?}");
                    Err(crate::Event::new(
                        address,
                        application.into(),
                        Cluster::Level(other),
                    ))
                }
            },
            other => {
                warn!("Unhandled level cluster: {other:?}");
                Err(crate::Event::new(address, application.into(), other))
            }
        }
    }
}
