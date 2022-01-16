// external file reference
// https://stackoverflow.com/questions/26388861/how-to-include-a-module-from-another-file-from-the-same-project
mod sonar_tools;

// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#constants
// [amature take] constants are free!
const INPUT_PATH: &str = "input.txt";

fn main() {
    let measurements = sonar_tools::path_to_veci32(INPUT_PATH);

    let measured_increases = sonar_tools::measure_increases(&measurements);

    println!("{measured_increases}")
}