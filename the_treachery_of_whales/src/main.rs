// sub optimal computation
use core::fmt;
use std::{collections::HashMap, fs, str::FromStr};

const INPUT_PATH: &str = "input.txt";

struct IdealCrabPositionResult {
    position: i32,
    fuel_cost: i32,
}

// https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
impl fmt::Display for IdealCrabPositionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Position: {position} \nFuel Cost: {fuel_cost}",
            position = self.position,
            fuel_cost = self.fuel_cost
        )
    }
}

fn main() {
    match get_crab_arrangement_input::<i32>(INPUT_PATH) {
        Ok(crab_arrangement) => {
            let ideal_crab_position = get_ideal_crab_position(crab_arrangement);

            println!("{ideal_crab_position}")
        }
        Err(_) => panic!("failed to parse"),
    }
}

fn get_ideal_crab_position(crab_arrangement: Vec<i32>) -> IdealCrabPositionResult {
    let low = *crab_arrangement.iter().min().unwrap();
    let high = *crab_arrangement.iter().max().unwrap();
    println!("{low} | {high}");
    let mut fuel_costs: HashMap<i32, i32> = HashMap::new();
    for ref_crab in &crab_arrangement {
        for target_position in low..=high {
            let active_distance = (ref_crab - target_position).abs();
            let mut fuel_cost = 0;
            for x in 1..=active_distance {
                fuel_cost += x;
            }
            if fuel_costs.contains_key(&target_position) {
                *fuel_costs
                    .get_mut(&target_position)
                    .expect("Failed to get mapped id") += fuel_cost;
            } else {
                fuel_costs.insert(target_position, fuel_cost);
            }
        }
    }
    let mut hash_vec: Vec<(&i32, &i32)> = fuel_costs.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));
    // let result = fuel_costs.iter().min().unwrap();
    IdealCrabPositionResult {
        position: *hash_vec.last().unwrap().0,
        fuel_cost: *hash_vec.last().unwrap().1,
    }
}

// https://doc.rust-lang.org/book/ch10-01-syntax.html
// according to Rust for Rustaceans generic types are compiled to all referenced versions of the function.
// in this case there is only one reference calling for an i32 flavor.
// The binaries for this application only contain one variant.
fn get_crab_arrangement_input<T: FromStr>(input_path: &str) -> Result<Vec<T>, T::Err> {
    fs::read_to_string(input_path)
        .expect("Failed to parse input file")
        .trim()
        .split(",")
        .map(|x| str::parse::<T>(x))
        .collect()
}
