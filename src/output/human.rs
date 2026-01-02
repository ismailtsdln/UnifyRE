use crate::core::analyzer::AnalysisResult;
use colored::*;

pub fn print_analysis_report(report: &AnalysisResult) {
    println!("\n{}", "=== UnifyRE Analysis Report ===".bold().green());
    println!("{:<20} {}", "Format:".bold(), report.format);
    println!("{:<20} {}", "Architecture:".bold(), report.architecture);
    println!("{:<20} {:#x}", "Entry Point:".bold(), report.entry_point);

    println!("\n{}", "--- Sections ---".bold().cyan());
    println!("{:<20} {:<15} {:<15}", "Name", "Address", "Size");
    for section in &report.sections {
        println!(
            "{:<20} {:#014x} {:#014x}",
            section.name, section.address, section.size
        );
    }

    if !report.symbols.is_empty() {
        println!("\n{}", "--- Symbols ---".bold().cyan());
        println!("{:<30} {:<15} {:<15}", "Name", "Address", "Kind");
        for symbol in report.symbols.iter().take(20) {
            // Limit output for readability
            println!(
                "{:<30} {:#014x} {:?}",
                symbol.name, symbol.address, symbol.kind
            );
        }
        if report.symbols.len() > 20 {
            println!("... ({} more symbols)", report.symbols.len() - 20);
        }
    }
}
