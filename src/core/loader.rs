use crate::errors::{Result, UnifyError};
use object::{File, Object};
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

    pub fn parse(&self) -> Result<File<'_>> {
        File::parse(&*self.data)
            .map_err(|e| UnifyError::ParseError(format!("Failed to parse binary: {}", e)))
    }

    #[allow(dead_code)]
    pub fn get_format_name(&self) -> Result<String> {
        let file = self.parse()?;
        Ok(format!("{:?}", file.format()))
    }

    #[allow(dead_code)]
    pub fn get_arch_name(&self) -> Result<String> {
        let file = self.parse()?;
        Ok(format!("{:?}", file.architecture()))
    }
}
