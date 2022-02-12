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
    /*
        First, the energy level of each octopus increases by 1.
        Then, any octopus with an energy level greater than 9 flashes.
        This increases the energy level of all adjacent octopuses by 1, including octopuses that are diagonally adjacent.
        If this causes an octopus to have an energy level greater than 9, it also flashes.
        This process continues as long as new octopuses keep having their energy level increased beyond 9.
            (An octopus can only flash at most once per step.)
        Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
    */

    for day in SIMULATION_RANGE {
        let mut days_flashes = Vec::<(Point, u32)>::new();

        // STEP ONE
        // increase each octopous by 1
        for (ri, row) in energy_grid.iter_mut().enumerate() {
            for (ci, cell) in row.iter_mut().enumerate() {
                if cell == &9 {
                    days_flashes.push((Point { x: ci, y: ri }, *cell))
                }

                if *cell > 9 {
                    *cell = 0;
                }

                *cell += 1;
            }
        }

        // STEP TWO

        // STEP THREE
    }

    println!("######");
    energy_grid.friendly_print();
}

type EnergyGrid = Vec<Vec<u32>>;

struct Point {
    x: usize,
    y: usize,
}

trait Energy {
    fn charge(&mut self);
}

impl Energy for u32 {
    fn charge(&mut self) {
        *self += 1;
    }
}

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
