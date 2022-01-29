use std::{fs, process::exit, thread};

const INPUT_PATH: &str = "input_test.txt";
const DAYS_PASSED: i32 = 80;

const NTHREADS: usize = 5;

// https://doc.rust-lang.org/rust-by-example/std_misc/threads.html

fn main() {
    let lanternfish_starting_population: Vec<u8> = get_lanternfish_input(INPUT_PATH);
    //  println!("{} {} {}", lanternfish_starting_population.len(), NTHREADS, (lanternfish_starting_population.len() / NTHREADS) + 1);
    // exit(0);
    let mut lanternfish_population_segments: Vec<Vec<u8>> = lanternfish_starting_population
        .chunks(lanternfish_starting_population.len() / NTHREADS)
        .map(|x| x.to_owned())
        .collect();
    let mut lanternfish_total_count = 0;

    // good enough for PART ONE
    // for lanternfish_population_segment in &mut lanternfish_population_segments {
    //     lanternfish_total_count +=
    //         simulate_lanternfish_population_growth_v2(lanternfish_population_segment, DAYS_PASSED);
    // }

    println!("{lanternfish_population_segments:?}");

    // for learning purposes | learn thread/channel basics > optimize PART ONE solution
    let mut children = vec![];
    for lanternfish_population_segment in lanternfish_population_segments {
        children.push(thread::spawn(move || -> usize {
            simulate_lanternfish_population_growth_alt(&lanternfish_population_segment, DAYS_PASSED)
        }));
    }
    for child in children {
        lanternfish_total_count += child.join().unwrap();
    }

    // assert_eq!(26984457539, lanternfish.len());  // input_test * 256 days =  26984457539
    println!("{lanternfish_total_count}"); // input_test * 80 days  =  5934
}

fn simulate_lanternfish_population_growth_alt(
    lanternfish_population: &[u8],
    days_remaining: i32,
) -> usize {
    println!("{days_remaining}");
    let mut new_list = Vec::<u8>::new();
    for lanternfish in lanternfish_population {
        if lanternfish == &0 {
            new_list.push(6);
            new_list.push(8);
        } else {
            new_list.push(lanternfish - 1);
        }
    }

    if days_remaining == 0 {
        lanternfish_population.len()
    } else {
        simulate_lanternfish_population_growth_alt(&new_list, days_remaining - 1)
    }
}

fn simulate_lanternfish_population_growth(lanternfish_population: &[u8], days: i32) -> usize {
    let mut lanternfish: Vec<u8> = Vec::from(lanternfish_population);
    for num in 0..days {
        let mut new_lanterfish: usize = 0;
        for fish in &mut lanternfish {
            if *fish == 0 {
                *fish = 6;
                new_lanterfish += 1;
            } else {
                *fish -= 1;
            }
        }
        for _ in 0..new_lanterfish {
            lanternfish.push(8)
        }
        println!("{num}")
    }

    lanternfish.len()
}

fn get_lanternfish_input(input_path: &str) -> Vec<u8> {
    fs::read_to_string(input_path)
        .expect("Failed to parse input file")
        .trim()
        .split(",")
        .map(|x| str::parse::<u8>(x).expect("Failed to parse increment of input file to u8"))
        .collect()
}
