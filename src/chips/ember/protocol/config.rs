use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Config {
    PacketBufferCount = 0x01,
    NeighborTableSize = 0x02,
    ApsUnicastMessageCount = 0x03,
    BindingTableSize = 0x04,
    AddressTableSize = 0x05,
    MulticastTableSize = 0x06,
    RouteTableSize = 0x07,
    DiscoveryTableSize = 0x08,
    StackProfile = 0x0C,
    SecurityLevel = 0x0D,
    MaxHops = 0x10,
    MaxEndDeviceChildren = 0x11,
    IndirectTransmissionTimeout = 0x12,
    EndDevicePollTimeout = 0x13,
    MobileNodePollTimeout = 0x14,
    ReservedMobileChildEntries = 0x15,
    TxPowerMode = 0x17,
    DisableRelay = 0x18,
    TrustCenterAddressCacheSize = 0x19,
    SourceRouteTableSize = 0x1A,
    EndDevicePollTimeoutShift = 0x1B,
    FragmentWindowSize = 0x1C,
    FragmentDelayMillis = 0x1D,
    KeyTableSize = 0x1E,
    ApsAckTimeout = 0x1F,
    BeaconJitterDuration = 0x20,
    EndDeviceBindTimeout = 0x21,
    PanIdConflictReportThreshold = 0x22,
    RequestKeyTimeout = 0x24,
    CertificateTableSize = 0x29,
    ApplicationZdoFlags = 0x2A,
    BroadcastTableSize = 0x2B,
    MacFilterTableSize = 0x2C,
    SupportedNetworks = 0x2D,
    SendMulticastsToSleepyAddress = 0x2E,
    ZllGroupAddress = 0x2F,
    ZllRssiThreshold = 0x30,
    MTorrFlowControl = 0x33,
    RetryQueueSize = 0x34,
    NewBroadcastEntryThreshold = 0x35,
    TransientKeyTimeoutSec = 0x36,
    BroadcastMinAcknowledgementsNeeded = 0x37,
    TcRejoinsUsingWellKnownKeyTimeoutSec = 0x38,
    CTuneValue = 0x39,
}

impl From<Config> for u8 {
    fn from(config_id: Config) -> Self {
        config_id.to_u8().expect("Could not convert Config to u8")
    }
}
