use std::{fs, cell};

const FILE_PATH: &str = "heightmap_test.txt";

type HeightMap = Vec<Vec<u32>>;

trait MapExtensions {
    fn get_cell_value(&self, point: &Cell) -> u32;
}

impl MapExtensions for HeightMap {
    fn get_cell_value(&self, point: &Cell) -> u32 {
        self[point.y][point.x]
    }
}

#[derive(Debug)]
struct Cell {
    x: usize,
    y: usize,
}

fn main() {
    let height_map = get_heightmap_from_file(FILE_PATH);
    get_regions(&height_map);
}

fn get_regions(map: &HeightMap) -> Option<Vec<i32>> {
    let mut basin_segments = Vec::<Vec<(Vec<(&u32, usize)>, bool)>>::new();
    let mut basin_segment_totals = Vec::<_>::new();

    for row in map.into_iter() {
        let row_iterator = row
            .iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<Vec<(_, _)>>();
        let row_segments = row_iterator
            .split(|number| *number.0 == 9)
            .map(|x| x.to_owned())
            .filter(|elm| elm.len() > 0)
            .collect::<Vec<_>>();

            basin_segments.push(row_segments);
    }

    for (i, row) in basin_segments.iter().enumerate() {
        for segment in row {
            let mut basin_size = 0;
            basin_size += segment.len();

            for cell in segment.iter() {
                'next_segment: for next_segment in basin_segments[i + 1].iter() {
                    for next_cell in next_segment {
                        if cell.1 == next_cell.1 {
                            basin_size += next_segment.len();
                            continue 'next_segment;
                        }
                    }
                }
            }

            basin_segment_totals.push(basin_size);
        }
        print!("\n")
    }

    println!("{basin_segment_totals:?}");

    None
}

fn check_adjacent_locations(map: &HeightMap, cell: Cell) -> Option<u32> {
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
