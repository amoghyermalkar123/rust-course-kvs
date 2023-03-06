use crate::constants::{KEY_SIZE, VALUE_SIZE};
use anyhow::Ok;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WALEntry<'a> {
    pub key_size: usize,
    pub value_size: usize,
    pub key: &'a [u8],
    pub value: &'a [u8],
}

impl<'a> WALEntry<'a> {
    pub fn encode_entry(entry: WALEntry) -> anyhow::Result<Vec<u8>> {
        if entry.key_size > KEY_SIZE || entry.value_size > VALUE_SIZE {
            println!("szs {} {}", entry.key_size, entry.value_size);
            Err(anyhow::Error::msg("incorrect key/ value size"))
        } else {
            let s = bson::to_vec(&entry)?;
            Ok(s)
        }
    }
}
