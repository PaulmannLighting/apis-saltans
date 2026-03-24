use crate::{Frame, Header};

/// Trait to create ZCL frame headers.
///
/// # Safety
///
/// The implementor must ensure that the header fields are consistent with the underlying payload.
#[expect(unsafe_code)]
pub unsafe trait HeaderFactory {
    /// Generate the header.
    fn header(&self, seq: u8) -> Header;

    /// Convert the header factory implementor into an appropriate ZCL frame with the given sequence number.
    fn frame(self, seq: u8) -> Frame<Self>
    where
        Self: Sized,
    {
        #[expect(unsafe_code)]
        // SAFETY: We crate the header with the trait method as defined for this type.
        // Since this trait is marked as unsafe itself, it is up to the implementor to uphold
        // the invariant that the header matches the given payload.
        unsafe {
            Frame::new_unchecked(self.header(seq), self)
        }
    }
}
