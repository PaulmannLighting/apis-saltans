use super::{ConcentratorConfig, TrustCenterJoinMode};

pub enum ConfigOption {
    ConcentratorConfig(ConcentratorConfig),
    TrustCenterJoinMode(TrustCenterJoinMode),
    TrustCenterLinkKey(Key),
    SupportedInputClusters,
    SupportedOutputClusters,
    InstallKey,
    DeviceType,
    RadioTxPower,
}
