use crate::constants::{KEY_SIZE, VALUE_SIZE};
use crate::wal::WALEntry;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use bytes::{BufMut, BytesMut};
use std::fs;
use std::fs::OpenOptions;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, fs::File, os::unix::prelude::FileExt, path::Path, str::from_utf8};

pub struct Meta {
    file_id: String,
    value_pos: usize,
    value_sz: u64,
}

pub struct DB {
    kv: HashMap<String, Meta>,
    db: Arc<Mutex<File>>,
    write_at: u64,
}

impl DB {
    pub fn load_indexes(&mut self) -> anyhow::Result<()> {
        let mut offset: usize = 0;
        let file_size = fs::metadata("this.db")?.len();

        loop {
            let mut prefix_buffer = [0u8; KEY_SIZE + VALUE_SIZE];
            self.db.lock().unwrap().read_at(&mut prefix_buffer, offset as u64)?;
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
            offset = offset + self.db.lock().unwrap().read_at(&mut complete_buf, offset as u64)?;
            let complete_buf = complete_buf.as_slice();

            let prefix_len = KEY_SIZE + VALUE_SIZE;
            let till_key = KEY_SIZE + VALUE_SIZE + act_key_size as usize;
            let key = from_utf8(&complete_buf[prefix_len..till_key])?;

            self.kv.insert(
                String::from_str(key)?,
                Meta {
                    file_id: "".to_string(),
                    value_pos: old_offset + till_key,
                    value_sz: act_val_size,
                },
            );
        }

        Ok(())
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
        let kv = HashMap::new();
        return Ok(DB {
            kv,
            db: Arc::new(Mutex::new(file)),
            write_at,
        });
    }

    // TODO: make this atomic
    pub fn insert(&mut self, key: String, val: String) -> anyhow::Result<()> {
        let wal_record = WALEntry {
            key_size: key.as_bytes().len(),
            value_size: val.as_bytes().len(),
            key: key.as_bytes(),
            value: val.as_bytes(),
        };

        if let Some(mut val) = self.kv.get_mut(&key) {
            //
        }

        let mut encoded_buffer = BytesMut::new();
        let mut pref_buf = vec![];
        pref_buf.write_u32::<BigEndian>(wal_record.key_size as u32)?;
        pref_buf.write_u64::<BigEndian>(wal_record.value_size as u64)?;

        encoded_buffer.put_slice(&pref_buf);
        encoded_buffer.put_slice(wal_record.key);
        encoded_buffer.put_slice(wal_record.value);
        let bytes_written = self.db.lock().unwrap().write_at(&encoded_buffer, self.write_at)?;

        self.write_at = self.write_at + bytes_written as u64;
        let till_key = KEY_SIZE + VALUE_SIZE + wal_record.key_size;
        let meta = Meta {
            file_id: "".to_string(),
            value_pos: till_key,
            value_sz: wal_record.value_size as u64,
        };
        println!("meta {:?}", meta.value_pos,);
        self.kv.insert(key, meta);
        Ok(())
    }

    pub fn del(&mut self, key: String) -> anyhow::Result<()> {
        if let Some(val) = self.kv.get_mut(&key) {
            let new_val = vec![0u8; val.value_sz as usize];
            let mut encoded_buffer = BytesMut::new();
            encoded_buffer.put_slice(&new_val);
            println!("writing at {}, with {:?}", val.value_pos, encoded_buffer);
            self.db.lock().unwrap().write_at(&encoded_buffer, val.value_pos as u64)?;
            self.kv.remove(&key);
            return Ok(());
        } else {
            Err(anyhow::Error::msg("del failed"))
        }
    }

    pub fn get(&self, key: String) -> anyhow::Result<String> {
        if let Some(meta) = self.kv.get(&key) {
            let mut value_buf = vec![0u8; meta.value_sz as usize];
            println!("read at:{}", meta.value_pos);
            let read_count = self.db.lock().unwrap().read_at(&mut value_buf, meta.value_pos as u64)?;
            if read_count != meta.value_sz as usize {
                Err(anyhow::Error::msg("something went weirdly wrong"))
            } else {
                let value = from_utf8(&value_buf)?;
                Ok(value.to_owned())
            }
        } else {
            Err(anyhow::Error::msg("couldn't find key"))
        }
    }
}
