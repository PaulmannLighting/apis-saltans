//! Track changes to compile-time allocation-free capacities.

fn main() {
    for name in [
        "ZIGBEE_APS_PAYLOAD_CAPACITY",
        "ZIGBEE_APS_FRAGMENT_CAPACITY",
        "ZIGBEE_APS_FRAGMENTS_CAPACITY",
        "ZIGBEE_APS_TRANSACTIONS_CAPACITY",
    ] {
        println!("cargo:rerun-if-env-changed={name}");
    }
}
