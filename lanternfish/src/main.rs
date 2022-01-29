// https://doc.rust-lang.org/rust-by-example/std_misc/channels.html
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::{fs, thread};

const INPUT_PATH: &str = "input_test.txt";
const DAYS: i32 = 256;

fn main() {
    let lanternfish_initial_population = get_lanternfish_input(INPUT_PATH);

    let thread_count = lanternfish_initial_population.len();

    let mut children = vec![];
    let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
    for lanternfish in lanternfish_initial_population {
        let thread_tx = tx.clone();

        let child = thread::spawn(move || {
            // thread_tx.send(vec![lanternfish])
            simulate_lanternfish_population_growth(&[lanternfish], DAYS, thread_tx)
        });

        children.push(child);
    }

    let mut final_result = 0; 
    for _ in 0..thread_count {
        final_result += rx.recv().unwrap().len();
    }

    println!("{final_result:?}")
}

fn simulate_lanternfish_population_growth(
    lanternfish_population: &[u8],
    days_remaining: i32,
    sender: Sender<Vec<u8>>
) {
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
        sender.send(Vec::from(lanternfish_population)).unwrap();
    } else {
        simulate_lanternfish_population_growth(&new_list, days_remaining - 1, sender)
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
