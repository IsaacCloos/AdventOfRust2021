use std::fs;

const FILE_PATH: &str = "heightmap.txt";


type HeightMap = Vec<Vec<u32>>;

struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let height_map = get_heightmap_from_file(FILE_PATH);
    let mut sum_risk = 0;
    for y in 0..height_map.len() {
        for x in 0..height_map[0].len() {
            match check_adjacent_locations(&height_map, Point { x, y }) {
                Some(risk) => sum_risk += risk,
                None => {
                    // No behavior
                }
            }
        }
    }
    println!("{sum_risk}")
}
fn check_adjacent_locations(map: &HeightMap, cell: Point) -> Option<u32> {
    let cell_value = map[cell.y][cell.x];
    let length = map[0].len() - 1;
    let height = map.len() - 1;
    let x_range = match cell.x {
        0 => (cell.x, cell.x + 1),
        _ if cell.x == length => (cell.x - 1, cell.x),
        _ => (cell.x - 1, cell.x + 1),
    };
    let y_range = match cell.y {
        0 => (cell.y, cell.y + 1),
        _ if cell.y == height => (cell.y - 1, cell.y),
        _ => (cell.y - 1, cell.y + 1),
    };
    for y in y_range.0..=y_range.1 {
        for x in x_range.0..=x_range.1 {
            let target_cell = map[y][x];
            if target_cell <= cell_value && target_cell != cell_value {
                return None;
            }
        }
    }
    Some(1 + cell_value)
}
fn get_heightmap_from_file(file_path: &str) -> HeightMap {
    fs::read_to_string(file_path)
        .expect("Failed to read file")
        .trim()
        .split_whitespace()
        .map(|map_row| {
            map_row
                .chars()
                .map(|map_cell| map_cell.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<HeightMap>()
}
