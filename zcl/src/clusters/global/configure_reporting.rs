//! Reporting configuration commands for the Global cluster.

use std::boxed::Box;
use std::iter::once;

use either::Either;
use le_stream::{FromLeStream, ToLeStream};
use zb_core::Direction;

pub use self::attribute_status::AttributeStatus;
pub use self::receive::Command as Receive;
pub use self::send::Command as Send;
use crate::macros::zcl_command;
use crate::{Command, ParseDirection, Scope, Scoped};

mod attribute_status;
pub mod receive;
pub mod send;

const COMMAND_ID: u8 = 0x06;

/// A command that configures a device to send or receive attribute reports.
///
/// Use [`Send`] or [`Receive`] directly when constructing an outgoing command so that the frame
/// direction is selected from the concrete command type.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ConfigureReporting {
    /// Configure the target to send attribute reports to its bindings.
    Send(Send),
    /// Configure the target to receive attribute reports.
    Receive(Receive),
}

impl Command for ConfigureReporting {
    const ID: u8 = COMMAND_ID;
    const DIRECTION: Direction = Direction::ClientToServer;
    const PARSE_DIRECTION: ParseDirection = ParseDirection::Both;
}

impl Scoped for ConfigureReporting {
    const SCOPE: Scope = Scope::Global;
}

impl FromLeStream for ConfigureReporting {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let direction = bytes.next()?;
        let bytes = once(direction).chain(bytes);

        match Direction::try_from(direction) {
            Ok(Direction::ClientToServer) => Send::from_le_stream(bytes).map(Self::Send),
            Ok(Direction::ServerToClient) => Receive::from_le_stream(bytes).map(Self::Receive),
            Err(_) => None,
        }
    }
}

impl ToLeStream for ConfigureReporting {
    type Iter = Either<<Send as ToLeStream>::Iter, <Receive as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Send(command) => Either::Left(command.to_le_stream()),
            Self::Receive(command) => Either::Right(command.to_le_stream()),
        }
    }
}

impl From<Send> for ConfigureReporting {
    fn from(command: Send) -> Self {
        Self::Send(command)
    }
}

impl From<Receive> for ConfigureReporting {
    fn from(command: Receive) -> Self {
        Self::Receive(command)
    }
}

impl TryFrom<ConfigureReporting> for Send {
    type Error = ConfigureReporting;

    fn try_from(command: ConfigureReporting) -> Result<Self, Self::Error> {
        match command {
            ConfigureReporting::Send(command) => Ok(command),
            other @ ConfigureReporting::Receive(_) => Err(other),
        }
    }
}

impl TryFrom<ConfigureReporting> for Receive {
    type Error = ConfigureReporting;

    fn try_from(command: ConfigureReporting) -> Result<Self, Self::Error> {
        match command {
            ConfigureReporting::Receive(command) => Ok(command),
            other @ ConfigureReporting::Send(_) => Err(other),
        }
    }
}

impl From<Send> for crate::Cluster {
    fn from(command: Send) -> Self {
        ConfigureReporting::from(command).into()
    }
}

impl From<Receive> for crate::Cluster {
    fn from(command: Receive) -> Self {
        ConfigureReporting::from(command).into()
    }
}

impl TryFrom<crate::Cluster> for Send {
    type Error = crate::Cluster;

    fn try_from(cluster: crate::Cluster) -> Result<Self, Self::Error> {
        let reporting = ConfigureReporting::try_from(cluster)?;
        reporting.try_into().map_err(Into::into)
    }
}

impl TryFrom<crate::Cluster> for Receive {
    type Error = crate::Cluster;

    fn try_from(cluster: crate::Cluster) -> Result<Self, Self::Error> {
        let reporting = ConfigureReporting::try_from(cluster)?;
        reporting.try_into().map_err(Into::into)
    }
}

impl From<ConfigureReporting> for crate::Cluster {
    fn from(command: ConfigureReporting) -> Self {
        Self::Global(crate::global::Command::ConfigureReporting(command.into()))
    }
}

impl TryFrom<crate::Cluster> for ConfigureReporting {
    type Error = crate::Cluster;

    fn try_from(cluster: crate::Cluster) -> Result<Self, Self::Error> {
        if let crate::Cluster::Global(crate::global::Command::ConfigureReporting(command)) = cluster
        {
            Ok(*command)
        } else {
            Err(cluster)
        }
    }
}

zcl_command! {
    /// Status of an attribute reporting configuration.
    Response {
        Global;
        command_id: 0x07;
        direction: Direction::ServerToClient;
        => crate::global::ConfigureReportingResponse;
        fields {
            status: Box<[AttributeStatus]>,
        }

        getters {
            /// Returns the status.
            #[must_use]
            pub fn status(&self) -> &[AttributeStatus] {
                &self.status
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use le_stream::{FromLeStream, ToLeStream};
    use zb_core::Direction;

    use super::{ConfigureReporting, Receive, Send, receive, send};
    use crate::Command;

    const ATTRIBUTE_ID: u16 = 0x1234;
    const ATTRIBUTE_DATA_TYPE: u8 = 0x20;
    const MINIMUM_REPORTING_INTERVAL: u16 = 0x0102;
    const MAXIMUM_REPORTING_INTERVAL: u16 = 0x0304;
    const TIMEOUT_PERIOD: u16 = 0x0506;

    #[test]
    fn send_command_has_client_to_server_direction() {
        assert_eq!(Send::DIRECTION, Direction::ClientToServer);
    }

    #[test]
    fn receive_command_has_server_to_client_direction() {
        assert_eq!(Receive::DIRECTION, Direction::ServerToClient);
    }

    #[test]
    fn serializes_and_parses_send_command() {
        let attribute = send::AttributeReportingConfiguration::new(
            ATTRIBUTE_ID,
            ATTRIBUTE_DATA_TYPE,
            MINIMUM_REPORTING_INTERVAL,
            MAXIMUM_REPORTING_INTERVAL,
            None,
        );
        let command = ConfigureReporting::from(Send::new(Box::new([attribute])));
        let bytes: Vec<_> = command.clone().to_le_stream().collect();
        let mut expected = vec![Direction::ClientToServer as u8];
        expected.extend(ATTRIBUTE_ID.to_le_bytes());
        expected.push(ATTRIBUTE_DATA_TYPE);
        expected.extend(MINIMUM_REPORTING_INTERVAL.to_le_bytes());
        expected.extend(MAXIMUM_REPORTING_INTERVAL.to_le_bytes());

        assert_eq!(bytes, expected);
        assert_eq!(
            ConfigureReporting::from_le_stream(bytes.into_iter()),
            Some(command)
        );
    }

    #[test]
    fn serializes_and_parses_receive_command() {
        let attribute = receive::AttributeReportingConfiguration::new(ATTRIBUTE_ID, TIMEOUT_PERIOD);
        let command = ConfigureReporting::from(Receive::new(Box::new([attribute])));
        let bytes: Vec<_> = command.clone().to_le_stream().collect();
        let mut expected = vec![Direction::ServerToClient as u8];
        expected.extend(ATTRIBUTE_ID.to_le_bytes());
        expected.extend(TIMEOUT_PERIOD.to_le_bytes());

        assert_eq!(bytes, expected);
        assert_eq!(
            ConfigureReporting::from_le_stream(bytes.into_iter()),
            Some(command)
        );
    }

    #[test]
    fn rejects_unknown_reporting_direction() {
        const UNKNOWN_REPORTING_DIRECTION: u8 = 0x02;

        assert_eq!(
            ConfigureReporting::from_le_stream([UNKNOWN_REPORTING_DIRECTION].into_iter()),
            None
        );
    }
}
