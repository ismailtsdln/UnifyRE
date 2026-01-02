mod cli;
mod core;
mod errors;
mod formats;
mod output;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use env_logger;

fn main() {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { binary, format, .. } => {
            let loader = match core::BinaryLoader::new(&binary) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("{} Error loading binary: {}", "✘".red(), e);
                    return;
                }
            };

            let analyzer = core::Analyzer::new(&loader);
            let result = match analyzer.analyze() {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{} Error analyzing binary: {}", "✘".red(), e);
                    return;
                }
            };

            if format == "json" {
                output::print_json_report(&result);
            } else {
                output::print_analysis_report(&result);
            }
        }
        Commands::Disasm { binary, .. } => {
            println!("{} Disassembling {}", "▶".blue(), binary.bold());
            // TODO: Implement disassembly
        }
        Commands::Debug { target } => {
            println!("{} Debugging {}", "▶".blue(), target.bold());
            // TODO: Implement debugging
        }
        Commands::Scan { action } => {
            match action {
                cli::commands::ScanCommands::Patterns { binary } => {
                    println!("{} Scanning patterns in {}", "▶".blue(), binary.bold());
                    // TODO: Implement pattern scanning
                }
            }
        }
    }
}
