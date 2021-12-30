use std::env;
use std::error;
use std::fs;

use aoc2021::error::Error;

fn parse_monad(input: &str) -> Result<Monad, Box<dyn error::Error>> {
    let mut parameter_groups = Vec::new();
    let mut parameter_group = ParameterGroup::default();

    for (i, line) in input.lines().enumerate() {
        if i % 18 == 4 {
            parameter_group.divisor = parse_line(line)?;
        } else if i % 18 == 5 {
            parameter_group.check = parse_line(line)?;
        } else if i % 18 == 15 {
            parameter_group.modifier = parse_line(line)?;
            parameter_groups.push(parameter_group);
            parameter_group = ParameterGroup::default();
        }
    }

    Ok(Monad::new(parameter_groups))
}

fn parse_line(line: &str) -> Result<isize, Box<dyn error::Error>> {
    line.split(' ')
        .nth(2)
        .and_then(|s| s.parse::<isize>().ok())
        .ok_or_else(|| -> Box<dyn error::Error> { Box::new(Error("".to_owned())) })
}

#[derive(Debug, Default, Clone, Copy)]
struct ParameterGroup {
    divisor: isize,
    check: isize,
    modifier: isize,
}

#[derive(Debug, Default, Clone)]
struct Monad {
    parameters: Vec<ParameterGroup>,
    stack: Vec<(usize, isize)>,
}

impl Monad {
    fn new(parameters: Vec<ParameterGroup>) -> Self {
        let init_stack_size = parameters.len();
        Self {
            parameters,
            stack: Vec::with_capacity(init_stack_size),
        }
    }

    fn find_nearest(&mut self, init: &[isize]) -> Vec<isize> {
        let mut input = init.to_vec();

        'outer: loop {
            self.stack.clear();
            for (i, &value) in input.iter().enumerate() {
                if let Some((index, value)) = self.test_input(i, value) {
                    input[index] = value;
                    continue 'outer;
                }
            }
            break;
        }

        input
    }

    fn test_input(&mut self, index: usize, input: isize) -> Option<(usize, isize)> {
        let parameters = &self.parameters[index];
        let (prev_index, last) = self.stack.last().copied().unwrap_or_default();
        let check = last + parameters.check;

        let mut result = None;

        if parameters.divisor == 26 {
            self.stack.pop();
        }
        if check != input {
            result = self.find_best_input(input, check, index, prev_index);
            self.stack.push((index, input + parameters.modifier));
        }

        result
    }

    fn find_best_input(
        &self,
        input: isize,
        check: isize,
        index: usize,
        prev_index: usize,
    ) -> Option<(usize, isize)> {
        let parameters = &self.parameters[index];
        if parameters.divisor == 26 {
            if check > 9 || check < 1 {
                let optimal_input = input - parameters.check - self.parameters[prev_index].modifier;
                Some((prev_index, optimal_input))
            } else {
                Some((index, check))
            }
        } else {
            None
        }
    }
}

fn input_to_value(input: &[isize]) -> isize {
    let mut value = 0;

    for i in input {
        value *= 10;
        value += i;
    }

    value
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

    let mut monad = parse_monad(&file_contents)?;
    let max = monad.find_nearest(&vec![9; 14]);
    println!("Part one: {}", input_to_value(&max));
    let min = monad.find_nearest(&vec![1; 14]);
    println!("Part two: {}", input_to_value(&min));

    Ok(())
}
