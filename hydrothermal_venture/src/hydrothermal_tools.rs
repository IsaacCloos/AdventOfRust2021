use std::fs;

#[derive(Debug)]
pub enum VentLineNature {
    /// x1 == x2
    Vertical,
    /// y1 == y2
    Horizontal,
    /// (x2 - x1) == (y2 - y1)
    Diagonal,
    ///
    Abstract,
}

#[derive(Debug)]
pub struct VentLine {
    pub start: (i32, i32),
    pub end: (i32, i32),
    pub nature: VentLineNature,
}

// https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
impl From<&String> for VentLine {
    fn from(item: &String) -> Self {
        let vent_coordinates: Vec<Vec<i32>> = item
            .split(" -> ")
            .map(|x| {
                x.trim()
                    .split(",")
                    .map(|y| {
                        str::parse::<i32>(y)
                            .expect("Failed to parse number from hydrothermal report increment")
                    })
                    .collect::<Vec<i32>>()
            })
            .collect();
        let x1 = vent_coordinates[0][0];
        let y1 = vent_coordinates[0][1];
        let x2 = vent_coordinates[1][0];
        let y2 = vent_coordinates[1][1];

        // if vent_coordinates[1][0] > vent_coordinates[0][0] {
        //     x1 = vent_coordinates[0][0];
        //     x2 = vent_coordinates[1][0];
        // } else {
        //     x1 = vent_coordinates[1][0];
        //     x2 = vent_coordinates[0][0];
        // }

        // if vent_coordinates[1][1] > vent_coordinates[0][1] {
        //     y1 = vent_coordinates[0][1];
        //     y2 = vent_coordinates[1][1];
        // } else {
        //     y1 = vent_coordinates[1][1];
        //     y2 = vent_coordinates[0][1];
        // }

        VentLine {
            start: (x1, y1),
            end: (x2, y2),
            nature: if x1 == x2 {
                VentLineNature::Vertical
            } else if y1 == y2 {
                VentLineNature::Horizontal
            } else if (x2 - x1).abs() == (y2 - y1).abs() {
                VentLineNature::Diagonal
            } else {
                VentLineNature::Abstract
            },
        }
    }
}

impl VentLine {
    pub fn get_line_coordinates(&self) -> Option<Vec<(i32, i32)>> {
        let mut result = Vec::new();
        match self.nature {
            VentLineNature::Vertical => {
                if self.end.1 > self.start.1 {
                    for num in self.start.1..=self.end.1 {
                        result.push((self.start.0, num));
                    }
                } else {
                    for num in self.end.1..=self.start.1 {
                        result.push((self.start.0, num));
                    }
                }

                Some(result)
            }
            VentLineNature::Horizontal => {
                if self.end.0 > self.start.0 {
                    for num in self.start.0..=self.end.0 {
                        result.push((num, self.start.1));
                    }
                } else {
                    for num in self.end.0..=self.start.0 {
                        result.push((num, self.start.1));
                    }
                }

                Some(result)
            }
            // PART 2
            VentLineNature::Diagonal => {
                // println!("{self:?}");
                let mut horizontal_ticks = Vec::<i32>::new();
                let mut vertical_ticks = Vec::<i32>::new();
                if self.end.0 > self.start.0 {
                    // forward
                    for num in self.start.0..=self.end.0 {
                        horizontal_ticks.push(num)
                    }
                } else {
                    // backward
                    for num in (self.end.0..=self.start.0).rev() {
                        horizontal_ticks.push(num)
                    }
                }
                if self.end.1 > self.start.1 {
                    // up
                    for num in self.start.1..=self.end.1 {
                        vertical_ticks.push(num)
                    }
                } else {
                    // down
                    for num in (self.end.1..=self.start.1).rev() {
                        vertical_ticks.push(num)
                    }
                }

                // println!("{horizontal_ticks:?}\n{vertical_ticks:?}");
                // horizontal_ticks and vertical_ticks will always have same length
                for (i, tick) in horizontal_ticks.iter().enumerate() {
                    result.push((*tick, vertical_ticks[i]))
                }

                Some(result)
            }
            VentLineNature::Abstract => None,
        }
        //     match vent_line.nature {
        //         VentLineNature::Vertical => {
        //             for num in vent_line.start.1..=vent_line.end.1 {
        //                 if straight_vent_map.contains_key(&(vent_line.start.0, num)) {
        //                     *straight_vent_map
        //                         .get_mut(&(vent_line.start.0, num))
        //                         .unwrap() += 1;
        //                 } else {
        //                     straight_vent_map.insert((vent_line.start.0, num), 1);
        //                 }
        //             }
        //         }
        //         VentLineNature::Horizontal => {
        //             for num in vent_line.start.0..=vent_line.end.0 {
        //                 if straight_vent_map.contains_key(&(num, vent_line.start.1)) {
        //                     *straight_vent_map
        //                         .get_mut(&(num, vent_line.start.1))
        //                         .unwrap() += 1;
        //                 } else {
        //                     straight_vent_map.insert((num, vent_line.start.1), 1);
        //                 }
        //             }
        //         }
        //         VentLineNature::Diagonal => {
        //             println!("{vent_line:?}");
        //             for num in vent_line.start.0..=vent_line.end.1 {
        //                 if straight_vent_map.contains_key(&(num, num)) {
        //                     *straight_vent_map
        //                         .get_mut(&(num, num))
        //                         .unwrap() += 1;
        //                 } else {
        //                     straight_vent_map.insert((num, num), 1);
        //                 }
        //             }
        //         }
        //         VentLineNature::Abstract => {}
        //     }
    }
}

pub fn import_hydrothermal_report(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .expect("Failed to import hydrothermal report")
        .trim()
        .split("\n")
        .map(str::to_owned)
        .collect()
}
