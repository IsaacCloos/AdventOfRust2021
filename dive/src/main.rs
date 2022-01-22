mod dive_tools;
use dive_tools::{parse_dive_instructions_file, DiveDirection};

const INPUT_FILE: &str = "input2.txt";

fn main() {
    // PART ONE
    let mut distance = 0;
    let mut depth = 0;
    
    let dive_commands = parse_dive_instructions_file(INPUT_FILE);

    // https://doc.rust-lang.org/book/ch06-02-match.html
    for dive_command in &dive_commands {
        match dive_command.0 {
            DiveDirection::Up => depth -= dive_command.1,
            DiveDirection::Down => depth += dive_command.1,
            DiveDirection::Forward => distance += dive_command.1
        }
    }

    println!("### PART ONE ###\ndistance:\t\t{distance}\ndepth:\t\t\t{depth}\nfinal calculation:\t{final}", final = distance * depth);

    // PART TWO
    // additional property to measure, and slightly extended logic for match pattern
    // Also convert for loops to references (would have broken the second if the first took ownership of the dive_commands)
    distance = 0;
    depth = 0;
    let mut accuracy = 0;

    for dive_command in &dive_commands {
        match dive_command.0 {
            DiveDirection::Up => accuracy -= dive_command.1,
            DiveDirection::Down => accuracy += dive_command.1,
            DiveDirection::Forward => {
                distance += dive_command.1;
                depth += dive_command.1 * accuracy;
            }
        }
    }

    println!("### PART TWO ###\ndistance:\t\t{distance}\ndepth:\t\t\t{depth}\nfinal calculation:\t{final}", final = distance * depth);
}
