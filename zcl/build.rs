//! Track changes to compile-time configuration.
fn main() {
    for name in [
        "ZCL_DISABLE_DEFAULT_RESPONSE",
        "ZIGBEE_ZCL_RECORDS_CAPACITY",
        "ZIGBEE_ZCL_ATTRIBUTE_IDS_CAPACITY",
        "ZIGBEE_ZCL_COMMAND_IDS_CAPACITY",
        "ZIGBEE_ZCL_INDEXES_CAPACITY",
        "ZIGBEE_ZCL_IMAGE_DATA_CAPACITY",
        "ZIGBEE_ZCL_EXTENSIONS_CAPACITY",
        "ZIGBEE_ZCL_REPORTABLE_CHANGE_CAPACITY",
        "ZIGBEE_ZCL_PAYLOAD_CAPACITY",
        "ZIGBEE_ZCL_WIRE_CAPACITY",
    ] {
        println!("cargo:rerun-if-env-changed={name}");
    }
}
