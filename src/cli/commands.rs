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
}

#[derive(Subcommand)]
pub enum ScanCommands {
    /// Scan for specific patterns
    Patterns { binary: String },
}
