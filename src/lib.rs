mod error;
mod read_bytes;

use error::Error;
use read_bytes::ReadBytesExt;

struct NSPFile;

impl NSPFile {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let magic = reader.read_bytes(4)?;
        assert_eq!(magic, b"PFS0");

        let file_count = reader.read_u32_le()?;
        let string_table_size = reader.read_u32_le()?;
        let padding = reader.read_u32_le()?;

        Ok(NSPFile)
    }
}
