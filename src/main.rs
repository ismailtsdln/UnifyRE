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
use object::{Object, ObjectSection};

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
        Commands::Disasm { binary, entry, .. } => {
            let loader = match core::BinaryLoader::new(&binary) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("{} Error loading binary: {}", "✘".red(), e);
                    return;
                }
            };

            let file = match loader.parse() {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("{} Error parsing binary: {}", "✘".red(), e);
                    return;
                }
            };

            let disasm = match core::Disassembler::new(file.architecture()) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("{} Error initializing disassembler: {}", "✘".red(), e);
                    return;
                }
            };

            if entry {
                let entry_addr = file.entry();
                // Find section containing entry point
                let section = file
                    .sections()
                    .find(|s| entry_addr >= s.address() && entry_addr < s.address() + s.size());

                if let Some(s) = section {
                    let offset = entry_addr - s.address();
                    if let Ok(data) = s.data() {
                        let code = &data[offset as usize..];
                        match disasm.disassemble(code, entry_addr) {
                            Ok(insns) => {
                                for i in insns.iter().take(20) {
                                    println!(
                                        "{:#014x}: {:<8} {}",
                                        i.address,
                                        i.mnemonic.yellow(),
                                        i.op_str
                                    );
                                }
                            }
                            Err(e) => eprintln!("{} Disassembly error: {}", "✘".red(), e),
                        }
                    }
                } else {
                    eprintln!(
                        "{} Entry point {:#x} not found in any section",
                        "✘".red(),
                        entry_addr
                    );
                }
            }
        }
        Commands::Debug { target } => {
            println!("{} Debugging {}", "▶".blue(), target.bold());
            // TODO: Implement debugging
        }
        Commands::Scan { action } => match action {
            cli::commands::ScanCommands::Patterns { binary, pattern } => {
                let loader = match core::BinaryLoader::new(&binary) {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("{} Error loading binary: {}", "✘".red(), e);
                        return;
                    }
                };

                let analyzer = core::Analyzer::new(&loader);
                match analyzer.scan_patterns(&pattern) {
                    Ok(matches) => {
                        println!(
                            "{} Found {} matches for pattern {}",
                            "✔".green(),
                            matches.len(),
                            pattern.bold()
                        );
                        for m in matches {
                            println!("  {:#014x}", m);
                        }
                    }
                    Err(e) => eprintln!("{} Scan error: {}", "✘".red(), e),
                }
            }
        },
        Commands::Report { binary, out } => {
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

            match serde_json::to_string_pretty(&result) {
                Ok(json) => {
                    if let Err(e) = std::fs::write(&out, json) {
                        eprintln!("{} Error writing report to {}: {}", "✘".red(), out, e);
                    } else {
                        println!(
                            "{} Report generated successfully: {}",
                            "✔".green(),
                            out.bold()
                        );
                    }
                }
                Err(e) => eprintln!("{} Error generating JSON: {}", "✘".red(), e),
            }
        }
    }
}
