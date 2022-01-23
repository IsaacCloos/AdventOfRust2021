use std::fs;

#[derive(Debug)]
pub struct DiagnosticReportResult {
    gamma_rate: isize,
    epsilon_rate: isize,
}

struct BinaryCount {
    one: usize,
    zero: usize,
}

// for each binary number in the input, find two new binary numbers (gamma, epsilon)
// gamma rate * epsilon rate = power consumption
impl From<&str> for DiagnosticReportResult {
    fn from(input_path: &str) -> Self {
        let binary_numbers = DiagnosticReportResult::import_diagnostic_report(input_path);
        let mut binary_counts = Vec::<BinaryCount>::new();
        let mut gamma_rate = String::new();
        let mut epsilon_rate = String::new();
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

        // FIXME: the instructions do not provide a tie-breaker
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
        DiagnosticReportResult {
            gamma_rate: isize::from_str_radix(&gamma_rate, 2).expect("failed to convert gamma_rate binary to decimal"),
            epsilon_rate: isize::from_str_radix(&epsilon_rate, 2).expect("failed to convert gamma_rate binary to decimal")
        }
    }
}

impl DiagnosticReportResult {
    pub fn get_power_consumption(self) -> isize {
        self.gamma_rate * self.epsilon_rate
    }

    fn import_diagnostic_report(input_path: &str) -> Vec<String> {
        fs::read_to_string(input_path)
            .expect(input_path)
            .trim()
            .split("\n")
            .map(String::from)
            .collect()
    }

}
