use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::error;
use std::fmt;
use std::fs;

use aoc2021::error::Error;

#[derive(Copy, Clone, Eq, PartialEq)]
struct QueueItem {
    risk: u32,
    coords: (usize, usize),
}

impl QueueItem {
    fn new(risk: u32, coords: (usize, usize)) -> Self {
        Self { risk, coords }
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.coords.cmp(&other.coords))
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default, Clone)]
struct Map {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(data: Vec<u32>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    fn find_path(&self, start: (usize, usize)) -> u32 {
        let target = (self.width - 1, self.height - 1);
        let mut total_risk = 0;
        let mut heap = BinaryHeap::new();
        let mut total_risks = vec![u32::MAX; self.data.len()];
        heap.push(QueueItem::new(0, start));

        while let Some(QueueItem { risk, coords }) = heap.pop() {
            if coords == target {
                total_risk = risk;
                break;
            }

            for (neighb_coords, neighb_risk) in self.get_neighbour_risks(coords.0, coords.1) {
                let next = QueueItem::new(risk + neighb_risk, neighb_coords);
                let next_offset = next.coords.0 * self.width + next.coords.1;

                if next.risk < total_risks[next_offset] {
                    heap.push(next);
                    total_risks[next_offset] = next.risk;
                }
            }
        }

        total_risk
    }

    fn get_neighbour_risks(&self, row: usize, col: usize) -> Vec<((usize, usize), u32)> {
        let mut neighbours = Vec::with_capacity(4);

        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (i, j) in &offsets {
            if let Some(coords) = self.get_offset(row as i32 + *i, col as i32 + *j) {
                let index = self.width * coords.0 + coords.1;
                neighbours.push((coords, self.data[index]));
            }
        }

        neighbours
    }

    fn get_offset(&self, row: i32, col: i32) -> Option<(usize, usize)> {
        if row < 0 || row >= self.height as i32 {
            return None;
        }
        if col < 0 || col >= self.width as i32 {
            return None;
        }
        Some((row as usize, col as usize))
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let offset = i * self.width + j;
                write!(f, "{}", self.data[offset])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_small_map(data: &str) -> Result<Map, Box<dyn error::Error>> {
    let mut risk_levels = Vec::with_capacity(data.len());
    let height = data.lines().count();
    let mut width = 0;

    for line in data.lines() {
        width = line.len();

        let line_risk_levels = line
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .ok_or_else(|| Error(format!("Unable to parse digit '{}'.", c)))
            })
            .collect::<Result<Vec<_>, _>>()?;

        risk_levels.extend_from_slice(&line_risk_levels);
    }

    Ok(Map::new(risk_levels, width, height))
}

fn _roll_risk_value(init: u32, offset: u32) -> u32 {
    let mut rolled = init + offset;
    if rolled > 9 {
        rolled -= 9;
    }
    rolled
}

fn parse_large_map(data: &str) -> Result<Map, Box<dyn error::Error>> {
    let mut risk_levels = Vec::with_capacity(data.len() * 25);
    let small_height = data.lines().count();
    let height = small_height * 5;
    let mut width = 0;

    for line in data.lines() {
        width = line.len() * 5;

        let mut line_risk_levels = line
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .ok_or_else(|| Error(format!("Unable to parse digit '{}'.", c)))
            })
            .collect::<Result<Vec<_>, _>>()?;
        risk_levels.extend_from_slice(&line_risk_levels);

        for _ in 1..5 {
            line_risk_levels
                .iter_mut()
                .for_each(|v| *v = _roll_risk_value(*v, 1));
            risk_levels.extend_from_slice(&line_risk_levels);
        }
    }

    for i in 1..5 {
        for row in 0..small_height {
            let row_risks = risk_levels[width * row..width * (row + 1)]
                .iter()
                .map(|e| _roll_risk_value(*e, i))
                .collect::<Vec<_>>();
            risk_levels.extend_from_slice(&row_risks);
        }
    }

    Ok(Map::new(risk_levels, width, height))
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

    let map = parse_small_map(&file_contents)?;
    let total_risk = map.find_path((0, 0));
    println!("Part one: {}", total_risk);

    let map = parse_large_map(&file_contents)?;
    let total_risk = map.find_path((0, 0));
    println!("Part one: {}", total_risk);

    Ok(())
}
