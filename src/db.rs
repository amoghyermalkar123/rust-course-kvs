use crate::constants::{KEY_SIZE, VALUE_SIZE};
use crate::wal::{self, WALEntry};
use anyhow::Ok;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use bytes::{BufMut, Bytes, BytesMut};
use std::fs::OpenOptions;
use std::os::unix::prelude::PermissionsExt;
use std::str::FromStr;
use std::{any, fs};
use std::{collections::HashMap, fs::File, os::unix::prelude::FileExt, path::Path, str::from_utf8};

pub struct Meta {
    value_pos: usize,
    value_sz: u64,
}

pub struct DB {
    db: File,
    write_at: u64,
}

impl DB {
    pub fn load_indexes(
        &self,
        mut index_map: HashMap<String, Meta>,
    ) -> anyhow::Result<HashMap<String, Meta>> {
        let mut offset: usize = 0;
        let file_size = fs::metadata("this.db")?.len();

        loop {
            let mut prefix_buffer = [0u8; KEY_SIZE + VALUE_SIZE];
            self.db.read_at(&mut prefix_buffer, offset as u64)?;
            if offset as u64 == file_size {
                // EOF
                break;
            }

            let mut key_sz = &prefix_buffer[..KEY_SIZE];
            let act_key_size = key_sz.read_u32::<BigEndian>()?;

            let mut val_sz = &prefix_buffer[KEY_SIZE..];
            let act_val_size = val_sz.read_u64::<BigEndian>()?;

            let buf_len = KEY_SIZE + VALUE_SIZE + (act_key_size as u64 + act_val_size) as usize;
            let mut complete_buf = vec![0u8; buf_len];
            let old_offset = offset;
            offset = offset + self.db.read_at(&mut complete_buf, offset as u64)?;
            let complete_buf = complete_buf.as_slice();

            let prefix_len = KEY_SIZE + VALUE_SIZE;
            let till_key = KEY_SIZE + VALUE_SIZE + act_key_size as usize;
            let key = from_utf8(&complete_buf[prefix_len..till_key])?;

            index_map.insert(
                String::from_str(key)?,
                Meta {
                    value_pos: old_offset + till_key,
                    value_sz: act_val_size,
                },
            );
        }

        Ok(index_map)
    }

    pub fn new() -> Result<Self, anyhow::Error> {
        let path = Path::new("this.db");
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        let write_at = fs::metadata(path)?.len();
        return Ok(DB { db: file, write_at });
    }

    // TODO: make this atomic
    pub fn insert(&mut self, wal_record: WALEntry) -> anyhow::Result<Meta> {
        let mut pref_buf = vec![];
        println!("ksz vsz: {} {}", wal_record.key_size, wal_record.value_size);
        pref_buf.write_u32::<BigEndian>(wal_record.key_size as u32)?;
        pref_buf.write_u64::<BigEndian>(wal_record.value_size as u64)?;

        let mut encoded_buffer = BytesMut::new();
        encoded_buffer.put_slice(&pref_buf);
        encoded_buffer.put_slice(wal_record.key);
        encoded_buffer.put_slice(wal_record.value);
        println!("b:{:?}", encoded_buffer.len());
        let bytes_written = self.db.write_at(&encoded_buffer, self.write_at)?;

        println!(
            "wrote at {:?}, rel val pos at {}",
            self.write_at,
            KEY_SIZE + VALUE_SIZE + wal_record.key_size
        );

        self.write_at = self.write_at + bytes_written as u64;
        let till_key = KEY_SIZE + VALUE_SIZE + wal_record.key_size;
        let meta = Meta {
            value_pos: till_key,
            value_sz: wal_record.value_size as u64,
        };

        Ok(meta)
    }

    pub fn get(&self, meta: &Meta) -> anyhow::Result<String> {
        let mut value_buf = vec![0u8; meta.value_sz as usize];
        println!("read at:{}", meta.value_pos);
        let read_count = self.db.read_at(&mut value_buf, meta.value_pos as u64)?;
        if read_count != meta.value_sz as usize {
            Err(anyhow::Error::msg("something went weirdly wrong"))
        } else {
            let value = from_utf8(&value_buf)?;
            Ok(value.to_owned())
        }
    }
}
