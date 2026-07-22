use std::fs::{self, File, OpenOptions};
use std::io::{Cursor, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use bytes::{BufMut, Bytes, BytesMut};

use super::{FieldControl, ParseImage, ParseImageError};

const MANUFACTURER_CODE: u16 = 0x1234;
const IMAGE_TYPE: u16 = 0x5678;
const FILE_VERSION: u32 = 0x0102_0304;
const STACK_VERSION: u16 = 0x0002;
const OTA_FILE_IDENTIFIER: u32 = 0x0bee_f11e;
const SUPPORTED_HEADER_VERSION: u16 = 0x0100;
const BASE_HEADER_LENGTH: usize = 56;
const HEADER_STRING_LENGTH: usize = 32;
const HEADER_STRING: &str = "Test OTA image";
const FIELD_CONTROL_OFFSET: usize = 8;
const HEADER_STRING_OFFSET: usize = 20;
const HARDWARE_VERSIONS_LENGTH: usize = 4;
const UNSUPPORTED_FIELD_CONTROL: u16 = 0x0008;
const PAYLOAD: &[u8] = &[1, 2, 3, 4];
const TEST_FILE_PREFIX: &str = "apis-saltans-ota-image";

static NEXT_TEST_FILE_ID: AtomicU64 = AtomicU64::new(0);

struct TemporaryImageFile {
    path: PathBuf,
    file: Option<File>,
}

impl TemporaryImageFile {
    fn new(bytes: &[u8]) -> Self {
        let id = NEXT_TEST_FILE_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "{TEST_FILE_PREFIX}-{}-{id}.ota",
            std::process::id()
        ));
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
            .expect("temporary OTA image can be created");
        file.write_all(bytes)
            .expect("temporary OTA image can be written");
        Self {
            path,
            file: Some(file),
        }
    }

    fn take(&mut self) -> File {
        self.file.take().expect("temporary OTA image is available")
    }
}

impl Drop for TemporaryImageFile {
    fn drop(&mut self) {
        drop(self.file.take());
        drop(fs::remove_file(&self.path));
    }
}

#[test]
fn parses_complete_file() {
    let mut source = TemporaryImageFile::new(&image_bytes(FieldControl::empty(), None));
    let image = source.take().parse().expect("valid OTA image");

    assert_eq!(image.id().manufacturer_code(), MANUFACTURER_CODE);
    assert_eq!(image.id().image_type(), IMAGE_TYPE);
    assert_eq!(image.id().file_version(), FILE_VERSION);
    assert_eq!(image.zigbee_stack_version(), STACK_VERSION);
    assert_eq!(image.header_string().as_str(), HEADER_STRING);
    assert_eq!(
        image.header_length(),
        u16::try_from(BASE_HEADER_LENGTH).expect("fixed header length fits u16")
    );
    assert_eq!(image.header().as_bytes().len(), BASE_HEADER_LENGTH);
    assert!(image.header().field_control().is_empty());
    assert_eq!(image.len(), BASE_HEADER_LENGTH + PAYLOAD.len());
}

#[test]
fn reads_header_and_payload_ranges_from_the_source() {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("test runtime can be built")
        .block_on(async {
            let bytes = image_bytes(FieldControl::empty(), None);
            let transfer = Cursor::new(bytes.clone())
                .parse()
                .expect("valid OTA image")
                .into_transfer();
            let crossing_offset = BASE_HEADER_LENGTH - 2;
            let crossing_length = 4;

            assert_eq!(
                transfer
                    .read_range(BASE_HEADER_LENGTH, PAYLOAD.len())
                    .await
                    .expect("payload is readable")
                    .as_ref(),
                PAYLOAD
            );
            assert_eq!(
                transfer
                    .read_range(crossing_offset, crossing_length)
                    .await
                    .expect("header-to-payload range is readable")
                    .as_ref(),
                &bytes[crossing_offset..crossing_offset + crossing_length]
            );
            assert_eq!(
                transfer
                    .read_range(0, BASE_HEADER_LENGTH)
                    .await
                    .expect("an earlier range remains readable")
                    .as_ref(),
                &bytes[..BASE_HEADER_LENGTH]
            );
        });
}

#[test]
fn validates_hardware_range() {
    let image = Cursor::new(image_bytes(FieldControl::HARDWARE_VERSIONS, Some((10, 20))))
        .parse()
        .expect("valid OTA image");

    assert_eq!(
        image.header().field_control(),
        FieldControl::HARDWARE_VERSIONS
    );
    assert!(image.header().supports_hardware(Some(10)));
    assert!(image.header().supports_hardware(Some(20)));
    assert!(!image.header().supports_hardware(Some(9)));
    assert!(!image.header().supports_hardware(None));
}

#[test]
fn rejects_inconsistent_total_size() {
    let mut bytes = image_bytes(FieldControl::empty(), None).to_vec();
    bytes.push(0);

    assert!(matches!(
        Cursor::new(bytes).parse(),
        Err(ParseImageError::InvalidImageSize { .. })
    ));
}

#[test]
fn rejects_unknown_field_control_flags() {
    let mut bytes = image_bytes(FieldControl::empty(), None).to_vec();
    bytes[FIELD_CONTROL_OFFSET..FIELD_CONTROL_OFFSET + size_of::<u16>()]
        .copy_from_slice(&UNSUPPORTED_FIELD_CONTROL.to_le_bytes());

    assert!(matches!(
        Cursor::new(bytes).parse(),
        Err(ParseImageError::UnsupportedFieldControl(bits))
            if bits == UNSUPPORTED_FIELD_CONTROL
    ));
}

#[test]
fn rejects_non_ascii_header_string() {
    let mut bytes = image_bytes(FieldControl::empty(), None).to_vec();
    bytes[HEADER_STRING_OFFSET] = u8::MAX;

    assert!(matches!(
        Cursor::new(bytes).parse(),
        Err(ParseImageError::InvalidHeaderString)
    ));
}

#[test]
fn rejects_unterminated_header_string() {
    let mut bytes = image_bytes(FieldControl::empty(), None).to_vec();
    bytes[HEADER_STRING_OFFSET..HEADER_STRING_OFFSET + HEADER_STRING_LENGTH].fill(b'A');

    assert!(matches!(
        Cursor::new(bytes).parse(),
        Err(ParseImageError::InvalidHeaderString)
    ));
}

fn image_bytes(field_control: FieldControl, hardware: Option<(u16, u16)>) -> Bytes {
    let header_length = BASE_HEADER_LENGTH + hardware.map_or(0, |_| HARDWARE_VERSIONS_LENGTH);
    let total_length = header_length + PAYLOAD.len();
    let mut bytes = BytesMut::with_capacity(total_length);
    bytes.put_u32_le(OTA_FILE_IDENTIFIER);
    bytes.put_u16_le(SUPPORTED_HEADER_VERSION);
    bytes.put_u16_le(u16::try_from(header_length).expect("test header length fits u16"));
    bytes.put_u16_le(field_control.bits());
    bytes.put_u16_le(MANUFACTURER_CODE);
    bytes.put_u16_le(IMAGE_TYPE);
    bytes.put_u32_le(FILE_VERSION);
    bytes.put_u16_le(STACK_VERSION);
    bytes.extend_from_slice(HEADER_STRING.as_bytes());
    bytes.extend_from_slice(&[0; HEADER_STRING_LENGTH - HEADER_STRING.len()]);
    bytes.put_u32_le(u32::try_from(total_length).expect("test image length fits u32"));
    if let Some((minimum, maximum)) = hardware {
        bytes.put_u16_le(minimum);
        bytes.put_u16_le(maximum);
    }
    bytes.extend_from_slice(PAYLOAD);
    bytes.freeze()
}
