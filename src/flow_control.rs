pub enum FlowControl {
    /// No flow control
    OutNone,
    /// XOn / XOff (software) flow control
    OutXOnOff,
    /// RTS / CTS (hardware) flow control
    OutRtsCts,
}
