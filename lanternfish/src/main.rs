// https://doc.rust-lang.org/rust-by-example/std_misc/channels.html
use std::{fs, thread};

const INPUT_PATH: &str = "input_test.txt";
const DAYS: u16 = 256;

fn main() {
    let lanternfish_initial_population = get_lanternfish_input(INPUT_PATH);
    let mut children = vec![];

    for lanternfish in lanternfish_initial_population {
        let child =
            thread::spawn(move || simulate_lanternfish_population_growth(&[lanternfish], DAYS));
        children.push(child);
    }

    let mut final_result = 0;
    for child in children {
        final_result += child.join().expect("Thread failed")
    }

    // let final_result =
    //     simulate_lanternfish_population_growth(&lanternfish_initial_population, DAYS);

    println!("{final_result:?}")
}

fn simulate_lanternfish_population_growth(
    lanternfish_population: &[u8],
    days_remaining: u16,
) -> usize {
    if days_remaining == 0 {
        return lanternfish_population.len();
    }
    // println!("{days_remaining}");
    let mut new_list = Vec::<u8>::new();
    for lanternfish in lanternfish_population {
        // match lanternfish {
        //     8 => new_list.push(7),
        //     7 => new_list.push(6),
        //     6 => new_list.push(5),
        //     5 => new_list.push(4),
        //     4 => new_list.push(3),
        //     3 => new_list.push(2),
        //     2 => new_list.push(1),
        //     1 => new_list.push(0),
        //     0 => {
        //         new_list.push(6);
        //         new_list.push(8);
        //     },
        //     _ => panic!("unexpected numebr!")
        // }
        if *lanternfish == 0 {
            new_list.push(6);
            new_list.push(8);
        } else {
            new_list.push(lanternfish - 1);
        }
    }

    if lanternfish_population.len() < 10_000_000 {
        simulate_lanternfish_population_growth(&new_list, days_remaining - 1)
    } else {
        println!("split! days remaining: {days_remaining}");
        // let (a, b) = new_list.split_at(new_list.len() / 2);
        // let halves = [a.to_owned(), b.to_owned()];

        let quarters = new_list.chunks(new_list.len() / 4).map(|x| x.to_owned());

        let mut children = Vec::new();

        for half in quarters {
            let child = thread::spawn(move || -> usize {
                simulate_lanternfish_population_growth(&half, days_remaining - 1)
            });

            children.push(child);
        }

        let mut result = 0;

        for child in children {
            result += child.join().unwrap()
        }

        result
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
