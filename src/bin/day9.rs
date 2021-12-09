use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;

struct HeatMap {
    data: Vec<u8>,
    num_rows: usize,
    num_cols: usize,
}

impl HeatMap {
    fn new(data: Vec<u8>, num_rows: usize, num_cols: usize) -> Self {
        Self {
            data,
            num_rows,
            num_cols,
        }
    }

    fn find_minima(&self) -> Vec<((usize, usize), u8)> {
        let mut values = Vec::with_capacity(self.data.len());
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                if let Some(offset) = self.get_offset(i as i32, j as i32) {
                    let neighbour_values = self.get_adjacent_values(i, j);
                    let value = self.data[offset.0 * self.num_cols + offset.1];

                    if neighbour_values.iter().all(|&((_, _), v)| v > value) {
                        values.push(((i, j), value));
                    }
                }
            }
        }

        values
    }

    fn find_basin_sizes(&self) -> Vec<usize> {
        let minima = self.find_minima();
        let mut basin_sizes = minima
            .iter()
            .map(|((r, c), h)| self.find_basin_size(*r, *c, *h))
            .collect::<Vec<usize>>();
        basin_sizes.sort();
        basin_sizes
    }

    fn find_basin_size(&self, row: usize, col: usize, height: u8) -> usize {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(((row, col), height));

        while let Some(((r, c), h)) = queue.pop_front() {
            if visited.contains(&(r, c)) || h == 9{
                continue;
            }

            let adjacent_values = self.get_adjacent_values(r, c);

            for ((row, col), height) in &adjacent_values {
                if visited.contains(&(*row, *col)) {
                    continue;
                }

                queue.push_back(((*row, *col), *height));
            }

            visited.insert((r, c));
        }

        visited.len()
    }

    fn get_adjacent_values(&self, row: usize, col: usize) -> Vec<((usize, usize), u8)> {
        let mut values = Vec::with_capacity(8);

        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (i, j) in &offsets {
            if let Some(offset) = self.get_offset(row as i32 + i, col as i32 + j) {
                values.push((offset, self.data[offset.0 * self.num_cols + offset.1]));
            }
        }

        values
    }

    fn get_offset(&self, row: i32, col: i32) -> Option<(usize, usize)> {
        if row < 0 || row >= self.num_rows as i32 {
            return None;
        }
        if col < 0 || col >= self.num_cols as i32 {
            return None;
        }
        Some((row as usize, col as usize))
    }
}

fn parse_input(raw_data: &str) -> Result<HeatMap, String> {
    let mut data = Vec::new();

    let line_count = raw_data.lines().count();
    let mut line_length = 0;

    for line in raw_data.lines() {
        line_length = line.chars().count();
        let row_data = line
            .chars()
            .map(|c| c.to_digit(10).map(|i| i as u8))
            .collect::<Option<Vec<_>>>()
            .ok_or("Unable to parse input.")?;
        data.extend_from_slice(&row_data);
    }

    Ok(HeatMap::new(data, line_count, line_length))
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format!("Usage: {} <input data path>", args[0]));
    }

    let path = &args[1];
    let file_contents =
        fs::read_to_string(path).map_err(|e| format!("Error reading input data: {}.", e))?;

    let heat_map = parse_input(&file_contents)?;
    let minima = heat_map.find_minima();
    let height_sum: usize = minima
        .iter()
        .map(|&((_, _), v)| v as usize + 1)
        .sum();

    println!("Part one: {}", height_sum);

    let basin_sizes = heat_map.find_basin_sizes();
    let basin_product: usize = basin_sizes.iter().rev().take(3).product();
    println!("Part two: {}", basin_product);

    Ok(())
}
