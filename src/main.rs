mod cli;
mod core;
mod errors;
mod formats;
mod output;
mod utils;

use crate::core::traits::BinaryProvider;
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
        Commands::Analyze {
            binary,
            format,
            profile,
            ..
        } => {
            let loader = match core::BinaryLoader::new(&binary) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("{} Error loading binary: {}", "✘".red(), e);
                    return;
                }
            };

            let analyzer = core::Analyzer::new(&loader, profile);
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
        Commands::Diff {
            binary1,
            binary2,
            format,
            profile,
        } => {
            let loader1 = core::BinaryLoader::new(&binary1);
            let loader2 = core::BinaryLoader::new(&binary2);

            if let (Ok(l1), Ok(l2)) = (loader1, loader2) {
                let analyzer1 = core::Analyzer::new(&l1, profile);
                let analyzer2 = core::Analyzer::new(&l2, profile);

                match (analyzer1.analyze(), analyzer2.analyze()) {
                    (Ok(res1), Ok(res2)) => {
                        let diff = core::diff::DiffEngine::compare(&res1, &res2);
                        if format == "json" {
                            println!("{}", serde_json::to_string_pretty(&diff).unwrap());
                        } else {
                            println!(
                                "{} {} vs {}",
                                "⚡ Binary Comparison:".bold().yellow(),
                                binary1,
                                binary2
                            );
                            println!("\n{} Sections:", "📦".blue());
                            for d in diff.section_diffs {
                                println!("  - {}", d);
                            }
                            println!("\n{} Symbols:", "🔍".green());
                            for d in diff.symbol_diffs {
                                println!("  - {}", d);
                            }
                        }
                    }
                    _ => eprintln!("{} Failed to analyze binaries for comparison", "✘".red()),
                }
            } else {
                eprintln!("{} Failed to load binaries for comparison", "✘".red());
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

                // Pattern scanning doesn't use the analyzer's profile logic yet
                let analyzer =
                    core::Analyzer::new(&loader, core::profiles::AnalysisProfile::Default);
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
        Commands::Report {
            binary,
            out,
            html,
            profile,
        } => {
            let loader = match core::BinaryLoader::new(&binary) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("{} Error loading binary: {}", "✘".red(), e);
                    return;
                }
            };

            let analyzer = core::Analyzer::new(&loader, profile);
            let result = match analyzer.analyze() {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{} Error analyzing binary: {}", "✘".red(), e);
                    return;
                }
            };

            if html {
                match crate::output::html::generate_html_report(&result, &out) {
                    Ok(_) => println!(
                        "{} HTML report generated successfully: {}",
                        "✔".green(),
                        out.bold()
                    ),
                    Err(e) => eprintln!("{} Error generating HTML report: {}", "✘".red(), e),
                }
            } else {
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
        Commands::Version => {
            println!("{} {}", "UnifyRE".bold().cyan(), env!("CARGO_PKG_VERSION"));
            println!("Architecture: {}", std::env::consts::ARCH);
            println!("OS: {}", std::env::consts::OS);
            println!("Contract Status: {}", "Locked (v1.0 Candidate)".green());
        }
        Commands::Run { script, binary } => {
            if let Err(e) = core::scripting::ScriptEngine::run(script, &binary) {
                eprintln!("{} Script error: {}", "✘".red(), e);
            }
        }
        Commands::Explain { finding_id } => {
            let engine = core::explanations::ExplanationEngine::new();
            if let Some(exp) = engine.explain(&finding_id) {
                println!(
                    "\n{} {}",
                    "🔍 Explanation for:".bold().cyan(),
                    finding_id.bold().yellow()
                );
                println!("{} {}", "Title:".bold(), exp.title);
                println!("{} {}", "Risk Level:".bold(), exp.risk_level.red());
                println!("\n{}", "Description:".bold());
                println!("{}", exp.description);
                println!("\n{}", "Detection Method:".bold());
                println!("{}", exp.detection_method);
            } else {
                println!("{} No explanation found for ID: {}", "✘".red(), finding_id);
            }
        }
    }
}
