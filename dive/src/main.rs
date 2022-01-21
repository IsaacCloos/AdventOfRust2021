mod dive_tools;

const INPUT_FILE: &str = "input2.txt";

fn main() {
    let test = dive_tools::parse_dive_instructions_file(INPUT_FILE);

    println!("{test:#?}")
}