macro_rules! zcl_cluster_profile {
    ([]) => {
        zb_core::Profile::ZigbeeHomeAutomation
    };
    ([$profile:expr]) => {
        $profile
    };
}

pub(crate) use zcl_cluster_profile;
