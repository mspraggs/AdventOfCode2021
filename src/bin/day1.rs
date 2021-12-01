use std::env;
use std::fs;

fn count_depth_changes(data: &[Option<i32>], offset: usize) -> Result<i32, String> {
    let mut count = 0;

    for (i, datum) in data.iter().enumerate() {
        if i < offset {
            continue;
        }
        let prev_depth = data[i - offset].unwrap();
        if let Some(depth) = datum {
            if *depth > prev_depth {
                count += 1;
            }
        } else {
            return Err("Unable to parse data.".to_string());
        }
    }

    Ok(count)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ./day1 <input file path>");
        return;
    }

    let data_path = &args[1];

    let file_contents = if let Ok(contents) = fs::read_to_string(data_path) {
        contents
    } else {
        eprintln!("Unable to open data file.");
        return;
    };

    let depths: Vec<Option<i32>> = file_contents
        .as_str()
        .lines()
        .map(|l| l.parse::<i32>().ok())
        .collect();

    let part1_count = match count_depth_changes(&depths, 1) {
        Ok(c) => c,
        Err(msg) => {
            eprintln!("{}", msg);
            return;
        }
    };
    let part2_count = match count_depth_changes(&depths, 3) {
        Ok(c) => c,
        Err(msg) => {
            eprintln!("{}", msg);
            return;
        }
    };

    println!("Part 1 result: {}", part1_count);
    println!("Part 2 result: {}", part2_count);
}
