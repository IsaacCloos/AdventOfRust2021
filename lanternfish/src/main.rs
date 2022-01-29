use std::fs;

const INPUT_PATH: &str = "input_test.txt";
const DAYS_PASSED: i32 = 256;

fn main() {
    let mut lanterfish = get_lanternfish_input(INPUT_PATH);

    for num in 0..DAYS_PASSED {
        let mut new_lanterfish: usize = 0;
        for fish in &mut lanterfish {
            if *fish == 0 {
                *fish = 6;
                new_lanterfish += 1;
            } else {
                *fish -= 1;
            }
        }
        for _ in 0..new_lanterfish {
            lanterfish.push(8)
        }
        println!("processing day: {num}")
    }

    assert_eq!(26984457539, lanterfish.len());
    // println!("{}", lanterfish.len());
}

fn get_lanternfish_input(input_path: &str) -> Vec<u8> {
    fs::read_to_string(input_path)
        .expect("Failed to parse input file")
        .trim()
        .split(",")
        .map(|x| str::parse::<u8>(x).expect("Failed to parse increment of input file to u8"))
        .collect()
}
