use std::fs;

#[derive(Debug)]
pub struct PowerConsumptionResult {
    gamma_rate: isize,
    epsilon_rate: isize,
}

#[derive(Debug)]
pub struct LifeSupportResult {
    oxygen_generator_rating: isize,
    co2_scrubber_rating: isize,
}

pub enum LifeSupportMetric {
    OxygenGenerator,
    Co2Scrubber,
}

struct BinaryCount {
    one: usize,
    zero: usize,
}

// for each binary number in the input, find two new binary numbers (gamma, epsilon)
// gamma rate * epsilon rate = power consumption
impl From<&str> for PowerConsumptionResult {
    fn from(input_path: &str) -> Self {
        let binary_numbers = import_diagnostic_report(input_path);
        let mut binary_counts = Vec::<BinaryCount>::new();
        for _r in 0..binary_numbers[0].len() {
            binary_counts.push(BinaryCount { one: 0, zero: 0 })
        }

        for binary_number in binary_numbers {
            // https://stackoverflow.com/questions/66288515/how-do-i-get-the-index-of-the-current-element-in-a-for-loop-in-rust
            for (index, binary) in binary_number.chars().enumerate() {
                match binary {
                    '0' => binary_counts[index].zero += 1,
                    '1' => binary_counts[index].one += 1,
                    _ => (),
                }
            }
        }

        let mut gamma_rate = String::new();
        let mut epsilon_rate = String::new();
        // FIXME: the instructions do not provide a tie-breaker
        // TIE-BREAKER = 1 | referenced in part 2
        for binary_count in binary_counts {
            if binary_count.zero > binary_count.one {
                gamma_rate.push('0');
                epsilon_rate.push('1');
            } else {
                gamma_rate.push('1');
                epsilon_rate.push('0');
            }
        }

        // https://doc.rust-lang.org/std/fmt/#formatting-traits
        // relying on std library to convert binary to decimal
        PowerConsumptionResult {
            gamma_rate: binary_to_decimal_convertion(&gamma_rate),
            epsilon_rate: binary_to_decimal_convertion(&epsilon_rate),
        }
    }
}

// PART TWO
impl From<&str> for LifeSupportResult {
    fn from(input_path: &str) -> Self {
        let binary_numbers = import_diagnostic_report(input_path);

        let binary_oxygen_generator_rating = filter_diagnostic_report_by_index_and_metric(
            &binary_numbers,
            0,
            LifeSupportMetric::OxygenGenerator,
        );
        let binary_co2_scrubber_rating = filter_diagnostic_report_by_index_and_metric(
            &binary_numbers,
            0,
            LifeSupportMetric::Co2Scrubber,
        );

        LifeSupportResult {
            oxygen_generator_rating: binary_to_decimal_convertion(&binary_oxygen_generator_rating),
            co2_scrubber_rating: binary_to_decimal_convertion(&binary_co2_scrubber_rating),
        }
    }
}

impl PowerConsumptionResult {
    pub fn get_result(self) -> isize {
        self.gamma_rate * self.epsilon_rate
    }
}

impl LifeSupportResult {
    /// Oxygen Generator Rating multiplied by CO2 Scrubber Rating
    pub fn get_life_support_rating(self) -> isize {
        self.oxygen_generator_rating * self.co2_scrubber_rating
    }
}

fn import_diagnostic_report(input_path: &str) -> Vec<String> {
    fs::read_to_string(input_path)
        .expect(input_path)
        .trim()
        .split("\n")
        .map(String::from)
        .collect()
}

fn binary_to_decimal_convertion(binary_number: &str) -> isize {
    isize::from_str_radix(&binary_number, 2).expect("failed to convert binary to decimal")
}

/// OxygenGenerator = most common value
///
/// Co2Scrubber     = least common value
fn filter_diagnostic_report_by_index_and_metric(
    binary_numbers: &Vec<String>,
    index: isize,
    life_support_metric: LifeSupportMetric,
) -> String {
    // TURBOFISH ha
    let mut filtered_list = Vec::<String>::new();
    let mut binary_count = BinaryCount { zero: 0, one: 0 };

    for binary_number in binary_numbers {
        match binary_number
            .chars()
            .nth(index.unsigned_abs())
            .expect("failed to find nth index of binary diagnostic")
        {
            '0' => binary_count.zero += 1,
            '1' => binary_count.one += 1,
            _ => (),
        }
    }

    let most_common_character: char;
    if binary_count.zero > binary_count.one {
        most_common_character = '0'
    } else {
        most_common_character = '1'
    }

    match life_support_metric {
        // most common
        LifeSupportMetric::OxygenGenerator => {
            for binary_number in binary_numbers {
                if binary_number
                    .chars()
                    .nth(index.unsigned_abs())
                    .expect("failed to find nth index of binary diagnostic")
                    == most_common_character
                {
                    filtered_list.push(binary_number.to_string())
                }
            }
        }
        // least common
        LifeSupportMetric::Co2Scrubber => {
            for binary_number in binary_numbers {
                if binary_number
                    .chars()
                    .nth(index.unsigned_abs())
                    .expect("failed to find nth index of binary diagnostic")
                    != most_common_character
                {
                    filtered_list.push(binary_number.to_string())
                }
            }
        }
    }

    if filtered_list.len() == 1 {
        filtered_list[0].clone()
    } else {
        // spent ... 30 minutes "debugging" a stackoverflow error to discover I wasn't incrementing the index 😤
        filter_diagnostic_report_by_index_and_metric(&filtered_list, index + 1, life_support_metric)
    }
}

// PART TWO
// life support rating = oxygen generator rating * CO2 scrubber rating
// tie breaker = 1

// let mut binary_counts = Vec::<BinaryCount>::new();
// for _r in 0..binary_numbers[0].len() {
//     binary_counts.push(BinaryCount { one: 0, zero: 0 })
// }

// for binary_number in binary_numbers {
//     for (index, binary) in binary_number.chars().enumerate() {
//         match binary {
//             '0' => binary_counts[index].zero += 1,
//             '1' => binary_counts[index].one += 1,
//             _ => (),
//         }
//     }
// }

// let mut oxygen_generator_rate = String::new();
// let mut co2_scrubber_rate = String::new();
// for binary_count in binary_counts {
//     if binary_count.zero > binary_count.one {
//         oxygen_generator_rate.push('0');
//         co2_scrubber_rate.push('1');
//     } else {
//         oxygen_generator_rate.push('1');
//         co2_scrubber_rate.push('0');
//     }
// }

// LifeSupportResult {
//     oxygen_generator_rating: binary_to_decimal_convertion(&oxygen_generator_rate),
//     co2_scrubber_rating: binary_to_decimal_convertion(&co2_scrubber_rate),
// }
