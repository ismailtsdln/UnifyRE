use crate::core::traits::BinaryProvider;
use crate::errors::Result;
use object::{Object, ObjectSection, ObjectSymbol};
use serde::Serialize;

#[derive(Serialize)]
pub struct AnalysisResult {
    pub format: String,
    pub architecture: String,
    pub entry_point: u64,
    pub sections: Vec<SectionInfo>,
    pub symbols: Vec<SymbolInfo>,
}

#[derive(Serialize)]
pub struct SectionInfo {
    pub name: String,
    pub address: u64,
    pub size: u64,
}

#[derive(Serialize)]
pub struct SymbolInfo {
    pub name: String,
    pub address: u64,
    pub kind: String,
}

pub struct Analyzer<'a> {
    provider: &'a dyn BinaryProvider,
}

impl<'a> Analyzer<'a> {
    pub fn new(provider: &'a dyn BinaryProvider) -> Self {
        Self { provider }
    }

    pub fn analyze(&self) -> Result<AnalysisResult> {
        let file = self.provider.parse()?;

        let sections = file
            .sections()
            .map(|s| SectionInfo {
                name: s.name().unwrap_or_default().to_string(),
                address: s.address(),
                size: s.size(),
            })
            .collect();

        let symbols = file
            .symbols()
            .map(|s| SymbolInfo {
                name: s.name().unwrap_or_default().to_string(),
                address: s.address(),
                kind: format!("{:?}", s.kind()),
            })
            .collect();

        Ok(AnalysisResult {
            format: format!("{:?}", file.format()),
            architecture: format!("{:?}", file.architecture()),
            entry_point: file.entry(),
            sections,
            symbols,
        })
    }

    pub fn scan_patterns(&self, hex_pattern: &str) -> Result<Vec<u64>> {
        let pattern = hex::decode(hex_pattern).map_err(|e| {
            crate::errors::UnifyError::InvalidArgument(format!("Invalid hex pattern: {}", e))
        })?;

        let mut matches = Vec::new();
        let data = self.provider.data();

        for i in 0..data.len().saturating_sub(pattern.len()) {
            if &data[i..i + pattern.len()] == pattern.as_slice() {
                matches.push(i as u64);
            }
        }

        Ok(matches)
    }
}
