use crate::errors::Result;
use object::File;

/// Abstract source of binary data
pub trait BinaryProvider: Send + Sync {
    /// Get the raw bytes of the binary
    fn data(&self) -> &[u8];

    /// Get the source path or identifier
    fn source(&self) -> &str;

    /// Parse the binary using the object crate
    fn parse(&self) -> Result<File<'_>> {
        File::parse(self.data()).map_err(|e| {
            crate::errors::UnifyError::ParseError(format!("Failed to parse binary: {}", e))
        })
    }
}

/// Interface for an analysis pass or component
pub trait AnalyzerComponent: Send + Sync {
    /// Name of the analysis component
    fn name(&self) -> &str;

    /// Description of what this component does
    fn description(&self) -> &str;

    /// Execute the analysis on the provided binary
    fn run(&self, provider: &dyn BinaryProvider) -> Result<serde_json::Value>;
}

/// Abstract instruction decoder
pub trait InstructionDecoder: Send + Sync {
    /// Decode instructions from the provided byte slice
    fn decode(
        &self,
        code: &[u8],
        address: u64,
    ) -> Result<Vec<crate::core::disassembler::InstructionInfo>>;
}
