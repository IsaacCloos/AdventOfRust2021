use crate::diagnostic_tools::{PowerConsumptionResult, LifeSupportResult};

mod diagnostic_tools;

const INPUT_PATH: &str = "input.txt";

fn main() {
    let power_consumption_results = PowerConsumptionResult::from(INPUT_PATH);

    println!("PART ONE: {}", power_consumption_results.get_result());

    let life_support_results = LifeSupportResult::from(INPUT_PATH);

    println!("PART TWO: {}", life_support_results.get_life_support_rating());
}
