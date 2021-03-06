use std::fs;

// https://dev.to/cthutu/rust-5-naming-conventions-3cjf#casing

// https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
#[derive(Debug)]
pub struct DiveCommand(pub DiveDirection, pub i32);

// https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
/// interpret a space single delimited string between a valid dive command and an integer defining distance
/// acceptable commands:
/// forward
/// up
/// down
impl From<&str> for DiveCommand {
    fn from(item: &str) -> Self {
        let command_parts = item.split_whitespace().collect::<Vec<&str>>();

        // TODO: research appropriate way to fail a FROM trait function
        if command_parts.len() != 2 {
            return DiveCommand(DiveDirection::Forward, 0)
        }

        let direction = match command_parts[0] {
            "up" => DiveDirection::Up,
            "down" => DiveDirection::Down,
            "forward" => DiveDirection::Forward,
            _ => panic!("failed to discern direction of dive command"),
        };

        let distance = str::parse::<i32>(command_parts[1]).expect("failed to measure distance of dive command");

        DiveCommand(direction, distance)
    }
}

// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
#[derive(Debug)]
pub enum DiveDirection {
    Up,
    Down,
    Forward,
}

/// interprets specified text file of dive instructions as DiveCommand(s)
pub fn parse_dive_instructions_file(input_path: &str) -> Vec<DiveCommand> {
    fs::read_to_string(input_path)
        .expect(input_path)
        .trim() // clean end of file
        .split("\n") // split on new lines
        .map(DiveCommand::from) // cool syntax for mapping vector into DiveCommand tuple struct
        .collect() // convert (cast ??) iterator to specified shape of the return for this function
}
