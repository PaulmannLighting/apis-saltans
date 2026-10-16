//! Track changes to bounded storage configuration.
fn main() {
    for name in [
        "ZIGBEE_ZDP_TLV_CAPACITY",
        "ZIGBEE_ZDP_ADDRESS_CAPACITY",
        "ZIGBEE_ZDP_CLUSTER_CAPACITY",
    ] {
        println!("cargo:rerun-if-env-changed={name}");
    }
}
