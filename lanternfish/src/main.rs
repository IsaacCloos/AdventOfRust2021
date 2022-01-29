use std::{fs, thread};

const INPUT_PATH: &str = "input_test.txt";
const DAYS: i32 = 256;

fn main() {
    let lanternfish_initial_population = get_lanternfish_input(INPUT_PATH);

    let mut children = vec![];
    for lanternfish in lanternfish_initial_population {
        children.push(thread::spawn(move || -> usize {
            simulate_lanternfish_population_growth(&[lanternfish], DAYS)
        }))
    }
    let final_result = children
        .into_iter()
        .map(|c| c.join().unwrap())
        .sum::<usize>();

    // let final_result = simulate_lanternfish_population_growth(lanternfish_initial_population.as_slice(), DAYS);

    println!("{final_result:?}")
}

fn simulate_lanternfish_population_growth(
    lanternfish_population: &[u8],
    days_remaining: i32,
) -> usize {
    println!("{days_remaining}");
    let mut new_list = Vec::<u8>::new();
    for lanternfish in lanternfish_population {
        if *lanternfish == 0 {
            new_list.push(6);
            new_list.push(8);
        } else {
            new_list.push(lanternfish - 1);
        }
    }

    if days_remaining == 0 {
        lanternfish_population.len()
    } else {
        simulate_lanternfish_population_growth(&new_list, days_remaining - 1)
    }
}

fn get_lanternfish_input(input_path: &str) -> Vec<u8> {
    fs::read_to_string(input_path)
        .expect("Failed to parse input file")
        .trim()
        .split(",")
        .map(|x| str::parse::<u8>(x).expect("Failed to parse increment of input file to u8"))
        .collect()
}