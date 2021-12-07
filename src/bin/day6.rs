use std::env;
use std::fs;

fn simulate(ages: [usize; 9], days: usize) -> [usize; 9] {
    let mut ages = ages;

    for _ in 0..days {
        let mut new_ages = [0; 9];

        for i in 0..8 {
            new_ages[i] = ages[i + 1];
        }
        new_ages[6] += ages[0];
        new_ages[8] = ages[0];

        ages = new_ages;
    }

    ages
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Usage: ./day6 <input data path>".to_string());
    }

    let path = &args[1];
    let file_contents =
        fs::read_to_string(path).map_err(|e| format!("Error reading input data: {}.", e))?;

    let ages = file_contents
        .split(",")
        .map(|s| {
            s.parse::<usize>()
                .or(Err("Unable to parse integer.".to_string()))
        })
        .collect::<Result<Vec<_>, String>>()?;

    let mut bucketed_ages = [0_usize; 9];

    for age in ages {
        bucketed_ages[age] += 1;
    }

    let final_ages = simulate(bucketed_ages.clone(), 80);
    let final_pop_size: usize = final_ages.iter().sum();

    println!("Part one: {}", final_pop_size);

    let final_ages = simulate(bucketed_ages.clone(), 256);
    let final_pop_size: usize = final_ages.iter().sum();

    println!("Part two: {}", final_pop_size);

    Ok(())
}
