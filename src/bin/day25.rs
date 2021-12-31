use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::ops::{Index, IndexMut};

use aoc2021::error::Error;

fn parse_input(data: &str) -> Result<Map, Box<dyn error::Error>> {
    let height = data.lines().count();
    let width = data
        .lines()
        .map(|l| l.len())
        .max()
        .ok_or_else(|| Error("Input is empty.".to_owned()))?;

    Ok(Map::new(
        data.chars().filter(|&c| c != '\n').collect(),
        width,
        height,
    ))
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Map {
    cells: Vec<char>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(cells: Vec<char>, width: usize, height: usize) -> Self {
        Self {
            cells,
            width,
            height,
        }
    }

    fn compute_offset(&self, index: [usize; 2]) -> usize {
        let mut index = index;
        if index[0] >= self.height {
            index[0] -= self.height;
        }
        if index[1] >= self.width {
            index[1] -= self.width;
        }
        index[0] * self.width + index[1]
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                write!(f, "{}", self[[i, j]])?;
            }
            if i < self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Index<[usize; 2]> for Map {
    type Output = char;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.cells[self.compute_offset(index)]
    }
}

impl IndexMut<[usize; 2]> for Map {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let offset = self.compute_offset(index);
        &mut self.cells[offset]
    }
}

struct HerdIterSpec {
    symbol: char,
    make_coord: fn(usize, usize) -> [usize; 2],
    next_coord: fn([usize; 2]) -> [usize; 2],
    inner_max: usize,
    outer_max: usize,
}

fn step(map: &mut Map) -> usize {
    let mut num_moves = 0;

    let herd_iter_specs: [HerdIterSpec; 2] = [
        HerdIterSpec {
            symbol: '>',
            make_coord: |i, j| [i, j],
            next_coord: |[i, j]| [i, j + 1],
            inner_max: map.width,
            outer_max: map.height,
        },
        HerdIterSpec {
            symbol: 'v',
            make_coord: |i, j| [j, i],
            next_coord: |[i, j]| [i + 1, j],
            inner_max: map.height,
            outer_max: map.width,
        },
    ];

    let mut vacated_coords = Vec::with_capacity(map.width * map.height);

    for spec in herd_iter_specs {
        vacated_coords.clear();

        for i in 0..spec.outer_max {
            let mut iter = 0..spec.inner_max;
            while let Some(j) = iter.next() {
                let coords = (spec.make_coord)(i, j);
                if map[coords] != spec.symbol {
                    continue;
                }

                let new_coords = (spec.next_coord)(coords);
                if map[new_coords] == '.' {
                    num_moves += 1;
                    map[new_coords] = map[coords];
                    vacated_coords.push(coords);
                    iter.next();
                }
            }
        }

        for coords in vacated_coords.iter().copied() {
            map[coords] = '.';
        }
    }

    num_moves
}

fn simulate(init: Map) -> usize {
    let mut map = init;
    let mut step_count = 0;

    while step(&mut map) > 0 {
        step_count += 1;
    }

    step_count + 1
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

    let map = parse_input(&file_contents)?;

    println!("Part one: {}", simulate(map));

    Ok(())
}
