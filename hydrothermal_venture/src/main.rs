use hydrothermal_tools::{import_hydrothermal_report, VentLine};
use std::collections::HashMap;

mod hydrothermal_tools;

const FILE_PATH: &str = "input.txt";

fn main() {
    let vent_lines_report = import_hydrothermal_report(FILE_PATH);
    // https://doc.rust-lang.org/std/collections/struct.HashMap.html
    let mut straight_vent_map: HashMap<(i32, i32), i32> = HashMap::new();

    let vent_lines: Vec<VentLine> = vent_lines_report.iter().map(VentLine::from).collect();

    for vent_line in vent_lines {
        match vent_line.get_line_coordinates() {
            Some(line_coordinates) => {
                for point in line_coordinates {
                    if straight_vent_map.contains_key(&point) {
                        *straight_vent_map.get_mut(&point).unwrap() += 1;
                    } else {
                        straight_vent_map.insert(point, 1);
                    }
                }
            }
            None => {
                // do nothing
            }
        }
    }

    let numbers_greater_than_2 = straight_vent_map.iter().filter(|x| x.1 >= &2).count();
    println!("{numbers_greater_than_2}");
}

// 3990

// brutal logic chain to solve part 1
// for vent_line in vent_lines {
//     if vent_line.start.0 == vent_line.end.0 {
//         if vent_line.end.1 > vent_line.start.1 {
//             for num in vent_line.start.1..=vent_line.end.1 {
//                 if straight_vent_map.contains_key(&(vent_line.start.0, num)) {
//                     *straight_vent_map
//                         .get_mut(&(vent_line.start.0, num))
//                         .unwrap() += 1;
//                 } else {
//                     straight_vent_map.insert((vent_line.start.0, num), 1);
//                 }
//             }
//         } else {
//             for num in vent_line.end.1..=vent_line.start.1 {
//                 if straight_vent_map.contains_key(&(vent_line.start.0, num)) {
//                     *straight_vent_map
//                         .get_mut(&(vent_line.start.0, num))
//                         .unwrap() += 1;
//                 } else {
//                     straight_vent_map.insert((vent_line.start.0, num), 1);
//                 }
//             }
//         }
//     } else if vent_line.start.1 == vent_line.end.1 {
//        if vent_line.end.0 > vent_line.start.0 {
//             for num in vent_line.start.0..=vent_line.end.0 {
//                 if straight_vent_map.contains_key(&(num, vent_line.start.1)) {
//                     *straight_vent_map
//                         .get_mut(&(num, vent_line.start.1))
//                         .unwrap() += 1;
//                 } else {
//                     straight_vent_map.insert((num, vent_line.start.1), 1);
//                 }
//             }
//         } else {
//             for num in vent_line.end.0..=vent_line.start.0 {
//                 if straight_vent_map.contains_key(&(num, vent_line.start.1)) {
//                     *straight_vent_map
//                         .get_mut(&(num, vent_line.start.1))
//                         .unwrap() += 1;
//                 } else {
//                     straight_vent_map.insert((num, vent_line.start.1), 1);
//                 }
//             }
//         }
//     }
// }
