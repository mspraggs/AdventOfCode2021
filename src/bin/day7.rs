use std::collections::HashMap;
use std::env;
use std::fs;

const GOLDEN_RATIO: f64 = 1.618033988749895;

fn parse_data(data: &str) -> Result<HashMap<i32, i32>, String> {
    let positions = data
        .split(',')
        .map(|s| {
            s.parse::<i32>()
                .or(Err("Unable to parse integer.".to_string()))
        })
        .collect::<Result<Vec<_>, String>>()?;

    let mut bucketed_positions = HashMap::new();

    for p in positions {
        let entry: &mut i32 = bucketed_positions.entry(p).or_default();
        *entry += 1;
    }
    Ok(bucketed_positions)
}

fn compute_fuel_const(positions: &HashMap<i32, i32>, position: f64) -> f64 {
    positions
        .iter()
        .map(|(&k, &v)| (k as f64 - position).abs() * v as f64)
        .sum()
}

fn compute_fuel_monotonic(positions: &HashMap<i32, i32>, position: f64) -> f64 {
    positions
        .iter()
        .map(|(&k, &v)| {
            let num_steps = (k as f64 - position).abs();
            let usage = num_steps * (num_steps + 1.0) / 2.0;
            usage * v as f64
        })
        .sum()
}

fn compute_optimal_fuel_usage(
    positions: &HashMap<i32, i32>,
    compute_fuel: fn(&HashMap<i32, i32>, f64) -> f64,
) -> Result<(i32, i32), String> {
    let min = positions
        .keys()
        .copied()
        .min()
        .ok_or("Input data empty.".to_string())?;
    let max = positions.keys().copied().max().unwrap();

    let mut lower = min as f64;
    let mut upper = max as f64;

    while (upper - lower).abs() > f32::EPSILON as f64 {
        let offset = (upper - lower) / GOLDEN_RATIO;
        let new_lower = upper - offset;
        let new_upper = lower + offset;
        let new_fuel_lower = compute_fuel(&positions, new_lower);
        let new_fuel_upper = compute_fuel(&positions, new_upper);

        if new_fuel_lower < new_fuel_upper {
            upper = new_upper;
        } else {
            lower = new_lower;
        }
    }

    Ok((lower.round() as i32, compute_fuel(&positions, lower.round()) as i32))
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format!("Usage: {} <input data path>", args[0]));
    }

    let path = &args[1];
    let file_contents =
        fs::read_to_string(path).map_err(|e| format!("Error reading input data: {}.", e))?;

    let positions = parse_data(&file_contents)?;

    let (_, fuel) = compute_optimal_fuel_usage(&positions, compute_fuel_const)?;
    println!("Part one: {}", fuel);

    let (_, fuel) = compute_optimal_fuel_usage(&positions, compute_fuel_monotonic)?;
    println!("Part two: {}", fuel);

    Ok(())
}
