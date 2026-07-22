//! OTA Upgrade cluster implementation.
//!
//! The cluster models image discovery, block and page transfer, and upgrade completion for OTA
//! clients and servers.

pub use self::attributes::{
    Id, ImageUpgradeStatus, Readable, Reportable, SendReport, UpgradeServerId, Writable,
};
pub use self::commands::{
    Command, ImageBlockRequest, ImageBlockResponse, ImageNotify, ImagePageRequest,
    QueryNextImageRequest, QueryNextImageResponse, QuerySpecificFileRequest,
    QuerySpecificFileResponse, UpgradeEndRequest, UpgradeEndResponse,
};
pub use self::types::{
    ImageBlock, ImageBlockResponsePayload, ImageId, ImageNotifyPayload, QueryJitter, QueryResponse,
    UpgradeEndStatus, WaitForData,
};

mod attributes;
mod commands;
mod types;

#[cfg(test)]
mod tests {
    use le_stream::ToLeStream;
    use zb_core::{Cluster as ClusterId, IeeeAddress};

    use super::{
        ImageBlock, ImageBlockRequest, ImageBlockResponse, ImageId, ImageNotify,
        ImageNotifyPayload, ImagePageRequest, QueryJitter, QueryNextImageRequest,
        QueryNextImageResponse, QueryResponse, QuerySpecificFileRequest, QuerySpecificFileResponse,
        UpgradeEndRequest, UpgradeEndResponse, UpgradeEndStatus,
    };
    use crate::{Command as CommandMetadata, Directed, Frame, Header, Scope};

    const SEQUENCE_NUMBER: u8 = 0x42;
    const HARDWARE_VERSION: u16 = 0x0102;
    const FILE_OFFSET: u32 = 0x0000_0080;
    const MAXIMUM_DATA_SIZE: u8 = 64;
    const PAGE_SIZE: u16 = 1024;
    const RESPONSE_SPACING: u16 = 100;
    const MINIMUM_BLOCK_PERIOD: u16 = 2;
    const CURRENT_TIME: u32 = 1000;
    const UPGRADE_TIME: u32 = 1100;
    const IMAGE_SIZE: u32 = 4096;
    const ZIGBEE_STACK_VERSION: u16 = 0x0002;
    const IMAGE: ImageId = ImageId::new(0x1234, 0x5678, 0x90AB_CDEF);
    const ADDRESS: IeeeAddress = IeeeAddress::new(1, 2, 3, 4, 5, 6, 7, 8);

    fn assert_runtime_round_trip<T>(command: T)
    where
        T: Clone + CommandMetadata + Directed + Into<crate::Cluster> + ToLeStream,
    {
        let expected = command.clone().into();
        let header = Header::new(
            Scope::ClusterSpecific,
            T::DIRECTION,
            T::DISABLE_DEFAULT_RESPONSE,
            T::MANUFACTURER_CODE,
            SEQUENCE_NUMBER,
            T::ID,
        );
        let bytes = header.to_le_stream().chain(command.to_le_stream());
        let frame = Frame::parse(ClusterId::OtaUpgrade.as_u16(), bytes)
            .expect("valid OTA command should parse");

        assert_eq!(frame.into_payload(), expected);
    }

    #[test]
    fn every_command_round_trips_through_runtime_dispatch() {
        let query_jitter = QueryJitter::new(QueryJitter::MAX).expect("valid query jitter");
        let image_data = Box::from([1, 2, 3, 4]);
        let image_block = ImageBlock::try_new(IMAGE, FILE_OFFSET, image_data)
            .expect("small image block should be valid");

        assert_runtime_round_trip(ImageNotify::new(ImageNotifyPayload::FileVersion {
            query_jitter,
            image: IMAGE,
        }));
        assert_runtime_round_trip(QueryNextImageRequest::new(IMAGE, Some(HARDWARE_VERSION)));
        assert_runtime_round_trip(QueryNextImageResponse::new(QueryResponse::Success {
            image: IMAGE,
            image_size: IMAGE_SIZE,
        }));
        assert_runtime_round_trip(ImageBlockRequest::new(
            IMAGE,
            FILE_OFFSET,
            MAXIMUM_DATA_SIZE,
            Some(ADDRESS),
            Some(MINIMUM_BLOCK_PERIOD),
        ));
        assert_runtime_round_trip(ImagePageRequest::new(
            IMAGE,
            FILE_OFFSET,
            MAXIMUM_DATA_SIZE,
            PAGE_SIZE,
            RESPONSE_SPACING,
            Some(ADDRESS),
        ));
        assert_runtime_round_trip(ImageBlockResponse::new(
            super::ImageBlockResponsePayload::Success(image_block),
        ));
        assert_runtime_round_trip(UpgradeEndRequest::new(UpgradeEndStatus::Success, IMAGE));
        assert_runtime_round_trip(UpgradeEndResponse::new(IMAGE, CURRENT_TIME, UPGRADE_TIME));
        assert_runtime_round_trip(QuerySpecificFileRequest::new(
            ADDRESS,
            IMAGE,
            ZIGBEE_STACK_VERSION,
        ));
        assert_runtime_round_trip(QuerySpecificFileResponse::new(
            QueryResponse::NoImageAvailable,
        ));
    }
}
