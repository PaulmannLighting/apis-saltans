use macaddr::MacAddr8;
use smarthomelib::{ZigbeeEndpointNumber, ZigbeeGroupId, ZigbeeIdCodec, ZigbeeIeeeAddress};

use crate::Coordinator;

impl ZigbeeIdCodec for Coordinator {
    type GroupId = u16;

    fn zigbee_device_id(&self, public: ZigbeeIeeeAddress) -> Self::DeviceId {
        MacAddr8::from(public.0.to_be_bytes())
    }

    fn zigbee_endpoint_id(&self, public: ZigbeeEndpointNumber) -> Self::EndpointId {
        public.0.into()
    }

    fn zigbee_group_id(&self, public: ZigbeeGroupId) -> Self::GroupId {
        public.0
    }

    fn public_zigbee_device_id(&self, native: Self::DeviceId) -> ZigbeeIeeeAddress {
        ZigbeeIeeeAddress(u64::from_be_bytes(native.into_array()))
    }

    fn public_zigbee_endpoint_id(&self, native: Self::EndpointId) -> ZigbeeEndpointNumber {
        ZigbeeEndpointNumber(native.into())
    }

    fn public_zigbee_group_id(&self, native: Self::GroupId) -> ZigbeeGroupId {
        ZigbeeGroupId(native)
    }
}

// Inline tests avoid making a production-only Coordinator constructor public just to exercise the
// public ZigbeeIdCodec mapping behavior.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zigbee_id_codec_when_mapping_macaddr8_then_preserves_display_byte_order() {
        let coordinator = CoordinatorIdCodecTest;
        let native = MacAddr8::new(0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef);

        assert_eq!(
            coordinator.public_zigbee_device_id(native),
            ZigbeeIeeeAddress(0x0123_4567_89ab_cdef)
        );
        assert_eq!(
            coordinator.zigbee_device_id(ZigbeeIeeeAddress(0x0123_4567_89ab_cdef)),
            native
        );
    }

    #[test]
    fn zigbee_id_codec_when_mapping_endpoint_then_uses_public_endpoint_number() {
        let coordinator = CoordinatorIdCodecTest;
        let native = zigbee::Endpoint::from(11);

        assert_eq!(
            coordinator.public_zigbee_endpoint_id(native),
            ZigbeeEndpointNumber(11)
        );
        assert_eq!(
            coordinator.zigbee_endpoint_id(ZigbeeEndpointNumber(11)),
            native
        );
    }

    #[derive(Clone, Copy)]
    struct CoordinatorIdCodecTest;

    impl smarthomelib::Protocol for CoordinatorIdCodecTest {
        type DeviceId = MacAddr8;
        type EndpointId = zigbee::Endpoint;
        type Error = crate::Error;
    }

    impl ZigbeeIdCodec for CoordinatorIdCodecTest {
        type GroupId = u16;

        fn zigbee_device_id(&self, public: ZigbeeIeeeAddress) -> Self::DeviceId {
            MacAddr8::from(public.0.to_be_bytes())
        }

        fn zigbee_endpoint_id(&self, public: ZigbeeEndpointNumber) -> Self::EndpointId {
            public.0.into()
        }

        fn zigbee_group_id(&self, public: ZigbeeGroupId) -> Self::GroupId {
            public.0
        }

        fn public_zigbee_device_id(&self, native: Self::DeviceId) -> ZigbeeIeeeAddress {
            ZigbeeIeeeAddress(u64::from_be_bytes(native.into_array()))
        }

        fn public_zigbee_endpoint_id(&self, native: Self::EndpointId) -> ZigbeeEndpointNumber {
            ZigbeeEndpointNumber(native.into())
        }

        fn public_zigbee_group_id(&self, native: Self::GroupId) -> ZigbeeGroupId {
            ZigbeeGroupId(native)
        }
    }
}
