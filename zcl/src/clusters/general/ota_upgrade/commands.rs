//! Commands of the OTA Upgrade cluster.

use zb_core::Cluster;

pub use self::image_block_request::ImageBlockRequest;
pub use self::image_block_response::ImageBlockResponse;
pub use self::image_notify::ImageNotify;
pub use self::image_page_request::ImagePageRequest;
pub use self::query_next_image_request::QueryNextImageRequest;
pub use self::query_next_image_response::QueryNextImageResponse;
pub use self::query_specific_file_request::QuerySpecificFileRequest;
pub use self::query_specific_file_response::QuerySpecificFileResponse;
pub use self::upgrade_end_request::UpgradeEndRequest;
pub use self::upgrade_end_response::UpgradeEndResponse;
use crate::macros::zcl_command_enum;

mod image_block_request;
mod image_block_response;
mod image_notify;
mod image_page_request;
mod query_next_image_request;
mod query_next_image_response;
mod query_specific_file_request;
mod query_specific_file_response;
mod upgrade_end_request;
mod upgrade_end_response;

zcl_command_enum! {
    { Cluster::OtaUpgrade } => OtaUpgrade;
    ImageNotify(ImageNotify),
    QueryNextImageRequest(QueryNextImageRequest),
    QueryNextImageResponse(QueryNextImageResponse),
    ImageBlockRequest(ImageBlockRequest),
    ImagePageRequest(ImagePageRequest),
    ImageBlockResponse(ImageBlockResponse),
    UpgradeEndRequest(UpgradeEndRequest),
    UpgradeEndResponse(UpgradeEndResponse),
    QuerySpecificFileRequest(QuerySpecificFileRequest),
    QuerySpecificFileResponse(QuerySpecificFileResponse),
}
