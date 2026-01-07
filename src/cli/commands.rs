use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "unifyre")]
#[command(about = "Next-generation reverse engineering and binary analysis tool", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Perform static analysis on a binary
    Analyze {
        /// Path to the binary file
        binary: String,
        /// Target architecture (auto, x86, x64, arm, arm64)
        #[arg(long, default_value = "auto")]
        arch: String,
        /// Output format (human, json)
        #[arg(long, default_value = "human")]
        format: String,
        /// Show sections
        #[arg(long)]
        sections: bool,
        /// Show imports
        #[arg(long)]
        imports: bool,
        /// Show exports
        #[arg(long)]
        exports: bool,
        /// Extract strings
        #[arg(long)]
        strings: bool,
        /// Analysis profile (malware, exploit, audit, default)
        #[arg(long, default_value = "default", value_enum)]
        profile: crate::core::profiles::AnalysisProfile,
    },
    /// Compare two binaries
    Diff {
        /// Path to the first binary
        binary1: String,
        /// Path to the second binary
        binary2: String,
        /// Output format (human, json)
        #[arg(long, default_value = "human")]
        format: String,
        /// Analysis profile (malware, exploit, audit, default)
        #[arg(long, default_value = "default", value_enum)]
        profile: crate::core::profiles::AnalysisProfile,
    },
    /// Disassemble a binary
    Disasm {
        /// Path to the binary file
        binary: String,
        /// Start disassembly at entry point
        #[arg(long)]
        entry: bool,
        /// Address or symbol of function to disassemble
        #[arg(long)]
        function: Option<String>,
        /// Range of addresses to disassemble (start:end)
        #[arg(long)]
        range: Option<String>,
    },
    /// Start a debugging session
    Debug {
        /// Path to the binary file or PID to attach to
        target: String,
    },
    /// Advanced security features and reporting
    Scan {
        #[command(subcommand)]
        action: ScanCommands,
    },
    /// Show detailed version information
    Version,
    /// Explain a specific finding ID
    Explain {
        /// The finding ID to explain
        finding_id: String,
    },
    /// Generate a comprehensive report
    Report {
        /// Path to the binary file
        binary: String,
        /// Output file path
        #[arg(long, default_value = "report.json")]
        out: String,
        /// Generate HTML report
        #[arg(long)]
        html: bool,
        /// Analysis profile (malware, exploit, audit, default)
        #[arg(long, default_value = "default")]
        profile: crate::core::profiles::AnalysisProfile,
    },
    /// Run an automation script (.ure)
    Run {
        /// Path to the script file
        script: String,
        /// Path to the binary file
        binary: String,
    },
}

#[derive(Subcommand)]
pub enum ScanCommands {
    /// Scan for specific patterns
    Patterns {
        binary: String,
        /// Hex pattern to search for (e.g., 4889e5)
        pattern: String,
    },
}
