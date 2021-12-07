use std::collections::HashMap;
use std::env;
use std::fs;

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

fn compute_fuel_const(positions: &HashMap<i32, i32>, position: i32) -> i32 {
    positions
        .iter()
        .map(|(&k, &v)| (k - position).abs() * v)
        .sum()
}

fn compute_fuel_monotonic(positions: &HashMap<i32, i32>, position: i32) -> i32 {
    positions
        .iter()
        .map(|(&k, &v)| {
            let num_steps = (k - position).abs();
            let usage = num_steps * (num_steps + 1) / 2;
            usage * v
        })
        .sum()
}

fn compute_optimal_fuel_usage(
    positions: &HashMap<i32, i32>,
    compute_fuel: fn(&HashMap<i32, i32>, i32) -> i32,
) -> Result<(i32, i32), String> {
    let min = positions
        .keys()
        .copied()
        .min()
        .ok_or("Input data empty.".to_string())?;
    let max = positions.keys().copied().max().unwrap();

    let mut min_pos = min;
    let mut min_fuel = compute_fuel(&positions, min_pos);

    for i in (min + 1)..(max + 1) {
        let fuel = compute_fuel(&positions, i);
        if fuel < min_fuel {
            min_pos = i;
            min_fuel = fuel;
        }
    }

    Ok((min_pos, min_fuel))
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
