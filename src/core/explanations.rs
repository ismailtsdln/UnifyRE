use std::collections::HashMap;

pub struct Explanation {
    pub title: String,
    pub description: String,
    pub detection_method: String,
    pub risk_level: String,
}

pub struct ExplanationEngine {
    kb: HashMap<String, Explanation>,
}

impl ExplanationEngine {
    pub fn new() -> Self {
        let mut kb = HashMap::new();

        kb.insert(
            "HIGH_ENTROPY".to_string(),
            Explanation {
                title: "High Entropy Section Detected".to_string(),
                description: "This section has a high Shannon entropy value (above 7.0). Entropy is a measure of randomness in data. High entropy is typical for encrypted or compressed data, which is often used by malware packers to hide their actual code.".to_string(),
                detection_method: "Calculated using the Shannon entropy formula on the raw byte content of the section.".to_string(),
                risk_level: "High - Possible packed or obfuscated code.".to_string(),
            },
        );

        kb.insert(
            "SUSPICIOUS_SEQ".to_string(),
            Explanation {
                title: "Suspicious Instruction Sequence".to_string(),
                description: "A sequence of instructions commonly associated with shellcode or exploitation attempts (e.g., long NOP sleds) was detected.".to_string(),
                detection_method: "Pattern scanning for known-bad or highly unusual opcode sequences.".to_string(),
                risk_level: "Medium - Could be legitimate padding or part of a shellcode payload.".to_string(),
            },
        );

        kb.insert(
            "SUSPICIOUS_SECTION".to_string(),
            Explanation {
                title: "Suspicious Section Name Detected".to_string(),
                description: "The binary contains sections with names commonly associated with packers or obfuscators (e.g., .packed, UPX, .aspack). This is a strong indicator that the binary is compressed or protected to hinder static analysis.".to_string(),
                detection_method: "Matches internal section names against a curated list of known-bad or packer-specific strings.".to_string(),
                risk_level: "Medium - High probability of packing.".to_string(),
            },
        );

        Self { kb }
    }

    pub fn explain(&self, id: &str) -> Option<&Explanation> {
        self.kb.get(id)
    }
}
