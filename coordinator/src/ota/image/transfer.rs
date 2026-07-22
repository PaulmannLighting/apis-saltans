use std::io::{self, Read, Seek, SeekFrom};

use tokio::sync::{mpsc, oneshot};
use zb_core::IeeeAddress;
use zb_zcl::ota_upgrade::ImageId;

use super::Header;
use super::source::ImageSource;

const TRANSFER_CHANNEL_SIZE: usize = 8;

#[derive(Clone, Debug)]
pub struct ImageTransfer {
    header: Header,
    requests: mpsc::Sender<ReadRequest>,
}

struct ImageTransferTask {
    source: Option<Box<dyn ImageSource>>,
    inbound: mpsc::Receiver<ReadRequest>,
}

struct ReadRequest {
    offset: usize,
    length: usize,
    response: oneshot::Sender<io::Result<Box<[u8]>>>,
}

impl ImageTransfer {
    pub(super) fn spawn(header: Header, source: Box<dyn ImageSource>) -> Self {
        let (requests, inbound) = mpsc::channel(TRANSFER_CHANNEL_SIZE);
        drop(tokio::spawn(
            ImageTransferTask {
                source: Some(source),
                inbound,
            }
            .run(),
        ));
        Self { header, requests }
    }

    pub const fn id(&self) -> ImageId {
        self.header.id()
    }

    pub const fn len(&self) -> usize {
        self.header.image_length()
    }

    pub const fn zigbee_stack_version(&self) -> u16 {
        self.header.zigbee_stack_version()
    }

    pub const fn upgrade_file_destination(&self) -> Option<IeeeAddress> {
        self.header.upgrade_file_destination()
    }

    pub const fn image_size(&self) -> u32 {
        self.header.total_image_size()
    }

    pub fn supports_hardware(&self, hardware_version: Option<u16>) -> bool {
        self.header.supports_hardware(hardware_version)
    }

    pub async fn read_range(&self, offset: usize, length: usize) -> io::Result<Box<[u8]>> {
        let end = offset
            .checked_add(length)
            .filter(|end| *end <= self.len())
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "invalid OTA image range")
            })?;
        let header_length = usize::from(self.header.header_length());
        let mut result = Vec::with_capacity(length);
        if offset < header_length {
            let header_end = end.min(header_length);
            result.extend_from_slice(&self.header.as_bytes()[offset..header_end]);
        }
        if end > header_length {
            let source_offset = offset.max(header_length);
            let source_length = end - source_offset;
            let (response, received) = oneshot::channel();
            self.requests
                .send(ReadRequest {
                    offset: source_offset,
                    length: source_length,
                    response,
                })
                .await
                .map_err(|_| io::Error::other("OTA image transfer task has stopped"))?;
            let payload = received
                .await
                .map_err(|_| io::Error::other("OTA image transfer response was dropped"))??;
            result.extend_from_slice(&payload);
        }
        Ok(result.into_boxed_slice())
    }
}

impl ImageTransferTask {
    async fn run(mut self) {
        while let Some(request) = self.inbound.recv().await {
            let Some(source) = self.source.take() else {
                return;
            };
            let ReadRequest {
                offset,
                length,
                response,
            } = request;
            match tokio::task::spawn_blocking(move || Self::read_range(source, offset, length))
                .await
            {
                Ok((source, result)) => {
                    self.source = Some(source);
                    drop(response.send(result));
                }
                Err(error) => {
                    drop(response.send(Err(io::Error::other(error))));
                    return;
                }
            }
        }
    }

    fn read_range(
        mut source: Box<dyn ImageSource>,
        offset: usize,
        length: usize,
    ) -> (Box<dyn ImageSource>, io::Result<Box<[u8]>>) {
        let result = read_source_range(source.as_mut(), offset, length);
        (source, result)
    }
}

fn read_source_range<R>(source: &mut R, offset: usize, length: usize) -> io::Result<Box<[u8]>>
where
    R: Read + Seek + ?Sized,
{
    let mut data = vec![0; length].into_boxed_slice();
    let offset = u64::try_from(offset).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "OTA image offset does not fit the reader",
        )
    })?;
    source.seek(SeekFrom::Start(offset))?;
    source.read_exact(&mut data)?;
    Ok(data)
}
