use crate::core::loader::BinaryLoader;
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
    loader: &'a BinaryLoader,
}

impl<'a> Analyzer<'a> {
    pub fn new(loader: &'a BinaryLoader) -> Self {
        Self { loader }
    }

    pub fn analyze(&self) -> Result<AnalysisResult> {
        let file = self.loader.parse()?;

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
}
