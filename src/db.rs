use std::{fs::File, os::unix::prelude::FileExt, path::Path, str::from_utf8};

use anyhow::Ok;

pub struct Meta {
    value_sz: u8,
    value_pos: u64,
}

pub struct DB {
    db: File,
    write_at: u64,
}

impl DB {
    pub fn new() -> Result<Self, anyhow::Error> {
        let path = Path::new("our.db");
        let file = File::create(&path)?;
        return Ok(DB {
            db: file,
            write_at: 0,
        });
    }

    pub fn insert(&mut self, buf: Vec<u8>) -> anyhow::Result<Meta> {
        let bytes_written = self.db.write_at(buf.as_ref(), self.write_at)?;
        // TODO: make this atomic
        let new_offset = self.write_at + bytes_written as u64;
        Ok(Meta { value_sz: buf.len() as u8, value_pos: new_offset })
    }

    pub fn get(&self, meta: &Meta) -> anyhow::Result<String> {
        let mut buf = [0u8, meta.value_sz];
        let read_count = self.db.read_at(&mut buf, meta.value_pos)?;
        if read_count as u8 != meta.value_sz {
            println!("WTF");
            Err(anyhow::Error::msg("weird shit happened"))
        } else {
            Ok(from_utf8(&buf).unwrap().to_owned())
        }
    }
}
