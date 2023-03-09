use crate::constants::{KEY_SIZE, VALUE_SIZE};
use anyhow::Ok;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WALEntry<'a> {
    pub key_size: usize,
    pub value_size: usize,
    pub key: &'a [u8],
    pub value: &'a [u8],
}
