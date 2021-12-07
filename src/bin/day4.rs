use core::fmt;
use std::env;
use std::fs;

fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<Board>), String> {
    let mut line_iter = input.lines();
    let numbers = if let Some(l) = line_iter.next() {
        l.split(",")
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
            .map_err(|_| "Unable to parse integer.")?
    } else {
        return Err("".to_string());
    };

    line_iter.next();

    let mut boards = Vec::new();
    let mut values = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;

    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            boards.push(Board::new(values, (height, width)));
            height = 0;
            width = 0;
            values = Vec::new();
        } else {
            height += 1;
            let row_values = line
                .split_whitespace()
                .map(|s| s.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()
                .map_err(|_| "Unable to parse integer.")?;
            values.extend_from_slice(&row_values);
            width = row_values.len();
        }
    }

    if !values.is_empty() {
        boards.push(Board::new(values, (height, width)));
    }

    Ok((numbers, boards))
}

#[derive(Debug, Default, Clone)]
struct Board {
    values: Vec<i32>,
    markers: Vec<bool>,
    size: (usize, usize),
}

impl Board {
    fn new(values: Vec<i32>, size: (usize, usize)) -> Self {
        let num_values = values.len();
        assert_eq!(num_values, (size.0 * size.1) as usize);
        Self {
            values,
            markers: vec![false; num_values],
            size,
        }
    }

    fn wins(&self) -> bool {
        let (height, width) = self.size;
        for i in 0..height {
            if self.markers.iter().skip(i * width).take(width).all(|v| *v) {
                return true;
            }
        }
        for i in 0..width {
            if self.markers.iter().skip(i).step_by(width).all(|v| *v) {
                return true;
            }
        }
        false
    }

    fn mark(&mut self, value: i32) {
        for (i, &v) in self.values.iter().enumerate() {
            if v == value {
                self.markers[i] = true;
            }
        }
    }

    fn sum_unmarked_values(&self) -> i32 {
        let mut sum = 0;
        for (i, &value) in self.values.iter().enumerate() {
            if !self.markers[i] {
                sum += value;
            }
        }
        sum
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (height, width) = self.size;
        for i in 0..height {
            for j in 0..width {
                let offset = i * width + j;
                if self.markers[offset] {
                    write!(f, "  x")?;
                } else {
                    write!(f, " {:>2}", self.values[offset])?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format!("Usage: {} <input data path>", args[0]));
    }

    let path = &args[1];
    let file_contents =
        fs::read_to_string(path).map_err(|e| format!("Error reading input data: {}.", e))?;

    let (numbers, mut boards) = parse_input(&file_contents)?;

    let mut ordered_scores = Vec::new();
    let mut board_is_winning = vec![false; boards.len()];

    for number in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            board.mark(number);
            if board.wins() && !board_is_winning[i] {
                let score = board.sum_unmarked_values() * number;
                ordered_scores.push(score);
                board_is_winning[i] = true;
            }
        }
    }

    println!("Part one: {}", ordered_scores.first().unwrap());
    println!("Part two: {}", ordered_scores.last().unwrap());

    Ok(())
}
