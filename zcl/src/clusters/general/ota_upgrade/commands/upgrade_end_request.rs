//! Statically distinct OTA client requests and their decoded wire representation.

use crate::ota_upgrade::{ImageId, UpgradeEndStatus};

mod wire;

#[cfg(test)]
mod tests;

/// A decoded Upgrade End Request, classified by its status byte.
///
/// Both variants share one wire command ID. Only the concrete variants implement
/// `ExpectResponse`, because each has a different response contract.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum UpgradeEndRequest {
    /// The downloaded image was validated and is ready to activate.
    Complete(UpgradeComplete),

    /// The client cannot activate the image and reports why.
    Incomplete(UpgradeIncomplete),
}

/// Reports a successfully downloaded and validated image; expects an Upgrade End Response.
///
/// Successful completion has no configurable failure reason or status field.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct UpgradeComplete {
    image: ImageId,
}

/// Reports an incomplete upgrade; expects only a Default Response acknowledging the report.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct UpgradeIncomplete {
    reason: UpgradeIncompleteReason,
    image: ImageId,
}

/// Reasons an OTA client cannot activate the downloaded image.
///
/// A wire status cannot be passed as an incomplete reason:
///
/// ```compile_fail
/// use apis_saltans_zcl::ota_upgrade::{ImageId, UpgradeEndStatus, UpgradeIncomplete};
/// let image = ImageId::new(0x1234, 0x5678, 1);
/// let request = UpgradeIncomplete::new(UpgradeEndStatus::Success, image);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum UpgradeIncompleteReason {
    /// The client aborted the upgrade process.
    Abort,

    /// The downloaded image failed validation.
    InvalidImage,

    /// Another image is required before the client can upgrade.
    RequireMoreImage,
}

impl UpgradeComplete {
    /// Construct a successful completion report without an unrestricted status field.
    #[must_use]
    pub const fn new(image: ImageId) -> Self {
        Self { image }
    }

    /// Return the downloaded image identifier.
    #[must_use]
    pub const fn image(self) -> ImageId {
        self.image
    }
}

impl UpgradeIncomplete {
    /// Construct a report whose reason cannot express successful completion.
    #[must_use]
    pub const fn new(reason: UpgradeIncompleteReason, image: ImageId) -> Self {
        Self { reason, image }
    }

    /// Return why the client cannot activate the image.
    #[must_use]
    pub const fn reason(self) -> UpgradeIncompleteReason {
        self.reason
    }

    /// Return the downloaded image identifier.
    #[must_use]
    pub const fn image(self) -> ImageId {
        self.image
    }
}

impl UpgradeEndRequest {
    /// Return the image identifier from either wire variant.
    #[must_use]
    pub const fn image(self) -> ImageId {
        match self {
            Self::Complete(request) => request.image(),
            Self::Incomplete(request) => request.image(),
        }
    }

    /// Return the status encoded by this variant.
    #[must_use]
    pub const fn status(self) -> UpgradeEndStatus {
        match self {
            Self::Complete(_) => UpgradeEndStatus::Success,
            Self::Incomplete(request) => match request.reason() {
                UpgradeIncompleteReason::Abort => UpgradeEndStatus::Abort,
                UpgradeIncompleteReason::InvalidImage => UpgradeEndStatus::InvalidImage,
                UpgradeIncompleteReason::RequireMoreImage => UpgradeEndStatus::RequireMoreImage,
            },
        }
    }
}
