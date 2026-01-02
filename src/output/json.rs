use crate::core::analyzer::AnalysisResult;
use serde_json;

pub fn print_json_report(report: &AnalysisResult) {
    match serde_json::to_string_pretty(report) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error generating JSON: {}", e),
    }
}
