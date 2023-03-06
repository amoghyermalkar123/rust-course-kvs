use crate::constants::{KEY_SIZE, VALUE_SIZE};
use crate::wal::{self, WALEntry};
use anyhow::Ok;
use bytes::{BufMut, Bytes, BytesMut};
use std::fs::OpenOptions;
use std::io::{SeekFrom, Write};
use std::os::unix::prelude::PermissionsExt;
use std::{collections::HashMap, fs::File, os::unix::prelude::FileExt, path::Path, str::from_utf8};

pub struct Meta {
    value_pos: u64,
    value_sz: u8,
}

pub struct DB {
    db: File,
    write_at: u64,
}

impl DB {
    pub fn load_indexes(&self, index_map: HashMap<String, Meta>) -> anyhow::Result<()> {
        let mut prefix_buffer = [0u8; KEY_SIZE + VALUE_SIZE];
        let read_count = self.db.read_at(&mut prefix_buffer, 0)?;
        let key_sz = prefix_buffer[..KEY_SIZE].len();
        let val_sz = prefix_buffer[KEY_SIZE..].len();
        let value = from_utf8(&prefix_buffer[VALUE_SIZE..])?;
        println!("k_sz, v_sz and value : {} {} {}", key_sz, val_sz, value);
        Ok(())
    }

    pub fn new() -> Result<Self, anyhow::Error> {
        let path = Path::new("this.db");
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;
        return Ok(DB {
            db: file,
            write_at: 0,
        });
    }

    pub fn insert(&mut self, wal_record: WALEntry) -> anyhow::Result<Meta> {
        let value_sz = wal_record.value_size as u8;
        let mut prefix_buffer = BytesMut::with_capacity(KEY_SIZE + VALUE_SIZE);
        prefix_buffer.put_u32(wal_record.key_size as u32);
        prefix_buffer.put_u64(wal_record.value_size as u64);
        let mut encoded_buffer = BytesMut::new();
        encoded_buffer.put_slice(&prefix_buffer);
        encoded_buffer.put_slice(wal_record.key);
        encoded_buffer.put_slice(wal_record.value);
        let bytes_written = self.db.write_at(&encoded_buffer, self.write_at)?;
        println!("bytes written {}", bytes_written);
        // TODO: make this atomic
        let old_offset = self.write_at;
        self.write_at = self.write_at + bytes_written as u64;
        let meta = Meta {
            value_pos: old_offset,
            value_sz,
        };
        Ok(meta)
    }

    pub fn get(&self, meta: &Meta) -> anyhow::Result<String> {
        let mut prefix_buffer = [0u8, KEY_SIZE as u8 + VALUE_SIZE as u8];
        let read_count = self.db.read_at(&mut prefix_buffer, meta.value_pos)?;
        let key_sz = prefix_buffer[..KEY_SIZE].len();
        let val_sz = prefix_buffer[KEY_SIZE..].len();
        let value = from_utf8(&prefix_buffer[VALUE_SIZE..])?;
        println!("k_sz, v_sz and value : {} {} {}", key_sz, val_sz, value);
        Ok(value.to_owned())
    }
}
