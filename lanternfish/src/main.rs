// PART TWO of DAY FIVE warranted help!
// via advice from https://medium.com/codex/big-o-in-a-little-pond-a4422246d398
// "you only need a 9 element collection"
use std::{collections::HashMap, fs};

const INPUT_PATH: &str = "input.txt";
const DAYS: i32 = 256;

fn main() {
    let lanternfish_initial_population = get_lanternfish_input(INPUT_PATH);

    let mut lanternfish: HashMap<u8, usize> = HashMap::from([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
    ]);

    for fish in lanternfish_initial_population {
        *lanternfish.get_mut(&fish).unwrap() += 1;
    }

    for _ in 0..DAYS {
        let mut new_fish: usize = 0;
        for i in 0..8 {
            if i == 0 {
                new_fish += *lanternfish.get_mut(&i).unwrap();
            }
            *lanternfish.get_mut(&i).unwrap() = *lanternfish.get_mut(&(i + 1)).unwrap();
            if i == 7 {
                *lanternfish.get_mut(&8).unwrap() = 0;
            }
        }
        *lanternfish.get_mut(&6).unwrap() += new_fish;
        *lanternfish.get_mut(&8).unwrap() += new_fish;
    }

    let final_result = lanternfish.iter().map(|x| x.1).sum::<usize>();

    println!("{final_result}")
}

fn get_lanternfish_input(input_path: &str) -> Vec<u8> {
    fs::read_to_string(input_path)
        .expect("Failed to parse input file")
        .trim()
        .split(",")
        .map(|x| str::parse::<u8>(x).expect("Failed to parse increment of input file to u8"))
        .collect()
}
