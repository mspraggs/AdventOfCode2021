use std::collections::VecDeque;
use std::env;
use std::fmt;
use std::fs;

#[derive(Debug, Default, Clone)]
struct OctopusGrid {
    energies: Vec<u8>,
    width: usize,
    height: usize,
}

impl OctopusGrid {
    fn new(energies: Vec<u8>, width: usize, height: usize) -> Self {
        Self {
            energies,
            width,
            height,
        }
    }

    fn size(&self) -> usize {
        self.width * self.height
    }

    fn step(&mut self) -> usize {
        for e in self.energies.iter_mut() {
            *e += 1;
        }

        let mut flashed_octopodes = vec![false; self.energies.len()];
        let mut flash_queue = VecDeque::new();

        for i in 0..self.height {
            for j in 0..self.width {
                let offset = i * self.width + j;
                if self.energies[offset] > 9 {
                    flash_queue.push_back((i, j));
                }
            }
        }

        while let Some((i, j)) = flash_queue.pop_back() {
            let offset = i * self.width + j;

            if self.energies[offset] <= 9 || flashed_octopodes[offset] {
                continue;
            }

            self.energies[offset] = 0;
            flashed_octopodes[offset] = true;

            let neighbour_coords = self.get_neighbour_coords(i, j);
            for (i, j) in &neighbour_coords {
                let offset = *i * self.width + *j;
                if !flashed_octopodes[offset] {
                    self.energies[offset] += 1;
                }
                if self.energies[offset] > 9 {
                    flash_queue.push_back((*i, *j));
                }
            }
        }

        flashed_octopodes.iter().filter(|v| **v).count()
    }

    fn get_neighbour_coords(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::with_capacity(8);

        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }

                let (r, c) = (row as i32 + i, col as i32 + j);

                if r < 0 || r >= self.height as i32 {
                    continue;
                }
                if c < 0 || c >= self.width as i32 {
                    continue;
                }
                neighbours.push((r as usize, c as usize));
            }
        }

        neighbours
    }
}

impl fmt::Display for OctopusGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let offset = i * self.width + j;
                write!(f, "{}", self.energies[offset])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn parse_input(data: &str) -> Result<OctopusGrid, String> {
    let height = data.lines().count();
    let mut width = 0;
    let mut energies = Vec::new();

    for line in data.lines() {
        let row_energies = line.chars().map(|c| {
            c.to_digit(10)
                .map(|i| i as u8)
                .ok_or(format!("Unable to parse character to integer '{}'.", c))
        }).collect::<Result<Vec<_>, _>>()?;
        width = row_energies.len();
        energies.extend_from_slice(&row_energies);
    }

    Ok(OctopusGrid::new(energies, width, height))
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format!("Usage: {} <input data path>", args[0]));
    }

    let path = &args[1];
    let file_contents =
        fs::read_to_string(path).map_err(|e| format!("Error reading input data: {}.", e))?;

    let mut grid = parse_input(&file_contents)?;
    let mut flash_count = 0;
    for _ in 0..100 {
        flash_count += grid.step();
    }
    println!("Part one: {}", flash_count);

    let mut grid = parse_input(&file_contents)?;
    let mut step_count = 0;
    loop {
        let flash_count = grid.step();
        step_count += 1;
        if flash_count == grid.size() {
            break;
        }
    }

    println!("Part two: {}", step_count);

    Ok(())
}
