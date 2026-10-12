use std::io::{self, Error, ErrorKind, SeekFrom};

use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt};

/// Owned asynchronous OTA image source.
pub(super) trait ImageSource
where
    Self: AsyncRead + AsyncSeek + Unpin + Send,
{
}

impl<T> ImageSource for T where T: AsyncRead + AsyncSeek + Unpin + Send {}

/// Extension methods for reading byte ranges from seekable readers.
pub(super) trait ReadRange
where
    Self: AsyncRead + AsyncSeek + Unpin + Send,
{
    /// Seek to `offset` and read exactly `length` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if `offset` cannot be represented by the reader, seeking fails, or the
    /// requested number of bytes cannot be read.
    fn read_range(
        &mut self,
        offset: usize,
        length: usize,
    ) -> impl Future<Output = io::Result<Box<[u8]>>> + Send {
        async move {
            let mut data = vec![0; length].into_boxed_slice();
            let offset = u64::try_from(offset).map_err(|_| {
                Error::new(
                    ErrorKind::InvalidInput,
                    "OTA image offset does not fit the reader",
                )
            })?;
            self.seek(SeekFrom::Start(offset)).await?;
            self.read_exact(&mut data).await?;
            Ok(data)
        }
    }
}

impl<T> ReadRange for T where T: AsyncRead + AsyncSeek + Unpin + Send + ?Sized {}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use tokio::runtime::Runtime;

    use super::ReadRange;

    const SOURCE: &[u8] = &[0x10, 0x20, 0x30, 0x40];
    const RANGE_OFFSET: usize = 1;
    const RANGE_LENGTH: usize = 2;

    #[test]
    fn reads_an_exact_range_from_a_seekable_reader() {
        Runtime::new().unwrap().block_on(async {
            let mut source = Cursor::new(SOURCE);

            let range = source
                .read_range(RANGE_OFFSET, RANGE_LENGTH)
                .await
                .expect("test range is readable");

            assert_eq!(range.as_ref(), &[0x20, 0x30]);
        });
    }
}
