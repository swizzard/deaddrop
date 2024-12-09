use sha2::{Digest, Sha256};
use sled::Tree;

pub struct RawKey<'a> {
    key: &'a [u8],
    pass: &'a [u8],
}

impl<'a> RawKey<'a> {
    pub fn new(key: &'a [u8], pass: &'a [u8]) -> Self {
        Self { key, pass }
    }
    fn to_key(&self) -> Vec<u8> {
        let mut h = Sha256::new();
        h.update(self.key);
        h.update(self.pass);
        h.finalize().to_vec()
    }
}

pub fn insert(db: &Tree, key: &RawKey, val: &[u8]) -> sled::Result<()> {
    db.insert(key.to_key(), val)?;
    Ok(())
}

pub fn get(db: &Tree, key: &RawKey) -> sled::Result<Option<Vec<u8>>> {
    Ok(db.get(key.to_key())?.map(|val| val.to_vec()))
}

pub fn get_once(db: &Tree, key: &RawKey) -> sled::Result<Option<Vec<u8>>> {
    Ok(db.remove(key.to_key())?.map(|val| val.to_vec()))
}
