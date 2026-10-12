//! Wire codecs, command metadata, and conversions for both Upgrade End request forms.

use core::iter::Chain;

use le_stream::{FromLeStream, ToLeStream};
use zb_core::{ClusterSpecific, ExpectResponse, Profile, Profiled};

use super::{UpgradeComplete, UpgradeEndRequest, UpgradeIncomplete, UpgradeIncompleteReason};
use crate::ota_upgrade::{ImageId, UpgradeEndResponse, UpgradeEndStatus};
use crate::{
    Cluster, Command, DefaultResponsePolicy, Directed, Direction, NoSpecificResponse,
    ParseDirection, RequestPolicy,
};

const COMMAND_ID: u8 = 0x06;

type WireBytes = Chain<<UpgradeEndStatus as ToLeStream>::Iter, <ImageId as ToLeStream>::Iter>;

impl FromLeStream for UpgradeEndRequest {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let status = UpgradeEndStatus::from_le_stream(&mut bytes)?;
        let image = ImageId::from_le_stream(bytes)?;
        let reason = match status {
            UpgradeEndStatus::Success => return Some(UpgradeComplete::new(image).into()),
            UpgradeEndStatus::Abort => UpgradeIncompleteReason::Abort,
            UpgradeEndStatus::InvalidImage => UpgradeIncompleteReason::InvalidImage,
            UpgradeEndStatus::RequireMoreImage => UpgradeIncompleteReason::RequireMoreImage,
        };
        Some(UpgradeIncomplete::new(reason, image).into())
    }
}

impl ToLeStream for UpgradeEndRequest {
    type Iter = WireBytes;

    fn to_le_stream(self) -> Self::Iter {
        self.status()
            .to_le_stream()
            .chain(self.image().to_le_stream())
    }
}

impl From<UpgradeEndRequest> for Cluster {
    fn from(request: UpgradeEndRequest) -> Self {
        Self::OtaUpgrade(crate::ota_upgrade::Command::UpgradeEndRequest(Box::new(
            request,
        )))
    }
}

impl TryFrom<Cluster> for UpgradeEndRequest {
    type Error = Cluster;

    fn try_from(cluster: Cluster) -> Result<Self, Self::Error> {
        if let Cluster::OtaUpgrade(crate::ota_upgrade::Command::UpgradeEndRequest(request)) =
            cluster
        {
            Ok(*request)
        } else {
            Err(cluster)
        }
    }
}

impl Command for UpgradeEndRequest {
    const ID: u8 = COMMAND_ID;
    const PARSE_DIRECTION: ParseDirection = ParseDirection::Single(Direction::ClientToServer);
    const DISABLE_DEFAULT_RESPONSE: bool = false;
}

impl Directed for UpgradeEndRequest {
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl ClusterSpecific<zb_core::Cluster> for UpgradeEndRequest {
    const ID: zb_core::Cluster = zb_core::Cluster::OtaUpgrade;
}

impl Profiled for UpgradeEndRequest {
    const PROFILE: Profile = Profile::ZigbeeHomeAutomation;
}

impl Command for UpgradeComplete {
    const ID: u8 = COMMAND_ID;
    const PARSE_DIRECTION: ParseDirection = ParseDirection::Single(Direction::ClientToServer);
    const DISABLE_DEFAULT_RESPONSE: bool = false;
}

impl Directed for UpgradeComplete {
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl ClusterSpecific<zb_core::Cluster> for UpgradeComplete {
    const ID: zb_core::Cluster = zb_core::Cluster::OtaUpgrade;
}

impl Profiled for UpgradeComplete {
    const PROFILE: Profile = Profile::ZigbeeHomeAutomation;
}

impl Command for UpgradeIncomplete {
    const ID: u8 = COMMAND_ID;
    const PARSE_DIRECTION: ParseDirection = ParseDirection::Single(Direction::ClientToServer);
    const DISABLE_DEFAULT_RESPONSE: bool = false;
}

impl Directed for UpgradeIncomplete {
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl ClusterSpecific<zb_core::Cluster> for UpgradeIncomplete {
    const ID: zb_core::Cluster = zb_core::Cluster::OtaUpgrade;
}

impl Profiled for UpgradeIncomplete {
    const PROFILE: Profile = Profile::ZigbeeHomeAutomation;
}

impl ExpectResponse<Cluster> for UpgradeComplete {
    type Response = UpgradeEndResponse;
}

impl From<UpgradeComplete> for UpgradeEndRequest {
    fn from(request: UpgradeComplete) -> Self {
        Self::Complete(request)
    }
}

impl TryFrom<UpgradeEndRequest> for UpgradeComplete {
    type Error = UpgradeEndRequest;

    fn try_from(request: UpgradeEndRequest) -> Result<Self, Self::Error> {
        match request {
            UpgradeEndRequest::Complete(request) => Ok(request),
            other @ UpgradeEndRequest::Incomplete(_) => Err(other),
        }
    }
}

impl From<UpgradeComplete> for Cluster {
    fn from(request: UpgradeComplete) -> Self {
        UpgradeEndRequest::from(request).into()
    }
}

impl FromLeStream for UpgradeComplete {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Self::try_from(UpgradeEndRequest::from_le_stream(bytes)?).ok()
    }
}

impl ToLeStream for UpgradeComplete {
    type Iter = WireBytes;

    fn to_le_stream(self) -> Self::Iter {
        UpgradeEndRequest::from(self).to_le_stream()
    }
}

impl ExpectResponse<Cluster> for UpgradeIncomplete {
    type Response = NoSpecificResponse;
}

impl From<UpgradeIncomplete> for UpgradeEndRequest {
    fn from(request: UpgradeIncomplete) -> Self {
        Self::Incomplete(request)
    }
}

impl TryFrom<UpgradeEndRequest> for UpgradeIncomplete {
    type Error = UpgradeEndRequest;

    fn try_from(request: UpgradeEndRequest) -> Result<Self, Self::Error> {
        match request {
            UpgradeEndRequest::Incomplete(request) => Ok(request),
            other @ UpgradeEndRequest::Complete(_) => Err(other),
        }
    }
}

impl From<UpgradeIncomplete> for Cluster {
    fn from(request: UpgradeIncomplete) -> Self {
        UpgradeEndRequest::from(request).into()
    }
}

impl FromLeStream for UpgradeIncomplete {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Self::try_from(UpgradeEndRequest::from_le_stream(bytes)?).ok()
    }
}

impl ToLeStream for UpgradeIncomplete {
    type Iter = WireBytes;

    fn to_le_stream(self) -> Self::Iter {
        UpgradeEndRequest::from(self).to_le_stream()
    }
}

impl From<UpgradeComplete> for crate::ota_upgrade::Command {
    fn from(request: UpgradeComplete) -> Self {
        UpgradeEndRequest::from(request).into()
    }
}

impl From<UpgradeIncomplete> for crate::ota_upgrade::Command {
    fn from(request: UpgradeIncomplete) -> Self {
        UpgradeEndRequest::from(request).into()
    }
}

impl RequestPolicy for UpgradeComplete {
    const DEFAULT_RESPONSE_POLICY: DefaultResponsePolicy = DefaultResponsePolicy::SpecificRequired;
}

impl RequestPolicy for UpgradeIncomplete {
    const DEFAULT_RESPONSE_POLICY: DefaultResponsePolicy = DefaultResponsePolicy::DefaultAllowed;
}
