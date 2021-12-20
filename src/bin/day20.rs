use std::collections::HashMap;
use std::env;
use std::error;
use std::fmt;
use std::fs;

use aoc2021::error::Error;

fn parse_input(data: &str) -> Result<(Image, Algorithm), Box<dyn error::Error>> {
    let mut line_iter = data.lines();
    let algorithm = line_iter
        .next()
        .ok_or_else(|| Error("Unable to parse input.".to_owned()))?
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();

    line_iter.next();

    let mut data = Vec::new();
    let mut height = 0;
    let mut width = 0;
    for line in line_iter {
        height += 1;
        width = line.len();
        let line_data = line.chars().map(|c| c == '#').collect::<Vec<_>>();
        data.extend_from_slice(&line_data);
    }

    Ok((Image::new(data, width, height), algorithm))
}

#[derive(Debug, Default, Clone)]
struct Image {
    data: Vec<bool>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(data: Vec<bool>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    fn process(&self, processor: &mut ImageProcessor, num_iterations: usize) -> Image {
        let output_width = self.width + 2 * num_iterations;
        let output_height = self.height + 2 * num_iterations;
        let output_size = output_width * output_height;

        let mut output_data = vec![false; output_size];

        let row_iter = -(num_iterations as i32)..(self.height + num_iterations) as i32;
        for (i, row) in row_iter.enumerate() {
            let col_iter = -(num_iterations as i32)..(self.width + num_iterations) as i32;
            for (j, col) in col_iter.enumerate() {
                output_data[i * output_width + j] =
                    processor.process(&|r, c| self.pixel_value(r, c), row, col, num_iterations)
            }
        }

        Image::new(output_data, output_width, output_height)
    }

    fn num_lit_pixels(&self) -> usize {
        self.data.iter().filter(|&&p| p).count()
    }

    fn pixel_value(&self, row: i32, col: i32) -> bool {
        if row < 0 || col < 0 || (row as usize) >= self.height || (col as usize) >= self.width {
            return false;
        }

        let index = (row as usize) * self.width + (col as usize);
        self.data[index]
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let c = if self.data[self.width * i + j] {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

type Algorithm = Vec<bool>;

struct ImageProcessor {
    algorithm: Algorithm,
    cache: HashMap<(i32, i32, usize), bool>,
}

impl ImageProcessor {
    fn new(algorithm: Algorithm) -> Self {
        Self {
            algorithm,
            cache: HashMap::new(),
        }
    }

    fn process(
        &mut self,
        get_pixel_value: &impl Fn(i32, i32) -> bool,
        row: i32,
        col: i32,
        num_iterations: usize,
    ) -> bool {
        if let Some(result) = self.cache.get(&(row, col, num_iterations)) {
            return *result;
        }

        let result = if num_iterations == 0 {
            get_pixel_value(row, col)
        } else {
            let mut pixels = [false; 9];
            for (i, r) in (-1..2).enumerate() {
                for (j, c) in (-1..2).enumerate() {
                    pixels[i * 3 + j] =
                        self.process(get_pixel_value, row + r, col + c, num_iterations - 1);
                }
            }

            let mut index = 0;
            for (i, &v) in pixels.iter().rev().enumerate() {
                if v {
                    index |= 1 << i;
                }
            }
            self.algorithm[index]
        };

        self.cache.insert((row, col, num_iterations), result);
        result
    }
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

    let (input, algorithm) = parse_input(&file_contents)?;

    let mut processor = ImageProcessor::new(algorithm);

    let output = input.process(&mut processor, 2);

    println!("Part one: {}", output.num_lit_pixels());

    let output = input.process(&mut processor, 50);

    println!("Part two: {}", output.num_lit_pixels());

    Ok(())
}
