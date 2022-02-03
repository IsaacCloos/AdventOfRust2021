use std::fs;
                                        //  1  7  4  8
const UNIQUE_SEGMENT_LENGTHS: [usize; 4] = [2, 3, 4, 7];
const FILE_PATH: &str = "input.txt";

#[derive(Debug)]
struct Segment {
    test_signals: Vec<String>,
    output_signals: Vec<String>,
}

impl From<Vec<Vec<String>>> for Segment {
    fn from(signals: Vec<Vec<String>>) -> Self {
        Segment {
            test_signals: signals[0].to_owned(),
            output_signals: signals[1].to_owned(),
        }
    }
}

impl Segment {
    fn count_output_occurance(self, reference_sizes: &Vec<usize>) -> usize {
        let mut occurance_count = 0;

        for x in self.output_signals {
            if reference_sizes.contains(&x.len()) {
                occurance_count += 1;
            }
        }
        
        occurance_count
    }
}

fn main() {
    let ref_lengths = Vec::from(UNIQUE_SEGMENT_LENGTHS);
    let data = get_seven_segment_data(FILE_PATH);
    let mut ref_count: usize = 0;
    for x in data {
       ref_count += x.count_output_occurance(&ref_lengths);
    }

    println!("{ref_count}")
}

fn get_seven_segment_data(input_path: &str) -> Vec<Segment> {
    fs::read_to_string(input_path)
        .expect("Failed to parse provided file path")
        .trim_end()
        .split("\n")
        .map(|line| {
            line.split("|")
                .map(|x| {
                    x.trim()
                        .split_whitespace()
                        .map(str::to_owned)
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>()
        })
        .map(Segment::from)
        .collect::<Vec<Segment>>()
}