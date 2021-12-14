use std::collections::HashMap;
use std::env;
use std::error;
use std::fs;

use aoc2021::error::Error;

#[derive(Debug, Default, Clone)]
struct Polymeriser {
    mapping: HashMap<(char, char), char>,
    cache: HashMap<(char, char, usize), HashMap<char, usize>>,
}

impl Polymeriser {
    fn new(mapping: HashMap<(char, char), char>) -> Self {
        Self {
            mapping,
            cache: HashMap::new(),
        }
    }

    fn polymerise(
        &mut self,
        template: &str,
        num_iterations: usize,
    ) -> Result<HashMap<char, usize>, Box<dyn error::Error>> {
        if template.is_empty() {
            return Ok(HashMap::new());
        }

        let mut char_iter = template.chars();
        let mut char_counts = HashMap::new();
        
        let mut first = char_iter.next().unwrap();
        char_counts.insert(first, 1);

        for second in char_iter {
            let counts = self.polymerise_pair((first, second), num_iterations)?;
            for (k, v) in counts.iter() {
                *char_counts.entry(*k).or_default() += v;
            }
            first = second;
        }

        Ok(char_counts)
    }

    fn polymerise_pair(
        &mut self,
        pair: (char, char),
        num_iterations: usize,
    ) -> Result<HashMap<char, usize>, Box<dyn error::Error>> {
        if let Some(result) = self.cache.get(&(pair.0, pair.1, num_iterations)) {
            return Ok(result.clone());
        }
        if num_iterations == 0 {
            let mut counts = HashMap::new();
            counts.insert(pair.1, 1);
            self.cache
                .insert((pair.0, pair.1, num_iterations), counts.clone());
            return Ok(counts);
        }

        let mut counts = HashMap::new();

        let new_char = self
            .mapping
            .get(&pair)
            .copied()
            .ok_or_else(|| Error(format!("Unable to find pair ({}, {}).", pair.0, pair.1)))?;

        let first_counts = self.polymerise_pair((pair.0, new_char), num_iterations - 1)?;
        let second_counts = self.polymerise_pair((new_char, pair.1), num_iterations - 1)?;

        for (k, v) in first_counts.iter() {
            *counts.entry(*k).or_default() += *v;
        }
        for (k, v) in second_counts.iter() {
            *counts.entry(*k).or_default() += *v;
        }

        self.cache
            .insert((pair.0, pair.1, num_iterations), counts.clone());
        Ok(counts)
    }
}

fn parse_line(line: &str) -> Result<((char, char), char), Box<dyn error::Error>> {
    let mut split_iter = line.split(" -> ");
    let element_pair = split_iter
        .next()
        .ok_or_else(|| Error("Unable to parse input".to_string()))?;
    let mut pair_chars = element_pair.chars();
    let first_element = pair_chars
        .next()
        .ok_or_else(|| Error("Unable to parse element pair.".to_string()))?;
    let second_element = pair_chars
        .next()
        .ok_or_else(|| Error("Unable to parse element pair.".to_string()))?;

    let result_element = split_iter
        .next()
        .and_then(|s| s.chars().next())
        .ok_or_else(|| Error("".to_string()))?;

    Ok(((first_element, second_element), result_element))
}

fn parse_input(data: &str) -> Result<(String, HashMap<(char, char), char>), Box<dyn error::Error>> {
    let mut lines_iter = data.lines();
    let template = lines_iter
        .next()
        .ok_or_else(|| Error("Input file is empty.".to_string()))?;

    lines_iter.next();

    let mut element_mapping = HashMap::new();

    for map in lines_iter {
        let (pair, new_element) = parse_line(map)?;
        element_mapping.insert(pair, new_element);
    }

    Ok((template.to_string(), element_mapping))
}

fn compute_min_max_diff(counts: &HashMap<char, usize>) -> Result<usize, Box<dyn error::Error>> {
    let min_count = counts
        .iter()
        .min_by(|a, b| a.1.cmp(b.1))
        .map(|(_, v)| *v)
        .ok_or_else(|| Error("Unable to derive minimum.".to_string()))?;
    let max_count = counts
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(_, v)| *v)
        .ok_or_else(|| Error("Unable to derive maximum.".to_string()))?;

    Ok(max_count - min_count)
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
    
    let (template, mapping) = parse_input(&file_contents)?;
    
    let mut polymeriser = Polymeriser::new(mapping);
    let counts = polymeriser.polymerise(&template, 10)?;
    println!("Part one: {}", compute_min_max_diff(&counts)?);

    let counts = polymeriser.polymerise(&template, 40)?;
    println!("Part two: {}", compute_min_max_diff(&counts)?);

    Ok(())
}
