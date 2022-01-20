// https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
use std::{fs, process::exit};

/// Convert line delimited list of numbers to vector of numbers
pub fn path_to_veci32(input_path: &str) -> Vec<i32> {
    // https://doc.rust-lang.org/book/ch12-02-reading-a-file.htmlx
    fs::read_to_string(input_path)
        .expect(input_path)
        // automatically trims end of file whitespace as opposed to .split("\n")
        .split_whitespace()
        // https://stackoverflow.com/questions/62690352/how-to-return-a-vector-of-strings-in-rust
        // moving data from stack frame to heap and passing pointer out of function
        // skipping this step results in a compiler error because the owner of the Vec<&str> falls out of scope!
        // https://doc.rust-lang.org/rust-by-example/fn/closures.html
        // https://mkaz.blog/working-with-rust/numbers/#:~:text=String%20to%20Integer,parse().
        // [amature take] .expect is like .unwrap but with a little message if it panics
        .map(|x| str::parse::<i32>(x).expect("failed to unwrap"))
        // collected type is inferred from function return
        .collect()
}

/// Measure number of positive increments from provided vector of numbers
pub fn measure_increases(measurements: &Vec<i32>) -> i32 {
    // edge protection
    if measurements.len() == 0 {
        println!("input file empty");
        exit(0);
    }

    // framed variable for adding up increased measurements
    let mut measurement_increases = 0;
    // framed variable for tracking last scanned measurement
    let mut previous_measurement = measurements[0];

    for measurement in measurements {
        if measurement > &previous_measurement {
            measurement_increases += 1;
        }
        // borrow reference to current measurement
        previous_measurement = *measurement;
    }

    measurement_increases
}

// PART 2
/// find average of values in a sliding window of specified size
pub fn convert_measurements_to_sliding_window(
    measurements: &Vec<i32>,
    window_size: usize,
) -> Vec<i32> {
    // https://dev.to/brunooliveira/learning-rust-understanding-vectors-2ep4
    let mut windowed_measurements = Vec::new();

    // suboptimal sliding window solution?
    for i in 0..measurements.len() {
        // "Stop when there aren't enough measurements left to create a new three-measurement sum."
        if i + window_size > measurements.len() { break }

        // add to-be referenced index in the returned vector
        windowed_measurements.push(0);
        for n in i..i + window_size {
            windowed_measurements[i] += measurements[n]
        }
    }

    windowed_measurements
}
