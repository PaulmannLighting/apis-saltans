use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive)]
pub enum Decision {
    AllowJoins = 0x00,
    AllowPreconfiguredKeyJoins = 0x01,
    AllowRejoinsOnly = 0x02,
    DisallowAllJoinsAndRejoins = 0x03,
    AllowJoinsRejoinsHaveLinkKey = 0x04,
    IgnoreTrustCenterRejoins = 0x05,
    BdbJoinUsesInstallCodeKey = 0x06,
    DeferJoinsRejoinsHaveLinkKey = 0x07,
    DisallowBindingModification = 0x10,
    AllowBindingModification = 0x11,
    CheckBindingModificationsAreValidEndpointClusters = 0x12,
    HostWillNotSupplyReply = 0x20,
    HostWillSupplyReply = 0x21,
    PollHandlerIgnore = 0x30,
    PollHandlerCallback = 0x31,
    MessageTagOnlyInCallback = 0x40,
    MessageTagAndContentsInCallback = 0x41,
    DenyTcKeyRequests = 0x50,
    AllowTcKeyRequestsAndSendCurrentKey = 0x51,
    AllowTcKeyRequestsAndGenerateNewKey = 0x52,
    DenyAppKeyRequests = 0x60,
    AllowAppKeyRequests = 0x61,
    PacketValidateLibraryChecksEnabled = 0x62,
    PacketValidateLibraryChecksDisabled = 0x63,
}

impl From<Decision> for u8 {
    fn from(decision: Decision) -> Self {
        decision as u8
    }
}
