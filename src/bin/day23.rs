use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::hash::Hash;
use std::iter::successors;

use aoc2021::error::Error;

fn parse_input(input: &str) -> Result<(Configuration, Configuration), Box<dyn error::Error>> {
    let extra_lines = "  #D#C#B#A#\n  #D#B#A#C#";
    let width = input
        .lines()
        .map(|l| l.len())
        .max()
        .ok_or_else(|| Error("Input is empty.".to_owned()))?;
    let height = input.lines().count();
    let mut cells = Vec::new();
    let mut extra_cells = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let mut line_cells = Vec::new();
        for c in line.chars() {
            line_cells.push(c);
        }
        while line_cells.len() < width {
            line_cells.push(' ');
        }
        cells.append(&mut line_cells.clone());
        extra_cells.append(&mut line_cells);

        if i == 2 {
            for extra_line in extra_lines.lines() {
                let mut line_cells = Vec::new();
                for c in extra_line.chars() {
                    line_cells.push(c);
                }
                while line_cells.len() < width {
                    line_cells.push(' ');
                }
                extra_cells.append(&mut line_cells);
            }
        }
    }

    Ok((
        Configuration::new(cells, width, height),
        Configuration::new(extra_cells, width, height + 2),
    ))
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Configuration {
    cells: Vec<char>,
    width: usize,
    height: usize,
}

impl Configuration {
    fn new(cells: Vec<char>, width: usize, height: usize) -> Self {
        Self {
            cells,
            width,
            height,
        }
    }

    fn get_adjacent_configurations(&self) -> Vec<(Self, usize)> {
        let mut adjacent_configs = Vec::new();

        for i in 0..self.height {
            for j in 0..self.width {
                let offset = i * self.width + j;
                if offset >= self.cells.len() {
                    break;
                }

                let mut configs = match self.cells[offset] {
                    'A' | 'B' | 'C' | 'D' => self.get_configs_for_cell(i, j),
                    _ => {
                        continue;
                    }
                };
                adjacent_configs.append(&mut configs);
            }
        }

        adjacent_configs
    }

    fn get_configs_for_cell(&self, row: usize, col: usize) -> Vec<(Self, usize)> {
        let offset = self.width * row + col;
        let amphipod = self.cells[offset];
        let base_energy = self.amphipod_energy(amphipod).unwrap();

        let row = row as isize;
        let col = col as isize;
        let candidate_positions = if row > 1 {
            self.get_room_exit_configs(row, col, base_energy)
        } else {
            self.get_room_entry_config(row, col, base_energy, amphipod)
        };

        candidate_positions
            .iter()
            .map(|&(r, c, e)| {
                let target_offset = r as usize * self.width + c as usize;
                let mut new_cells = self.cells.clone();
                new_cells[target_offset] = new_cells[offset];
                new_cells[offset] = '.';
                (
                    Configuration::new(new_cells, self.width, self.height),
                    e as usize,
                )
            })
            .collect::<Vec<_>>()
    }

    fn get_room_exit_configs(
        &self,
        row: isize,
        col: isize,
        base_energy: usize,
    ) -> Vec<(isize, isize, usize)> {
        let mut positions = Vec::new();

        for new_row in (1..row).rev() {
            if self.cell(new_row, col) != Some('.') {
                return Vec::new();
            }
        }
        for new_col in (col + 1)..(self.width as isize - 1) {
            if self.cell(1, new_col) != Some('.') {
                break;
            } else if self.cell(2, new_col) == Some('#') {
                let energy = calculate_energy((row, col), (1, new_col), base_energy);
                positions.push((1, new_col, energy));
            }
        }
        for new_col in (1..=col - 1).rev() {
            if self.cell(1, new_col) != Some('.') {
                break;
            } else if self.cell(2, new_col) == Some('#') {
                let energy = calculate_energy((row, col), (1, new_col), base_energy);
                positions.push((1, new_col, energy));
            }
        }

        positions
    }

    fn get_room_entry_config(
        &self,
        row: isize,
        col: isize,
        base_energy: usize,
        amphipod: char,
    ) -> Vec<(isize, isize, usize)> {
        let target_col = self.target_col(amphipod).unwrap() as isize;
        if !self.can_enter_col(target_col as usize) {
            return Vec::new();
        }

        let step = if col > target_col { -1 } else { 1 };
        let col_iter = successors(Some(col + step), |&c| {
            if c != target_col {
                Some(c + step)
            } else {
                None
            }
        });

        for new_col in col_iter {
            if self.cell(row, new_col) != Some('.') {
                return Vec::new();
            }
        }
        for new_row in row + 1..self.height as isize - 1 {
            if self.cell(new_row + 1, target_col) != Some('.') {
                let energy = calculate_energy((1, col), (new_row, target_col), base_energy);
                return vec![(new_row, target_col, energy)];
            }
        }

        Vec::new()
    }

    fn target_col(&self, amphipod: char) -> Option<usize> {
        match amphipod {
            'A' => Some(3),
            'B' => Some(5),
            'C' => Some(7),
            'D' => Some(9),
            _ => None,
        }
    }

    fn amphipod_energy(&self, amphipod: char) -> Option<usize> {
        match amphipod {
            'A' => Some(1),
            'B' => Some(10),
            'C' => Some(100),
            'D' => Some(1000),
            _ => None,
        }
    }

    fn can_enter_col(&self, col: usize) -> bool {
        for row in 2..self.height - 1 {
            let col_amphipod = match col {
                3 => 'A',
                5 => 'B',
                7 => 'C',
                9 => 'D',
                _ => return false,
            };
            if self
                .cell(row as isize, col as isize)
                .map(|c| c != '.' && c != col_amphipod)
                .unwrap_or_default()
            {
                return false;
            }
        }
        true
    }

    fn cell_offset(&self, row: isize, col: isize) -> Option<(usize, char)> {
        if row < 0 || row >= self.height as isize || col < 0 || col >= self.width as isize {
            return None;
        }
        let offset = row as usize * self.width + col as usize;
        if offset < self.cells.len() {
            Some((offset, self.cells[offset]))
        } else {
            None
        }
    }

    fn cell(&self, row: isize, col: isize) -> Option<char> {
        self.cell_offset(row, col).map(|(_, c)| c)
    }

    fn is_organised(&self) -> bool {
        for row in 2..self.height - 1 {
            for col in [3, 5, 7, 9] {
                match (col, self.cell(row as isize, col)) {
                    (3, Some('A')) | (5, Some('B')) | (7, Some('C')) | (9, Some('D')) => {}
                    _ => {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl Hash for Configuration {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cells.hash(state);
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let offset = i * self.width + j;
                write!(f, "{}", self.cells[offset])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn calculate_energy(start: (isize, isize), end: (isize, isize), step_cost: usize) -> usize {
    ((end.0 - start.0).abs() + (end.1 - start.1).abs()) as usize * step_cost
}

fn find_minimum_energy(start: Configuration) -> Result<usize, Box<dyn error::Error>> {
    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    struct QueueItem {
        energy: usize,
        config: Configuration,
    }

    impl Ord for QueueItem {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.energy.cmp(&self.energy)
        }
    }

    impl PartialOrd for QueueItem {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut energies = HashMap::new();
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push(QueueItem {
        energy: 0,
        config: start,
    });

    while let Some(QueueItem { energy, config }) = heap.pop() {
        if config.is_organised() {
            return Ok(energy);
        }

        if visited.contains(&config) {
            continue;
        }

        visited.insert(config.clone());

        for (new_config, extra_energy) in config.get_adjacent_configurations() {
            if visited.contains(&new_config) {
                continue;
            }

            let next = QueueItem {
                energy: energy + extra_energy,
                config: new_config,
            };
            let current_energy = *energies.get(&next.config).unwrap_or(&usize::MAX);
            if next.energy < current_energy {
                energies.insert(next.config.clone(), next.energy);
                heap.push(next);
            }
        }
    }

    Err(Box::new(Error("Unable to find minimum energy.".to_owned())))
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

    let (config, extra_config) = parse_input(&file_contents)?;

    println!("Part one: {}", find_minimum_energy(config)?);
    println!("Part two: {}", find_minimum_energy(extra_config)?);

    Ok(())
}
