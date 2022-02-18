mod pathing_tools;
use pathing_tools::{CaveMap, CaveDiver};

/*
input_test results
    PART 1: 10
    PART 2: 36
*/

const INPUT_PATH: &str = "input_test.txt";

fn main() {
    let mut cave_map = CaveMap::from(INPUT_PATH);
    let cave_diver = CaveDiver::new();

    cave_diver.scan(&mut cave_map);

    for path_option in cave_map.path_options.iter() {
        println!("{path_option:?}")
    }

    println!("###");
    println!("{}", cave_map.path_options.len());
}