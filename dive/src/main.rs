mod dive_tools;
use dive_tools::{parse_dive_instructions_file, DiveDirection};

const INPUT_FILE: &str = "input2.txt";

fn main() {
    let mut distance = 0;
    let mut depth = 0;
    
    let dive_commands = parse_dive_instructions_file(INPUT_FILE);

    // https://doc.rust-lang.org/book/ch06-02-match.html
    for dive_command in dive_commands {
        match dive_command.0 {
            DiveDirection::Up => depth -= dive_command.1,
            DiveDirection::Down => depth += dive_command.1,
            DiveDirection::Forward => distance += dive_command.1
        }
    }

    println!("distance: {distance}\ndepth: {depth}\ncalculate: {final}", final = distance * depth)
}
