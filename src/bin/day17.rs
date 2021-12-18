use std::env;
use std::error;
use std::fs;

use aoc2021::error::Error;

type Point = (i32, i32);

#[derive(Debug, Default, Copy, Clone)]
struct Rectangle {
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
}

impl Rectangle {
    fn new(point0: Point, point1: Point) -> Self {
        let x_bounds = if point0.0 < point1.0 {
            (point0.0, point1.0)
        } else {
            (point1.0, point0.0)
        };
        let y_bounds = if point0.1 < point1.1 {
            (point0.1, point1.1)
        } else {
            (point1.1, point0.1)
        };
        Self { x_bounds, y_bounds }
    }

    fn point_displacement(&self, point: Point) -> Point {
        (
            if point.0 < self.x_bounds.0 {
                point.0 - self.x_bounds.0
            } else if point.0 > self.x_bounds.1 {
                point.0 - self.x_bounds.1
            } else {
                0
            },
            if point.1 < self.y_bounds.0 {
                point.1 - self.y_bounds.0
            } else if point.1 > self.y_bounds.1 {
                point.1 - self.y_bounds.1
            } else {
                0
            },
        )
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Probe {
    position: Point,
    velocity: Point,
}

impl Probe {
    fn new(position: Point, velocity: Point) -> Self {
        Self { position, velocity }
    }

    fn step(&mut self) -> Point {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.velocity.0 += if self.velocity.0 > 0 {
            -1
        } else if self.velocity.0 < 0 {
            1
        } else {
            0
        };
        self.velocity.1 -= 1;
        self.position
    }
}

fn simulate_probe(target: &Rectangle, init_velocity: Point, max_steps: usize) -> Option<i32> {
    let mut probe = Probe::new((0, 0), init_velocity);
    let mut max_height = 0;

    for _ in 0..max_steps {
        let pos = probe.step();
        if pos.1 > max_height {
            max_height = pos.1;
        }
        let displacement = target.point_displacement(pos);
        if displacement == (0, 0) {
            return Some(max_height);
        }
        if displacement.1 < -10 {
            return None;
        }
        if displacement.0 > 10 {
            return None;
        }
    }

    None
}

fn parse_input(data: &str) -> Result<Rectangle, Box<dyn error::Error>> {
    let stripped_data = data
        .strip_prefix("target area: ")
        .ok_or_else(|| Error("Unable to parse input.".to_string()))?;

    let mut values = [0; 4];
    let prefixes = ["x=", "y="];

    let mut split_iter = stripped_data.split(", ");
    for (i, prefix) in prefixes.iter().enumerate() {
        let coord_data = split_iter
            .next()
            .ok_or(Error("Unable to parse input.".to_string()))?;
        let mut coord_value_iter = coord_data
            .strip_prefix(*prefix)
            .map(|s| s.split(".."))
            .ok_or_else(|| Error("Unable to parse input.".to_string()))?;
        for j in 0..2 {
            let coord_value = coord_value_iter
                .next()
                .and_then(|s| s.parse::<i32>().ok())
                .ok_or_else(|| Error("Unable to parse value.".to_string()))?;
            values[i * 2 + j] = coord_value;
        }
    }

    Ok(Rectangle::new(
        (values[0], values[2]),
        (values[1], values[3]),
    ))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Box::new(Error(format!(
            "Usage: {} <input data path>",
            args[0]
        ))));
    }

    let path = &args[1];
    let file_contents =
        fs::read_to_string(path).map_err(|e| format!("Error reading input data: {}.", e))?;

    let target_area = parse_input(&file_contents)?;

    let mut max_height = 0;
    let mut velocity_count = 0;
    let max_x_vel = target_area.x_bounds.1 * 2;
    let min_y_vel = target_area.y_bounds.1 * 2;
    for x_vel in 0..max_x_vel {
        for y_vel in min_y_vel..1000 {
            if let Some(height) = simulate_probe(&target_area, (x_vel, y_vel), 10000) {
                velocity_count += 1;
                if height > max_height {
                    max_height = height;
                }
            }
        }
    }

    println!("Part one: {}", max_height);
    println!("Part two: {}", velocity_count);

    Ok(())
}
