use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum AnalysisProfile {
    Malware,
    Exploit,
    Audit,
    Default,
}

impl AnalysisProfile {
    pub fn should_run_entropy(&self) -> bool {
        match self {
            Self::Malware | Self::Audit | Self::Default => true,
            Self::Exploit => false,
        }
    }

    pub fn should_run_suspicious_seq(&self) -> bool {
        match self {
            Self::Malware | Self::Exploit | Self::Audit | Self::Default => true,
        }
    }
}
