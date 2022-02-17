mod pathing_tools;
use pathing_tools::{CaveMap, CaveDiver};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let mut cave_map = CaveMap::from(INPUT_PATH);
    let cave_diver = CaveDiver::new();

    cave_diver.scan(&mut cave_map);

    for path_option in cave_map.path_options.iter() {
        // println!("{path_option:?}")
    }
    println!("{}", cave_map.path_options.len())
}