mod error;
mod read_bytes;

use std::{fs::File, io::Write};

use error::Error;
use read_bytes::ReadBytesExt;

pub struct NSPFile;

#[derive(Debug)]
pub struct FileEntryTable {
    _offset: u64,
    size: u64,
    _name_offset: u32,
}

impl NSPFile {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let magic = reader.read_bytes(4)?;
        assert_eq!(magic, b"PFS0");

        // Number of files in archive
        let file_count = reader.read_u32_le()?;
        println!("Partition contains {} files.", file_count);

        // Size of the string table
        let string_table_size = reader.read_u32_le()? as usize;
        let _ = reader.read_u32_le()?; // padding

        let mut file_entry_tables = Vec::new();
        for _ in 0..file_count {
            file_entry_tables.push(FileEntryTable {
                _offset: reader.read_u64_le()?,
                size: reader.read_u64_le()?,
                _name_offset: reader.read_u32_le()?,
            });

            let _ = reader.read_u32_le()?; // padding
        }

        let string_table = reader.read_bytes(string_table_size)?;
        let mut string_table = string_table.as_slice();

        for i in 0..file_count {
            let filename = string_table.read_string_utf8()?;
            println!("filename {:?}", filename);
            let file = reader.read_bytes(file_entry_tables[i as usize].size as usize)?;

            let mut buffer = File::create(filename)?;
            buffer.write(&file)?;
        }

        Ok(NSPFile)
    }
}
