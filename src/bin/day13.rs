use std::collections::HashSet;
use std::env;
use std::error;
use std::fs;

use aoc2021::error::Error;

type Point = (usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Axis {
    X,
    Y,
}

impl Default for Axis {
    fn default() -> Self {
        Self::X
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Fold {
    pos: usize,
    axis: Axis,
}

impl Fold {
    fn new(pos: usize, axis: Axis) -> Self {
        Self { pos, axis }
    }

    fn fold(&self, point: &Point) -> Point {
        if self.axis == Axis::X {
            if point.0 <= self.pos {
                *point
            } else {
                (point.0 - 2 * (point.0 - self.pos), point.1)
            }
        } else {
            if point.1 <= self.pos {
                *point
            } else {
                (point.0, point.1 - 2 * (point.1 - self.pos))
            }
        }
    }
}

fn parse_input(data: &str) -> Result<(Vec<Point>, Vec<Fold>), Box<dyn error::Error>> {
    let mut line_iter = data.lines();

    let mut points = Vec::new();
    while let Some(line) = line_iter.next() {
        if line.len() == 0 {
            break;
        }

        let mut split = line.split(',');
        let x = split
            .next()
            .and_then(|s| s.parse::<usize>().ok())
            .ok_or(Error("Unable to parse input.".to_string()))?;
        let y = split
            .next()
            .and_then(|s| s.parse::<usize>().ok())
            .ok_or(Error("Unable to parse input.".to_string()))?;

        points.push((x, y));
    }

    let mut folds = Vec::new();
    while let Some(line) = line_iter.next() {
        let line = line
            .strip_prefix("fold along ")
            .ok_or(Error("Unable to parse input.".to_string()))?;
        let mut split = line.split('=');
        let axis = split
            .next()
            .map(|s| if s == "x" { Axis::X } else { Axis::Y })
            .ok_or(Error("Unable to parse input.".to_string()))?;
        let position = split
            .next()
            .and_then(|s| s.parse::<usize>().ok())
            .ok_or(Error("Unable to parse input.".to_string()))?;

        folds.push(Fold::new(position, axis));
    }

    Ok((points, folds))
}

fn fold_points(points: &[Point], folds: &[Fold]) -> Vec<Point> {
    let mut folded_points = points.to_vec();

    for fold in folds {
        folded_points = folded_points
            .iter()
            .map(|p| fold.fold(p))
            .collect::<Vec<_>>();
    }

    folded_points
}

fn count_unique_points(points: &[Point]) -> usize {
    let mut set = HashSet::new();
    for point in points {
        set.insert(*point);
    }
    set.len()
}

fn print_points(points: &[Point]) -> Result<(), Box<dyn error::Error>> {
    let mut unique_points = HashSet::new();
    for point in points {
        unique_points.insert(point);
    }

    let max_x = points
        .iter()
        .max_by(|&p1, &p2| p1.0.cmp(&p2.0))
        .map(|p| p.0 + 1)
        .ok_or(Error("Unable to find_maximum".to_string()))?;
    let max_y = points
        .iter()
        .max_by(|&p1, &p2| p1.1.cmp(&p2.1))
        .map(|p| p.1 + 1)
        .ok_or(Error("Unable to find_maximum".to_string()))?;

    for y in 0..max_y {
        for x in 0..max_x {
            if unique_points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Box::new(Error(format!("Usage: {} <input data path>", args[0]))));
    }

    let path = &args[1];
    let file_contents =
        fs::read_to_string(path).map_err(|e| format!("Error reading input data: {}.", e))?;

    let (points, folds) = parse_input(&file_contents)?;

    let folded_points = fold_points(&points, &folds[..1]);
    let num_unique_points = count_unique_points(&folded_points);

    println!("Part one: {}", num_unique_points);

    let folded_points = fold_points(&points, &folds);
    println!("Part two:");
    print_points(&folded_points)?;

    Ok(())
}
