use std::env;
use std::error;
use std::fs;

use aoc2021::error::Error;

const NDIMS: usize = 3;

fn parse_cuboid(spec: &str) -> Result<Cuboid, Box<dyn error::Error>> {
    let mut values = [0; 6];

    for (i, axis_spec) in spec.split(',').enumerate() {
        let axis_spec = axis_spec
            .split('=')
            .skip(1)
            .next()
            .ok_or_else(|| Error("Unable to parse input.".to_owned()))?;
        for (j, s) in axis_spec.split("..").enumerate() {
            values[2 * i + j] = s.parse::<i32>()?;
        }
    }

    Ok(Cuboid::new(
        [values[0], values[2], values[4]],
        [values[1] + 1, values[3] + 1, values[5] + 1],
    ))
}

fn parse_line(line: &str) -> Result<Instruction, Box<dyn error::Error>> {
    let mut split_iter = line.split(' ');
    let turn_on = split_iter
        .next()
        .map(|s| s == "on")
        .ok_or_else(|| Error("Unable to parse input.".to_owned()))?;
    let line = split_iter
        .next()
        .ok_or_else(|| Error("Unable to parse input.".to_owned()))?;

    let cuboid = parse_cuboid(line)?;

    Ok(Instruction::new(turn_on, cuboid))
}

fn parse_input(data: &str) -> Result<Vec<Instruction>, Box<dyn error::Error>> {
    let mut ret = Vec::new();

    for line in data.lines() {
        ret.push(parse_line(line)?);
    }

    Ok(ret)
}

type Point = [i32; NDIMS];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cuboid {
    lower: Point,
    upper: Point,
}

impl Cuboid {
    fn new(lower: Point, upper: Point) -> Self {
        Self { lower, upper }
    }

    fn intersects(&self, other: &Cuboid) -> bool {
        (0..NDIMS).all(|i| self.upper[i] > other.lower[i] || self.lower[i] > other.upper[i])
    }

    fn contains(&self, other: &Cuboid) -> bool {
        (0..NDIMS).all(|i| self.upper[i] <= other.upper[i] && self.lower[i] >= other.upper[i])
    }

    fn contains_point(&self, point: Point) -> bool {
        (0..NDIMS).all(|i| point[i] >= self.lower[i] && point[i] < self.upper[i])
    }

    fn difference(&self, other: &Cuboid) -> Vec<Cuboid> {
        if !self.intersects(other) {
            return Vec::new();
        }

        let mut corners = self.corners();
        corners.append(&mut other.corners());
        corners.sort();

        let mut spans = [[0; 4]; NDIMS];

        for i in 0..NDIMS {
            spans[i][0] = self.lower[i];
            spans[i][1] = self.upper[i];
            spans[i][2] = other.lower[i];
            spans[i][3] = other.upper[i];
            spans[i].sort();
        }

        let mut cuboids = Vec::new();

        for indices in NDimsIter::new(3) {
            let mut new_lower = [0; NDIMS];
            let mut new_upper = [0; NDIMS];
            for (i, &idx) in indices.iter().enumerate() {
                new_lower[i] = spans[i][idx];
                new_upper[i] = spans[i][idx + 1];
            }

            if !other.contains_point(new_lower) && self.contains_point(new_lower) {
                cuboids.push(Cuboid::new(new_lower, new_upper));
            }
        }

        cuboids
    }

    fn volume(&self) -> usize {
        (0..NDIMS)
            .map(|i| self.upper[i] - self.lower[i])
            .product::<i32>() as usize
    }

    fn corners(&self) -> Vec<Point> {
        let bounds = &[self.lower, self.upper];

        let mut points = Vec::new();

        for indices in NDimsIter::new(2) {
            let mut corner = [0; NDIMS];
            for i in 0..NDIMS {
                corner[i] = bounds[indices[i]][i];
            }

            points.push(corner);
        }

        points.sort();
        points
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Instruction {
    turn_on: bool,
    cuboid: Cuboid,
}

impl Instruction {
    fn new(turn_on: bool, cuboid: Cuboid) -> Self {
        Self { turn_on, cuboid }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct NDimsIter {
    current: [usize; NDIMS],
    max_index: usize,
}

impl NDimsIter {
    fn new(max_index: usize) -> Self {
        Self {
            current: [0; NDIMS],
            max_index,
        }
    }
}

impl Iterator for NDimsIter {
    type Item = [usize; NDIMS];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == [self.max_index; NDIMS] {
            return None;
        }

        let current = self.current;

        let mut dim = 0;
        self.current[dim] += 1;
        while self.current[dim] == self.max_index {
            self.current[dim] = 0;
            dim += 1;

            if dim == NDIMS {
                self.current = [self.max_index; NDIMS];
                break;
            }

            self.current[dim] += 1;
        }

        Some(current)
    }
}

fn in_bounds(point: &Point) -> bool {
    point.iter().any(|p| p.abs() <= 51)
}

fn add_cuboid(active: Vec<Cuboid>, to_add: &Cuboid) -> Vec<Cuboid> {
    let mut new_active = remove_cuboid(active, to_add);
    new_active.push(*to_add);
    new_active
}

fn remove_cuboid(active: Vec<Cuboid>, to_remove: &Cuboid) -> Vec<Cuboid> {
    let mut new_active = Vec::new();

    for cuboid in active {
        if !cuboid.intersects(to_remove) {
            new_active.push(cuboid);
            continue;
        }

        if to_remove.contains(&cuboid) {
            continue;
        }

        let mut new_cuboids = cuboid.difference(to_remove);
        new_active.append(&mut new_cuboids);
    }

    new_active
}

fn count_cubes(instructions: &[Instruction]) -> usize {
    let mut cuboid_store: Vec<Cuboid> = Vec::new();

    for instruction in instructions.iter() {
        if instruction.turn_on {
            cuboid_store = add_cuboid(cuboid_store, &instruction.cuboid);
        } else {
            cuboid_store = remove_cuboid(cuboid_store, &instruction.cuboid);
        }
    }

    cuboid_store.iter().map(|c| c.volume()).sum()
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

    let instructions = parse_input(&file_contents)?;

    let restricted_instructions = instructions
        .iter()
        .copied()
        .filter(|i| in_bounds(&i.cuboid.lower) && in_bounds(&i.cuboid.upper))
        .collect::<Vec<_>>();

    println!("Part one: {}", count_cubes(&restricted_instructions));
    println!("Part two: {}", count_cubes(&instructions));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference() {
        let first = Cuboid::new([0, 0, 0], [10, 10, 10]);
        let second = Cuboid::new([5, 5, 5], [15, 15, 15]);

        let expected = vec![
            Cuboid::new([5, 5, 10], [10, 10, 15]),
            Cuboid::new([5, 10, 5], [10, 15, 10]),
            Cuboid::new([5, 10, 10], [10, 15, 15]),
            Cuboid::new([10, 5, 5], [15, 10, 10]),
            Cuboid::new([10, 5, 10], [15, 10, 15]),
            Cuboid::new([10, 10, 5], [15, 15, 10]),
            Cuboid::new([10, 10, 10], [15, 15, 15]),
        ];

        let mut difference = second.difference(&first);
        difference.sort();

        assert_eq!(expected, difference);

        let first = Cuboid::new([0, 0, 0], [15, 10, 10]);
        let second = Cuboid::new([5, 5, 5], [10, 15, 15]);

        let expected = vec![
            Cuboid::new([5, 5, 10], [10, 10, 15]),
            Cuboid::new([5, 10, 5], [10, 15, 10]),
            Cuboid::new([5, 10, 10], [10, 15, 15]),
        ];

        let mut difference = second.difference(&first);
        difference.sort();

        assert_eq!(expected, difference);

        let first = Cuboid::new([0, 0, 5], [10, 10, 10]);
        let second = Cuboid::new([5, 5, 0], [15, 15, 15]);

        let expected = vec![
            Cuboid::new([5, 5, 0], [10, 10, 5]),
            Cuboid::new([5, 5, 10], [10, 10, 15]),
            Cuboid::new([5, 10, 0], [10, 15, 5]),
            Cuboid::new([5, 10, 5], [10, 15, 10]),
            Cuboid::new([5, 10, 10], [10, 15, 15]),
            Cuboid::new([10, 5, 0], [15, 10, 5]),
            Cuboid::new([10, 5, 5], [15, 10, 10]),
            Cuboid::new([10, 5, 10], [15, 10, 15]),
            Cuboid::new([10, 10, 0], [15, 15, 5]),
            Cuboid::new([10, 10, 5], [15, 15, 10]),
            Cuboid::new([10, 10, 10], [15, 15, 15]),
        ];

        let mut difference = second.difference(&first);
        difference.sort();

        assert_eq!(expected, difference);
    }

    #[test]
    fn test_volume() {
        let cube = Cuboid::new([-1, 2, -3], [5, 6, 4]);
        assert_eq!(168, cube.volume());
    }
}
