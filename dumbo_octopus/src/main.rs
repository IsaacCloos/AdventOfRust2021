// reduce messaging while in dev
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{fs, ops::Range};

const INPUT_TEST_PATH: &str = "input_test.txt"; // 1656 total for input_test
const INPUT_PATH: &str = "input.txt"; // ...

const SIMULATION_RANGE: Range<i32> = 0..100;

fn main() {
    let mut energy_grid = get_energy_levels_grid_from_file(INPUT_TEST_PATH);
    let mut flashes_count = 0;

    energy_grid.friendly_print();

    for day in SIMULATION_RANGE {
        for row in energy_grid.iter_mut() {
            for cell in row.iter_mut() {
                *cell += 1;
            }
        }
    }

    energy_grid.friendly_print()
}

type EnergyGrid = Vec<Vec<u32>>;

trait FriendlyPrint {
    fn friendly_print(&self) -> ();
}

impl FriendlyPrint for EnergyGrid {
    fn friendly_print(&self) -> () {
        for row in self {
            println!("{row:?}")
        }
    }
}


fn get_energy_levels_grid_from_file(file_path: &str) -> EnergyGrid {
    fs::read_to_string(file_path)
        .expect("Failed to read file from input path")
        .trim()
        .split_whitespace()
        .map(|row| {
            row.chars()
                .map(|c| char::to_digit(c, 10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<_>()
}
