use std::io::{self, Read, Seek, SeekFrom};

/// Owned OTA image source that can be moved to a blocking worker.
pub(super) trait ImageSource
where
    Self: Read + Seek + Send,
{
}

impl<T> ImageSource for T where T: Read + Seek + Send {}

/// Extension methods for reading byte ranges from seekable readers.
pub(super) trait ReadRange
where
    Self: Read + Seek,
{
    /// Seek to `offset` and read exactly `length` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if `offset` cannot be represented by the reader, seeking fails, or the
    /// requested number of bytes cannot be read.
    fn read_range(&mut self, offset: usize, length: usize) -> io::Result<Box<[u8]>> {
        let mut data = vec![0; length].into_boxed_slice();
        let offset = u64::try_from(offset).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "OTA image offset does not fit the reader",
            )
        })?;
        self.seek(SeekFrom::Start(offset))?;
        self.read_exact(&mut data)?;
        Ok(data)
    }
}

impl<T> ReadRange for T where T: Read + Seek + ?Sized {}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::ReadRange;

    const SOURCE: &[u8] = &[0x10, 0x20, 0x30, 0x40];
    const RANGE_OFFSET: usize = 1;
    const RANGE_LENGTH: usize = 2;

    #[test]
    fn reads_an_exact_range_from_a_seekable_reader() {
        let mut source = Cursor::new(SOURCE);

        let range = source
            .read_range(RANGE_OFFSET, RANGE_LENGTH)
            .expect("test range is readable");

        assert_eq!(
            range.as_ref(),
            &SOURCE[RANGE_OFFSET..RANGE_OFFSET + RANGE_LENGTH]
        );
    }
}
