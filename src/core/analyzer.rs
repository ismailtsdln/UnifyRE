use crate::core::profiles::AnalysisProfile;
use crate::core::traits::BinaryProvider;
use crate::errors::Result;
use object::{Object, ObjectSection, ObjectSymbol};
use serde::Serialize;

#[derive(Serialize)]
pub struct AnalysisResult {
    pub format: String,
    pub architecture: String,
    pub entry_point: u64,
    pub profile: AnalysisProfile,
    pub sections: Vec<SectionInfo>,
    pub symbols: Vec<SymbolInfo>,
    pub findings: Vec<Finding>,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Clone, Debug)]
pub enum Confidence {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Clone, Debug)]
pub struct Finding {
    pub id: String,
    pub message: String,
    pub confidence: Confidence,
    pub offset: Option<u64>,
}

#[derive(Serialize)]
pub struct SectionInfo {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub entropy: f64,
}

#[derive(Serialize)]
pub struct SymbolInfo {
    pub name: String,
    pub address: u64,
    pub kind: String,
}

pub struct Analyzer<'a> {
    provider: &'a dyn BinaryProvider,
    profile: AnalysisProfile,
}

impl<'a> Analyzer<'a> {
    pub fn new(provider: &'a dyn BinaryProvider, profile: AnalysisProfile) -> Self {
        Self { provider, profile }
    }

    pub fn analyze(&self) -> Result<AnalysisResult> {
        let file = self.provider.parse()?;

        let mut sections: Vec<SectionInfo> = file
            .sections()
            .map(|s| {
                let data = s.data().unwrap_or_default();
                let entropy = crate::utils::helpers::calculate_entropy(data);
                SectionInfo {
                    name: s.name().unwrap_or_default().to_string(),
                    address: s.address(),
                    size: s.size(),
                    entropy,
                }
            })
            .collect();
        sections.sort_by_key(|s| s.address);

        let mut symbols: Vec<SymbolInfo> = file
            .symbols()
            .map(|s| SymbolInfo {
                name: s.name().unwrap_or_default().to_string(),
                address: s.address(),
                kind: format!("{:?}", s.kind()),
            })
            .collect();
        symbols.sort_by_key(|s| s.address);

        let mut findings: Vec<Finding> = Vec::new();

        if self.profile.should_run_entropy() {
            for section in &sections {
                if section.entropy > 7.0 {
                    findings.push(Finding {
                        id: "HIGH_ENTROPY".to_string(),
                        message: format!(
                            "Section {} has high entropy ({:.2}) - potentially packed or encrypted.",
                            section.name, section.entropy
                        ),
                        confidence: Confidence::High,
                        offset: Some(section.address),
                    });
                }
            }
        }

        if self.profile.should_run_suspicious_seq() {
            let suspicious =
                crate::utils::helpers::detect_suspicious_sequences(self.provider.data());
            for (offset, desc) in suspicious {
                findings.push(Finding {
                    id: "SUSPICIOUS_SEQ".to_string(),
                    message: desc.clone(),
                    confidence: Confidence::Medium,
                    offset: Some(offset as u64),
                });
            }
        }

        // Low confidence finding: Suspicious section names
        for section in &sections {
            if section.name == ".packed" || section.name == "UPX" {
                findings.push(Finding {
                    id: "SUSPICIOUS_SECTION".to_string(),
                    message: format!("Suspicious section name found: {}", section.name),
                    confidence: Confidence::Low,
                    offset: Some(section.address),
                });
            }
        }

        // Extract format-specific metadata
        let metadata = match file.format() {
            object::BinaryFormat::Elf => {
                crate::formats::elf::ElfAnalyzer::new(&file).extract_metadata()?
            }
            object::BinaryFormat::Pe => {
                crate::formats::pe::PeAnalyzer::new(&file).extract_metadata()?
            }
            object::BinaryFormat::MachO => {
                crate::formats::mach::MachOAnalyzer::new(&file).extract_metadata()?
            }
            _ => serde_json::Value::Null,
        };

        Ok(AnalysisResult {
            format: format!("{:?}", file.format()),
            architecture: format!("{:?}", file.architecture()),
            entry_point: file.entry(),
            profile: self.profile,
            sections,
            symbols,
            findings,
            metadata,
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
