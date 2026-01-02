use crate::core::traits::BinaryProvider;
use crate::errors::{Result, UnifyError};
use object::File;
use std::fs;

pub struct BinaryLoader {
    #[allow(dead_code)]
    pub path: String,
    pub data: Vec<u8>,
}

impl BinaryLoader {
    pub fn new(path: &str) -> Result<Self> {
        let data = fs::read(path).map_err(|e| UnifyError::Io(e))?;
        Ok(Self {
            path: path.to_string(),
            data,
        })
    }
}

impl BinaryProvider for BinaryLoader {
    fn data(&self) -> &[u8] {
        &self.data
    }

    fn source(&self) -> &str {
        &self.path
    }
}
