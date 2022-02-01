use core::fmt;
use std::{collections::HashMap, fs, str::FromStr};

const INPUT_PATH: &str = "input.txt";

struct IdealCrabPositionResult {
    position: i32,
    fuel_cost: i32,
}

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
} // position 2 | fuel 37

fn get_ideal_crab_position(crab_arrangement: Vec<i32>) -> IdealCrabPositionResult {
    let mut fuel_costs: HashMap<i32, i32> = HashMap::with_capacity(crab_arrangement.len());
    for ref_crab in &crab_arrangement {
        if fuel_costs.contains_key(&ref_crab) {
            println!("skipping {ref_crab}");
            continue;
        }
        println!("{ref_crab}");
        for target_crab in &crab_arrangement {
            let active_distance = (ref_crab - target_crab).abs();

            // *fuel_costs.get_mut(&ref_crab).unwrap() += active_distance;

            if fuel_costs.contains_key(&ref_crab) {
                *fuel_costs
                    .get_mut(&ref_crab)
                    .expect("Failed to get mapped id") += active_distance;
            } else {
                fuel_costs.insert(*ref_crab, active_distance);
            }
        }
    }
    let mut hash_vec: Vec<(&i32, &i32)> = fuel_costs.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));
    // fuel_costs.sort_by(|a, b| a.cmp(b));

    IdealCrabPositionResult {
        position: *hash_vec.last().unwrap().0,
        fuel_cost: *hash_vec.last().unwrap().1,
    }
}

fn get_crab_arrangement_input<T: FromStr>(input_path: &str) -> Result<Vec<T>, T::Err> {
    fs::read_to_string(input_path)
        .expect("Failed to parse input file")
        .trim()
        .split(",")
        .map(|x| str::parse::<T>(x))
        .collect()
}
