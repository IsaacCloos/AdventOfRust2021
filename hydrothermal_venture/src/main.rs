use hydrothermal_tools::{import_hydrothermal_report, VentLine};

mod hydrothermal_tools;

const FILE_PATH: &str = "input.txt";

fn main() {
    let vent_lines_report = import_hydrothermal_report(FILE_PATH);

    let vent_lines: Vec<VentLine> = vent_lines_report.iter()
        .map(VentLine::from)
        .collect();

    for vent_line in vent_lines.iter().filter(|x| x.has_straight_line()) {
        println!("{:?} {:?}", vent_line.start, vent_line.end);
        
        // create log of numbers between which ever ranges qualify
        // could be a great excuse to learn about hashmaps in rust! 
        // increment the occurances number if the value shows up in the log repeatedly
    }
}