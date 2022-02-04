use std::fs;
//  1  7  4  8
const UNIQUE_SEGMENT_LENGTHS: [usize; 4] = [2, 3, 4, 7];
const FILE_PATH: &str = "input.txt";

#[derive(Debug)]
struct Number {
    value: u8,
    code: String,
}

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
    // narrow solution for part 1
    fn count_output_occurance(self, reference_sizes: &Vec<usize>) -> usize {
        let mut occurance_count = 0;

        for x in self.output_signals {
            if reference_sizes.contains(&x.len()) {
                occurance_count += 1;
            }
        }

        occurance_count
    }

    fn analyze_test_signals(&self) -> Vec<Number> {
        let mut numbers = Vec::<Number>::new();
        let mut one = self
            .test_signals
            .iter()
            .filter(|x| x.len() == 2)
            .next()
            .expect("Test signal set did not include 1")
            .chars()
            .collect::<Vec<char>>();
        let mut seven = self
            .test_signals
            .iter()
            .filter(|x| x.len() == 3)
            .next()
            .expect("Test signal set did not include 7")
            .chars()
            .collect::<Vec<char>>();
        let mut four = self
            .test_signals
            .iter()
            .filter(|x| x.len() == 4)
            .next()
            .expect("Test signal set did not include 7")
            .chars()
            .collect::<Vec<char>>();
        let top = seven
            .iter()
            .filter(|x| !one.contains(x))
            .collect::<Vec<&char>>();
        let upper_left_and_middle = four
            .iter()
            .filter(|x| !one.contains(x))
            .collect::<Vec<&char>>();

        let mut bottom = Vec::<char>::new();
        let mut middle = Vec::<char>::new();
        let mut bottom_right = Vec::<char>::new();
        let mut upper_right = Vec::<char>::new();
        let mut bottom_left = Vec::<char>::new();
        let mut eight = Vec::<char>::new();
        let mut zero: Vec<&char> = Vec::new();

        for segment in self.test_signals.iter() {
            let chars = segment.chars().collect::<Vec<char>>();
            if chars.len() == 7 {
                eight = Vec::from(chars.clone());
            }
        }

        // finding 5
        'signals: for segment in self.test_signals.iter() {
            let chars = segment.chars().collect::<Vec<char>>();
            let mut relevance_to_one = 0;
            let mut bottom_right_ref = 'z';
            let mut upper_right_ref = 'z';
            // chars.push(top.first().unwrap().to_owned());
            // for x in one.iter() {
            //     if !chars.contains(x) {
            //         continue 'signals;
            //     }
            // }

            for c in chars.iter() {
                for c1 in one.iter() {
                    if c == c1 {
                        relevance_to_one += 1;
                        bottom_right_ref = *c;
                        for c2 in one.iter() {
                            if c2 != c1 {
                                upper_right_ref = *c2;
                            }
                        }
                    }
                }
            }

            // for c in chars.iter() {
            //     if one.contains(c) {
            //         relevance_to_one += 1;
            //     }
            // }

            if relevance_to_one != 1 || chars.len() != 5 {
                continue 'signals;
            }

            for c in upper_left_and_middle.iter() {
                if !chars.contains(c) {
                    continue 'signals;
                }
            }

            let b = chars
                .into_iter()
                .filter(|c| !four.contains(c) && !top.contains(&c))
                .next()
                .unwrap();

            bottom.push(b);
            bottom_right.push(bottom_right_ref);
            upper_right.push(upper_right_ref);

            let bottom_left_ref = match eight
                .iter()
                .filter(|c| {
                    !one.contains(c)
                        && !upper_left_and_middle.contains(c)
                        && !top.contains(c)
                        && !bottom.contains(c)
                })
                .next()
            {
                Some(data) => data,
                None => {
                    panic!("failed to get bottom left")
                }
            };

            bottom_left.push(bottom_left_ref.to_owned());
        }

        // finding 3
        'signals1: for segment in self.test_signals.iter() {
            let chars = segment.chars().collect::<Vec<char>>();
            let mut relevance_to_upper_left_and_middle = 0;

            for c in chars.iter() {
                if upper_left_and_middle.contains(&c) {
                    relevance_to_upper_left_and_middle += 1;
                }
            }

            if relevance_to_upper_left_and_middle != 1 || chars.len() != 5 {
                continue 'signals1;
            }

            for c in one.iter() {
                if !chars.contains(c) {
                    continue 'signals1;
                }
            }

            for c in top.iter() {
                if !chars.contains(c) {
                    continue 'signals1;
                }
            }

            for c in bottom.iter() {
                if !chars.contains(c) {
                    continue 'signals1;
                }
            }

            let resp = chars
                .iter()
                .filter(|c| !one.contains(c) && !top.contains(c) && !bottom.contains(c))
                .next()
                .unwrap();

            middle.push(resp.to_owned());
        }

        // println!("top: {top:?}");
        // println!("bottom: {bottom:?}");
        // println!("middle: {middle:?}");
        // println!("upper left and middle: {upper_left_and_middle:?}");

        one.sort();
        numbers.push(Number {
            value: 1,
            code: one.iter().collect::<String>().to_owned(),
        });
        let mut three = Vec::new();
        for c in one.iter() {
            three.push(c);
        }
        for c in top.iter() {
            three.push(c)
        }
        for c in bottom.iter() {
            three.push(c)
        }
        for c in middle.iter() {
            three.push(c)
        }
        three.sort();
        numbers.push(Number {
            value: 3,
            code: three.iter().map(|c| c.to_owned()).collect::<String>(),
        });

        let mut five: Vec<&char> = Vec::new();
        for c in top.iter() {
            five.push(c)
        }
        for c in upper_left_and_middle.iter() {
            five.push(c);
        }
        for c in bottom_right.iter() {
            five.push(c);
        }
        for c in bottom.iter() {
            five.push(c)
        }
        five.sort();
        numbers.push(Number {
            value: 5,
            code: five.iter().map(|c| c.to_owned()).collect::<String>(),
        });
        let mut nine: Vec<&char> = Vec::new();
        for c in top.iter() {
            nine.push(c)
        }
        for c in upper_left_and_middle.iter() {
            nine.push(c);
        }
        for c in one.iter() {
            nine.push(c);
        }
        for c in bottom.iter() {
            nine.push(c)
        }
        nine.sort();
        numbers.push(Number {
            value: 9,
            code: nine.iter().map(|c| c.to_owned()).collect::<String>(),
        });

        let mut two: Vec<&char> = Vec::new();
        for c in top.iter() {
            two.push(c)
        }
        for c in bottom.iter() {
            two.push(c)
        }
        for c in upper_right.iter() {
            two.push(c)
        }
        for c in middle.iter() {
            two.push(c)
        }
        for c in bottom_left.iter() {
            two.push(c)
        }
        two.sort();
        numbers.push(Number {
            value: 2,
            code: two.iter().map(|c| c.to_owned()).collect::<String>(),
        });

        for c in bottom_left.iter() {
            five.push(c)
        }
        five.sort();
        numbers.push(Number {
            value: 6,
            code: five.iter().map(|c| c.to_owned()).collect::<String>(),
        });

        four.sort();
        numbers.push(Number {
            value: 4,
            code: four.iter().map(|c| c.to_owned()).collect::<String>(),
        });

        seven.sort();
        numbers.push(Number {
            value: 7,
            code: seven.iter().map(|c| c.to_owned()).collect::<String>(),
        });

        eight.sort();
        numbers.push(Number {
            value: 8,
            code: eight.iter().map(|c| c.to_owned()).collect::<String>(),
        });
        for c in eight.iter() {
            // println!("{c}");
            if !middle.contains(c) {
                zero.push(c)
            }
        }
        zero.sort();
        numbers.push(Number {
            value: 0,
            code: zero.iter().map(|c| c.to_owned()).collect::<String>(),
        });
        // println!("zero: {zero:?}");
        // println!("eight: {eight:?}");

        numbers
    }

    fn interpret_output_signals(&self, numbers: Vec<Number>) -> i32 {
        let mut output = String::new();

        for signal in self.output_signals.iter() {
            let mut signal_chars = signal.chars().collect::<Vec<char>>();
            signal_chars.sort();

            for number in numbers.iter() {
                if signal_chars.iter().collect::<String>() == number.code {
                    let temp = &number.value.to_string();
                    // println!("{temp}");
                    output += temp;
                }
            }
        }

        output.parse().unwrap()
    }
}

fn main() {
    // part one lengths
    // let ref_lengths = Vec::from(UNIQUE_SEGMENT_LENGTHS);
    let data = get_seven_segment_data(FILE_PATH);

    // println!("{data:#?}");

    let mut part_two_sum = 0;
    data.into_iter().for_each(|s| {
        let nums = s.analyze_test_signals();
        let interpretted_numbers = s.interpret_output_signals(nums);
        println!("{interpretted_numbers}");
        part_two_sum += interpretted_numbers;
    });

    println!("PART TWO : {part_two_sum}")
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
