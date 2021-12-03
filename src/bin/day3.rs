use std::env;
use std::fs;

fn parse_input(data: &str) -> Result<(Vec<u16>, usize), String> {
    let num_bits = if let Some(l) = data.lines().next() {
        l.len()
    } else {
        return Err("Input file is empty.".to_string());
    };
    let values = data
        .lines()
        .map(|l| {
            u16::from_str_radix(l, 2).map_err(|_| format!("Error parsing input line '{}'.", l))
        })
        .collect::<Result<Vec<u16>, String>>();

    values.map(|v| (v, num_bits))
}

fn calculate_bit_ratios(values: &[u16], num_bits: usize) -> Result<Vec<f64>, String> {
    let mut counts = vec![0; num_bits];

    for &value in values {
        for i in 0..num_bits {
            if ((1 << i) & value) > 0 {
                counts[i] += 1;
            }
        }
    }

    let mut ratios = vec![0.0; num_bits];
    for i in 0..num_bits {
        ratios[i] = counts[i] as f64 / values.len() as f64;
    }

    Ok(ratios)
}

fn calculate_rate(values: &[u16], num_bits: usize, filter: fn(f64) -> bool) -> Result<u16, String> {
    let ratios = calculate_bit_ratios(values, num_bits)?;
    let mut rate = 0u16;

    for (i, &r) in ratios.iter().enumerate() {
        if filter(r) {
            rate |= 1 << i;
        }
    }

    Ok(rate)
}

fn calcuate_rating(
    values: &[u16],
    num_bits: usize,
    filter: fn(f64) -> bool,
) -> Result<u16, String> {
    let mut filtered_numbers = values.to_vec();

    for i in (0..num_bits).rev() {
        let ratios = calculate_bit_ratios(&filtered_numbers, num_bits)?;
        let mask = 1 << i;
        let expected = if filter(ratios[i]) { mask } else { 0 };

        filtered_numbers = filtered_numbers
            .iter()
            .filter_map(|&v| if v & mask == expected { Some(v) } else { None })
            .collect();

        if filtered_numbers.len() == 1 {
            return Ok(filtered_numbers[0]);
        }
    }

    Err("Multiple numbers remaining after filterering.".to_string())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Usage: ./day3 <input data path>".to_string());
    }

    let path = &args[1];
    let file_contents =
        fs::read_to_string(path).map_err(|e| format!("Error reading input data: {}.", e))?;

    let (values, num_bits) = parse_input(&file_contents)?;

    let gamma_rate = calculate_rate(&values, num_bits, |r| r > 0.5)?;
    let epsilon_rate = calculate_rate(&values, num_bits, |r| r <= 0.5)?;

    println!("Part one: {}", gamma_rate as u32 * epsilon_rate as u32);

    let o2_gen_rating = calcuate_rating(&values, num_bits, |r| r >= 0.5)?;
    let co2_scrub_rating = calcuate_rating(&values, num_bits, |r| r < 0.5)?;

    println!(
        "Part two: {}",
        o2_gen_rating as u32 * co2_scrub_rating as u32
    );

    Ok(())
}
