use anyhow::Ok;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WALEntry<'a> {
    pub key: &'a [u8],
    pub value: &'a [u8],
}

impl<'a> WALEntry<'a> {
    pub fn entry(key: &'a [u8], val: &'a [u8]) -> anyhow::Result<WALEntry<'a>> {
        Ok(WALEntry {
            key,
            value: val,
        })
    }

    pub fn encode_entry(entry: WALEntry) -> anyhow::Result<Vec<u8>> {
        let s = bson::to_vec(&entry)?;
        Ok(s)
    }
}
