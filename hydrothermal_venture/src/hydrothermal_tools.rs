use std::fs;

pub struct VentLine {
    pub start: (i32, i32),
    pub end: (i32, i32),
}

// https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
impl From<&String> for VentLine {
    fn from(item: &String) -> Self {
        let vent_coordinates: Vec<Vec<i32>> = item
            .split(" -> ")
            .map(|x| 
                x.split(",")
                    .map(|y| str::parse::<i32>(y).expect("Failed to parse number from hydrothermal report increment"))
                    .collect::<Vec<i32>>()
            )
            .collect();

        VentLine {
            start: (vent_coordinates[0][0], vent_coordinates[0][1]),
            end: (vent_coordinates[1][0], vent_coordinates[1][1]),
        }
    }
}

impl VentLine {
    pub fn has_straight_line(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
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
