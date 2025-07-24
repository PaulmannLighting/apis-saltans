/// Available ZCL status codes.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Status {
    /// Indicates the command was successful.
    Success = 0x00,
    /// Indicates the command failed.
    Failure = 0x01,
    /// Indicates the command is not authorized.
    NotAuthorized = 0x7e,
    /// A reserved field was not zero.
    ReservedFieldNotZero = 0x7f,
    /// Indicates the command was malformed.
    MalformedCommand = 0x80,
    /// Indicates the cluster command is not supported.
    UnsupportedClusterCommand = 0x81,
    /// Indicates the general command is not supported.
    UnsupportedGeneralCommand = 0x82,
    /// Indicates the manufacturer-specific cluster command is not supported.
    UnsupportedManufacturerClusterCommand = 0x83,
    /// Indicates the manufacturer-specific general command is not supported.
    UnsupportedManufacturerGeneralCommand = 0x84,
    /// Indicates the field in the command is invalid.
    InvalidField = 0x85,
    /// Indicates the attribute is unsupported.
    UnsupportedAttribute = 0x86,
    /// Indicates the value of the attribute is invalid.
    InvalidValue = 0x87,
    /// Indicates the attribute is read-only.
    ReadOnly = 0x88,
    /// Indicates there is insufficient space to perform the operation.
    InsufficientSpace = 0x89,
    /// Indicates a duplicate entry exists.
    DuplicateExists = 0x8a,
    /// Indicates the requested entry was not found.
    NotFound = 0x8b,
    /// Indicates the attribute is unreportable.
    UnreportableAttribute = 0x8c,
    /// Indicates the data type of the attribute is invalid.
    InvalidDataType = 0x8d,
    /// Indicates the selector is invalid.
    InvalidSelector = 0x8e,
    /// Indicates the attribute is write-only.
    WriteOnly = 0x8f,
    /// Indicates the startup state is inconsistent.
    InconsistentStartupState = 0x90,
    /// Indicates the command was defined out of band.
    DefinedOutOfBand = 0x91,
    /// Indicates the command is inconsistent.
    Inconsistent = 0x92,
    /// Indicates the action is denied.
    ActionDenied = 0x93,
    /// Indicates the command timed out.
    Timeout = 0x94,
    /// Indicates the command was aborted.
    Abort = 0x95,
    /// Indicates the image is invalid.
    InvalidImage = 0x96,
    /// Indicates the device is waiting for data.
    WaitForData = 0x97,
    /// Indicates no image is available.
    NoImageAvailable = 0x98,
    /// Indicates more image data is required.
    RequireMoreImage = 0x99,
    /// Indicates the notification is pending.
    NotificationPending = 0x9a,
    /// Indicates a hardware failure.
    HardwareFailure = 0xc0,
    /// Indicates a software failure.
    SoftwareFailure = 0xc1,
    /// Indicates a calibration error.
    CalibrationError = 0xc2,
    /// Indicates that the cluster is unsupported.
    UnsupportedCluster = 0xc3,
}
