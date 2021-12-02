use std::env;
use std::fs;

trait CommandProcessor {
    fn process(&mut self, direction: &str, distance: i32) -> Result<(), String>;
}

#[derive(Debug, Default)]
struct BasicProcessor {
    horizontal: i32,
    depth: i32,
}

impl CommandProcessor for BasicProcessor {
    fn process(&mut self, direction: &str, distance: i32) -> Result<(), String> {
        match direction {
            "forward" => self.horizontal += distance,
            "down" => self.depth += distance,
            "up" => self.depth -= distance,
            _ => {
                return Err("Unknown direction".to_string());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct AimProcessor {
    aim: i32,
    depth: i32,
    horizontal: i32,
}

impl CommandProcessor for AimProcessor {
    fn process(&mut self, direction: &str, distance: i32) -> Result<(), String> {
        match direction {
            "forward" => {
                self.horizontal += distance;
                self.depth += self.aim * distance;
            }
            "down" => self.aim += distance,
            "up" => self.aim -= distance,
            _ => {
                return Err("Unknown direction".to_string());
            }
        }
        Ok(())
    }
}

fn parse_line(line: &str) -> Result<(String, i32), ()> {
    let mut iter = line.split_ascii_whitespace();
    let direction = if let Some(d) = iter.next() {
        d
    } else {
        return Err(());
    };
    let distance = if let Some(d) = iter.next() {
        if let Ok(i) = d.parse::<i32>() {
            i
        } else {
            return Err(());
        }
    } else {
        return Err(());
    };

    Ok((direction.to_string(), distance))
}

fn calculate_depth_and_distance<T: CommandProcessor + Default>(data: &str) -> Result<T, String> {
    let mut processor = T::default();

    for line in data.lines() {
        let (direction, distance) = if let Ok(movement) = parse_line(line) {
            movement
        } else {
            return Err("Error parsing input file.".to_string());
        };

        processor.process(&direction, distance)?;
    }

    Ok(processor)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ./day2 <input data path>");
        return;
    }

    let path = &args[1];
    let file_contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading input data: {}", e);
            return;
        }
    };

    match calculate_depth_and_distance::<BasicProcessor>(&file_contents) {
        Ok(p) => {
            println!("Part one: {}", p.horizontal * p.depth)
        }
        Err(msg) => {
            eprintln!("{}", msg);
            return;
        }
    }

    match calculate_depth_and_distance::<AimProcessor>(&file_contents) {
        Ok(p) => {
            println!("Part two: {}", p.horizontal * p.depth)
        }
        Err(msg) => {
            eprintln!("{}", msg);
            return;
        }
    }
}
