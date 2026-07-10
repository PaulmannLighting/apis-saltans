//! Security ZDP services.

pub use self::security_challenge_req::SecurityChallengeReq;
pub use self::security_challenge_rsp::SecurityChallengeRsp;
pub use self::security_decommission_req::SecurityDecommissionReq;
pub use self::security_decommission_rsp::SecurityDecommissionRsp;
pub use self::security_get_authentication_level_req::SecurityGetAuthenticationLevelReq;
pub use self::security_get_authentication_level_rsp::SecurityGetAuthenticationLevelRsp;
pub use self::security_get_configuration_req::SecurityGetConfigurationReq;
pub use self::security_get_configuration_rsp::SecurityGetConfigurationRsp;
pub use self::security_retrieve_authentication_token_req::SecurityRetrieveAuthenticationTokenReq;
pub use self::security_retrieve_authentication_token_rsp::SecurityRetrieveAuthenticationTokenRsp;
pub use self::security_set_configuration_req::SecuritySetConfigurationReq;
pub use self::security_set_configuration_rsp::SecuritySetConfigurationRsp;
pub use self::security_start_key_negotiation_req::SecurityStartKeyNegotiationReq;
pub use self::security_start_key_negotiation_rsp::SecurityStartKeyNegotiationRsp;
pub use self::security_start_key_update_req::SecurityStartKeyUpdateReq;
pub use self::security_start_key_update_rsp::SecurityStartKeyUpdateRsp;

mod security_challenge_req;
mod security_challenge_rsp;
mod security_decommission_req;
mod security_decommission_rsp;
mod security_get_authentication_level_req;
mod security_get_authentication_level_rsp;
mod security_get_configuration_req;
mod security_get_configuration_rsp;
mod security_retrieve_authentication_token_req;
mod security_retrieve_authentication_token_rsp;
mod security_set_configuration_req;
mod security_set_configuration_rsp;
mod security_start_key_negotiation_req;
mod security_start_key_negotiation_rsp;
mod security_start_key_update_req;
mod security_start_key_update_rsp;

crate::zdp_command_group! {
    /// Security Commands.
    Security {
        SecurityStartKeyNegotiationReq,
        SecurityRetrieveAuthenticationTokenReq,
        SecurityGetAuthenticationLevelReq,
        SecuritySetConfigurationReq,
        SecurityGetConfigurationReq,
        SecurityStartKeyUpdateReq,
        SecurityDecommissionReq,
        SecurityChallengeReq,
        SecurityStartKeyNegotiationRsp,
        SecurityRetrieveAuthenticationTokenRsp,
        SecurityGetAuthenticationLevelRsp,
        SecuritySetConfigurationRsp,
        SecurityGetConfigurationRsp,
        SecurityStartKeyUpdateRsp,
        SecurityDecommissionRsp,
        SecurityChallengeRsp,
    }
}
