use std::{collections::HashSet, fs};

const FILE_PATH: &str = "heightmap.txt";

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
    let result = get_regions(&height_map);

    // 750975 is too low

    // println!("{result}")
}

fn get_regions(map: &HeightMap) -> usize {
    let mut basin_segments = Vec::<Vec<(Vec<(&u32, usize)>, bool)>>::new();
    let mut basin_segment_sizes = Vec::<(usize, Vec<(usize, HashSet<usize>)>)>::new();

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
            .collect::<Vec<_>>()
            .into_iter()
            .map(|x| (x, false))
            .collect::<_>();

        basin_segments.push(row_segments);
    }

    // get each row and index
    for (i, row) in basin_segments.into_iter().enumerate() {
        // get each segment of row
        for mut segment in row {
            // if row has been integrated skip it
            if segment.1 {
                continue;
            }

            // initialize set of indexes for segment
            let mut index_set = HashSet::<usize>::new();

            // load segment indexes
            for cell in segment.0.iter() {
                index_set.insert(cell.1);
            }

            // pre integration
            let mut match_found = false;
            let mut matched_index = 0;

            // integration check

            // for all segments collected already
            for (i1, collected_segments) in basin_segment_sizes.iter_mut().enumerate() {
                // 0 is y axis
                let previous_segments = &mut collected_segments.1;

                for index in index_set.iter() {
                    for previous_segment in previous_segments.iter_mut() {
                        for previous_index in previous_segment.1.iter() {
                            if index == previous_index
                                && previous_segment.0 as i32 == (i as i32 - 1 as i32)
                            {
                                match_found = true;
                                matched_index = i1;
                            }
                        }
                    }
                }
            }

            // apply and mark segment as completed
            if match_found {
                let matched_basin = basin_segment_sizes.get_mut(matched_index).unwrap();
                matched_basin.0 += segment.0.len();

                let matched_basin_definition = &mut matched_basin.1;
                matched_basin_definition.push((i, index_set));
            } else {
                basin_segment_sizes.push((segment.0.len(), Vec::from([(i, index_set)])));
            }

            // mark segment as integrated
            segment.1 = true;
        }
    }

    basin_segment_sizes.sort_by(|a, b| b.0.cmp(&a.0));

    let mut result: usize = 1;

    for defined_segment in &basin_segment_sizes {
        if defined_segment.0 == 1 {
            println!("{:?}", defined_segment);
        }
        // result *= defined_segment.0;
    }

    result
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
