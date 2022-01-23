use crate::diagnostic_tools::DiagnosticReportResult;
mod diagnostic_tools;

const INPUT_PATH: &str = "input.txt";

fn main() {
    let diagnostic_report_results = DiagnosticReportResult::from(INPUT_PATH);

    println!("{}", diagnostic_report_results.get_power_consumption())
}
