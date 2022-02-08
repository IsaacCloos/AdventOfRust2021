use std::fs;

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
    let mut basin_edges = Vec::<Cell>::new();
    let mut basin_edge_groups = Vec::<Vec<Cell>>::new();
    let length = map[0].len();
    let height = map.len();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let cell = Cell { x, y };
            let cell_value = map.get_cell_value(&cell);
            if cell_value == 9 {
                basin_edges.push(cell);
            }
        }
    }

    // println!(
    //     "{}/{}\n#####################################\n{basin_edges:?}",
    //     basin_edges.len(),
    //     length * height
    // );

    for row in map.iter() {
        let row_iterator = row
            .iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<Vec<(_, _)>>();
        let basin_segments = row_iterator
            .split(|number| *number.0 == 9)
            .filter(|elm| elm.len() > 0)
            .collect::<Vec<_>>();

        println!("{:?}", basin_segments);
    }

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
