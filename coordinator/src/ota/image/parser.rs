use std::io::{Cursor, ErrorKind, SeekFrom};

use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt};

use super::header::HeaderBuilder;
use super::{BaseHeaderBytes, Header, Image, ParseImageError};

/// Extension trait for asynchronously parsing a seekable Zigbee OTA image source.
///
/// Implemented for [`File`] and compatible [`Cursor`] types. Custom asynchronous readers
/// can implement this trait and override individual parsing stages.
pub trait ParseImage
where
    Self: AsyncRead + AsyncSeek + Unpin + Send + Sized + 'static,
{
    /// Parse and validate this complete Zigbee OTA image source.
    ///
    /// The source must contain exactly one OTA image starting at offset zero. Parsing retains the
    /// source for later payload reads while keeping only the parsed header resident in memory.
    ///
    /// # Errors
    ///
    /// Returns [`ParseImageError`] if the file header is malformed, unsupported, or inconsistent
    /// with the source length, or if reading or seeking fails.
    fn parse(mut self) -> impl Future<Output = Result<Image, ParseImageError>> + Send {
        async move {
            self.seek_to_image_start().await?;
            let base_header = self.read_base_header().await?;
            let header = HeaderBuilder::parse(base_header)?;
            let optional_header = self
                .read_optional_header(header.optional_header_length())
                .await?;
            let image_length = self.read_image_length().await?;
            let header = header.finish(&optional_header, image_length)?;
            self.seek_to_payload(&header).await?;
            Ok(Image::new(header, Box::new(self)))
        }
    }

    /// Position the source at the start of the OTA image.
    ///
    /// # Errors
    ///
    /// Returns [`ParseImageError::Io`] if seeking fails.
    fn seek_to_image_start(&mut self) -> impl Future<Output = Result<(), ParseImageError>> + Send {
        async move {
            self.seek(SeekFrom::Start(0)).await?;
            Ok(())
        }
    }

    /// Read the mandatory fixed-width OTA header.
    ///
    /// # Errors
    ///
    /// Returns [`ParseImageError::TruncatedHeader`] when the fixed header is incomplete, or
    /// [`ParseImageError::Io`] for another read failure.
    fn read_base_header(
        &mut self,
    ) -> impl Future<Output = Result<BaseHeaderBytes, ParseImageError>> + Send {
        async move {
            let mut header = [0; size_of::<BaseHeaderBytes>()];
            self.read_header_exact(&mut header).await?;
            Ok(header)
        }
    }

    /// Read `length` bytes containing the optional OTA header fields.
    ///
    /// # Errors
    ///
    /// Returns [`ParseImageError::TruncatedHeader`] when the optional fields are incomplete, or
    /// [`ParseImageError::Io`] for another read failure.
    fn read_optional_header(
        &mut self,
        length: usize,
    ) -> impl Future<Output = Result<Box<[u8]>, ParseImageError>> + Send {
        async move {
            let mut header = vec![0; length].into_boxed_slice();
            self.read_header_exact(&mut header).await?;
            Ok(header)
        }
    }

    /// Fill a header buffer, mapping an unexpected end of input to a truncated-header error.
    ///
    /// # Errors
    ///
    /// Returns [`ParseImageError::TruncatedHeader`] when `buffer` cannot be filled, or
    /// [`ParseImageError::Io`] for another read failure.
    fn read_header_exact(
        &mut self,
        buffer: &mut [u8],
    ) -> impl Future<Output = Result<(), ParseImageError>> + Send {
        async move {
            self.read_exact(buffer).await.map(|_| ()).map_err(|error| {
                if error.kind() == ErrorKind::UnexpectedEof {
                    ParseImageError::TruncatedHeader
                } else {
                    error.into()
                }
            })
        }
    }

    /// Determine the complete OTA image length without reading its payload.
    ///
    /// # Errors
    ///
    /// Returns [`ParseImageError::ImageTooLarge`] if the source length does not fit this platform,
    /// or [`ParseImageError::Io`] if seeking fails.
    fn read_image_length(&mut self) -> impl Future<Output = Result<usize, ParseImageError>> + Send {
        async move {
            usize::try_from(self.seek(SeekFrom::End(0)).await?)
                .map_err(|_| ParseImageError::ImageTooLarge)
        }
    }

    /// Position the source at the first payload byte after parsing.
    ///
    /// # Errors
    ///
    /// Returns [`ParseImageError::Io`] if seeking fails.
    fn seek_to_payload(
        &mut self,
        header: &Header,
    ) -> impl Future<Output = Result<(), ParseImageError>> + Send {
        async move {
            self.seek(SeekFrom::Start(u64::from(header.header_length())))
                .await?;
            Ok(())
        }
    }
}

impl ParseImage for File {}

impl<T> ParseImage for Cursor<T> where T: AsRef<[u8]> + Unpin + Send + 'static {}
