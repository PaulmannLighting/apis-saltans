/// Available ZCL command frame IDs.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum CommandFrameId {
    /// Read attributes command.
    ReadAttributes = 0x00,
    /// Read attributes response.
    ReadAttributesResponse = 0x01,
    /// Write attributes command.
    WriteAttributes = 0x02,
    /// Write attributes undivided command.
    WriteAttributesUndivided = 0x03,
    /// Write attributes response.
    WriteAttributesResponse = 0x04,
    /// Write attributes no response.
    WriteAttributesNoResponse = 0x05,
    /// Configure reporting command.
    ConfigureReporting = 0x06,
    /// Configure reporting response.
    ConfigureReportingResponse = 0x07,
    /// Read reporting configuration command.
    ReadReportingConfiguration = 0x08,
    /// Read reporting configuration response.
    ReadReportingConfigurationResponse = 0x09,
    /// Report attributes command.
    ReportAttributes = 0x0a,
    /// Default response.
    DefaultResponse = 0x0b,
    /// Discover attributes command.
    DiscoverAttributes = 0x0c,
    /// Discover attributes response.
    DiscoverAttributesResponse = 0x0d,
    /// Read attributes structured command.
    ReadAttributesStructured = 0x0e,
    /// Write attributes structured command.
    WriteAttributesStructured = 0x0f,
    /// Write attributes structured response.
    WriteAttributesStructuredResponse = 0x10,
    /// Discover commands received command.
    DiscoverCommandsReceived = 0x11,
    /// Discover commands received response.
    DiscoverCommandsReceivedResponse = 0x12,
    /// Discover commands generated command.
    DiscoverCommandsGenerated = 0x13,
    /// Discover commands generated response.
    DiscoverCommandsGeneratedResponse = 0x14,
    /// Discover attributes extended command.
    DiscoverAttributesExtended = 0x15,
    /// Discover attributes extended response.
    DiscoverAttributesExtendedResponse = 0x16,
}
