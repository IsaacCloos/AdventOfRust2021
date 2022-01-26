use std::collections::HashMap;
use hydrothermal_tools::{import_hydrothermal_report, VentLine};

mod hydrothermal_tools;

const FILE_PATH: &str = "input.txt";

fn main() {
    let vent_lines_report = import_hydrothermal_report(FILE_PATH);
    // https://doc.rust-lang.org/std/collections/struct.HashMap.html
    let mut straight_vent_map: HashMap<i32, i32> = HashMap::new();

    let vent_lines: Vec<VentLine> = vent_lines_report.iter()
        .map(VentLine::from)
        .collect();

    for straight_vent_line in vent_lines.iter().filter(|x| x.has_straight_line()) {
        // println!("{:?} {:?}", straight_vent_line.start, straight_vent_line.end);
        
        // create log of numbers between which ever ranges qualify
        // could be a great excuse to learn about hashmaps in rust! 
        // increment the occurances number if the value shows up in the log repeatedly
        match straight_vent_line.get_straight_line() {
            Some(x) => {
                for number in x.0..x.1 {
                    if straight_vent_map.contains_key(&number) {
                        // https://stackoverflow.com/questions/30414424/how-can-i-update-a-value-in-a-mutable-hashmap
                        // dereference required??
                        *straight_vent_map.get_mut(&number).unwrap() += 1
                    } else {
                        straight_vent_map.insert(number, 1);
                    }
                }
            },
            None => panic!("Straight line not found within filtered list of straight lines"),
        }
    }

    let numbers_greater_than_2 = straight_vent_map.iter().filter(|x| x.1 >= &2).count();

    // 960 was too low! Probably limiting the range on line 25

    println!("{numbers_greater_than_2}")
}