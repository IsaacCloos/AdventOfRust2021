// reduce messaging while in dev
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{fs, ops::Range};

const INPUT_TEST_PATH: &str = "input_test.txt"; // 1656 total for input_test
const INPUT_PATH: &str = "input.txt"; // 1637 for input

const SIMULATION_RANGE: Range<i32> = 1..301; // start with 1 for steps! threw off part 2 solution by 1

fn main() {
    let mut energy_grid = get_energy_levels_grid_from_file(INPUT_PATH);
    let mut total_flashes_count = 0;

    energy_grid.friendly_print();
    /*
        // charge
        1. First, the energy level of each octopus increases by 1.

        // flash chain
        2. Then, any octopus with an energy level greater than 9 flashes.
           This increases the energy level of all adjacent octopuses by 1, including octopuses that are diagonally adjacent.
           If this causes an octopus to have an energy level greater than 9, it also flashes.
           This process continues as long as new octopuses keep having their energy level increased beyond 9.
           (An octopus can only flash at most once per step.)

        // clean up
        3. Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
    */

    for step in SIMULATION_RANGE {
        // STEP ONE
        // grid increment
        for row in energy_grid.iter_mut() {
            for cell in row {
                *cell += 1;
            }
        }

        // STEP TWO
        // recursive flash check
        let pre = total_flashes_count;
        flash_analysis(&mut energy_grid, &mut total_flashes_count);
        let post = total_flashes_count;

        if post - pre == 100 {
            println!("{step}")
        }

        // if daily_flashes == 100 {
        //     println!("{day}");
        //     break;
        // }

        // total_flashes_count += daily_flashes;

        // STEP THREE
        // clean up
        for row in energy_grid.iter_mut() {
            for cell in row {
                if *cell > 9 {
                    *cell = 0;
                }
            }
        }
    }

    println!("######");
    println!("{total_flashes_count}")
}

fn flash_analysis(grid: &mut EnergyGrid, flashes_count: &mut i32) {
    let mut flash_points = Vec::<Point>::new();
    for (ri, row) in grid.iter_mut().enumerate() {
        for (ci, cell) in row.iter_mut().enumerate() {
            if *cell == 10 {
                *flashes_count += 1;
                *cell = 11; // release from ten after counted
                flash_points.push(Point { x: ci, y: ri })
            }
        }
    }

    for flash_point in flash_points.iter_mut() {
        let y_range = match flash_point.y {
            _ if flash_point.y == 0 => 0..=1,
            _ if flash_point.y == grid.len() - 1 => grid.len() - 2..=grid.len() - 1,
            _ => flash_point.y - 1..=flash_point.y + 1,
        };

        let x_range = match flash_point.x {
            _ if flash_point.x == 0 => 0..=1,
            _ if flash_point.x == grid.first().unwrap().len() - 1 => {
                grid.first().unwrap().len() - 2..=grid.first().unwrap().len() - 1
            }
            _ => flash_point.x - 1..=flash_point.x + 1,
        };

        for y in y_range {
            for x in x_range.clone() {
                let target = grid.get_mut(y).unwrap().get_mut(x).unwrap();
                if *target < 10 { // buffer to ten
                    *target += 1;
                }
            }
        }
    }

    if flash_points.len() > 0 {
        flash_analysis(grid, flashes_count)
    }
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
