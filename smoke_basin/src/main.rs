use std::fs;

const FILE_PATH: &str = "heightmap.txt";

type HeightMap = Vec<Vec<u32>>;

type SegmentRow = Vec<Vec<(usize, usize)>>;

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

    // 7010262 is too high

    // 1017792 YES!!

    println!("{result}")
}

fn get_regions(map: &HeightMap) -> usize {
    let mut segment_rows = Vec::<SegmentRow>::new();

    // load segments
    for (row_index, row) in map.into_iter().enumerate() {
        let row_iterator = row
            .iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<Vec<(_, _)>>();

        let row_segments = row_iterator
            .split(|number| *number.0 == 9)
            .map(|x| {
                x.iter()
                    .map(|(_a, b)| (row_index, *b))
                    .collect::<Vec<(_, _)>>()
            })
            .filter(|elm| elm.len() > 0)
            .collect::<Vec<_>>();

        segment_rows.push(row_segments);
    }

    // debugging block
    // for (i, segment_row) in segment_rows.iter().enumerate() {
    //     for segment in segment_row {
    //         print!("{segment:?}")
    //     }
    //     print!("\n")
    // }

    let mut reduced_output = segment_rows.into_iter().reduce(connect_segments).unwrap();

    // println!("###################################");
    // println!("reduced set");
    // // println!("{reduced_output:?}");
    // for x in reduced_output.iter() {
    //     print!("{} ", x.len());
    //     println!("{x:?}")
    // }

    reduced_output.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
    reduced_output[..3].iter().map(|x| x.len()).reduce(|a, b| a * b).unwrap()
}

fn connect_segments(
    mut segment_row_current: SegmentRow,
    segment_row_next: SegmentRow,
) -> SegmentRow {
    // for each segment of next row
    for next_segment in segment_row_next.iter() {
        let mut matched_segment_indicies = Vec::<usize>::new();

        // for each cell of next segment
        for next_cell in next_segment {
            // check each collected segment
            for (current_index, current_segment) in segment_row_current.iter().enumerate() {
                // check each cell of each collected segment
                for current_cell in current_segment {
                    if current_cell.1 == next_cell.1 && current_cell.0 == (next_cell.0 - 1) {
                        if !matched_segment_indicies.contains(&current_index) {
                            matched_segment_indicies.push(current_index);
                        }
                    }
                }
            }
        }

        match matched_segment_indicies.len() {
            // add new segment to reducing list
            0 => {
                segment_row_current.push(next_segment.clone());
            }
            // merge new segment with existing segment in collected list
            1 => {
                let connected_segment_index = *matched_segment_indicies.iter().next().unwrap();

                let current_connected_segment = segment_row_current
                    .get_mut(connected_segment_index)
                    .unwrap();

                for x in next_segment {
                    current_connected_segment.push(*x);
                }
            }
            // merge several segments together from previous rows of the map
            _ => {
                let mut new_merged_segment = Vec::<_>::new();
                matched_segment_indicies.sort();
                // println!("{matched_segment_indicies:?}");
                for matched_segment_index in matched_segment_indicies.into_iter().rev() {
                    let current_connected_segment = segment_row_current.remove(matched_segment_index);

                    for connected_segment_cell in current_connected_segment {
                        new_merged_segment.push(connected_segment_cell);
                    }
                }

                for next_segment_cell in next_segment {
                    new_merged_segment.push(*next_segment_cell)
                }

                segment_row_current.push(new_merged_segment);
            }
        }
    }

    segment_row_current
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
