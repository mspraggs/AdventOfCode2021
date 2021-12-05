use std::cmp::Ordering;
use std::env;
use std::fmt;
use std::fs;

macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct PointIter {
    current: Point,
    end: Point,
    step: (i32, i32),
}

impl PointIter {
    fn new(begin: Point, end: Point) -> Self {
        assert!(
            begin.x == end.x
                || begin.y == end.y
                || (end.x as i32 - begin.x as i32).abs() == (end.y as i32 - begin.y as i32).abs()
        );

        let step = match (begin.x.cmp(&end.x), begin.y.cmp(&end.y)) {
            (Ordering::Less, Ordering::Less) => (1, 1),
            (Ordering::Less, Ordering::Equal) => (1, 0),
            (Ordering::Less, Ordering::Greater) => (1, -1),
            (Ordering::Equal, Ordering::Less) => (0, 1),
            (Ordering::Equal, Ordering::Equal) => (0, 0),
            (Ordering::Equal, Ordering::Greater) => (0, -1),
            (Ordering::Greater, Ordering::Less) => (-1, 1),
            (Ordering::Greater, Ordering::Equal) => (-1, 0),
            (Ordering::Greater, Ordering::Greater) => (-1, -1),
        };

        let end = Point::new(
            (end.x as i32 + step.0) as usize,
            (end.y as i32 + step.1) as usize,
        );

        Self {
            current: begin,
            end,
            step,
        }
    }
}

impl Iterator for PointIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let ret = self.current;
            self.current.x = (self.current.x as i32 + self.step.0) as usize;
            self.current.y = (self.current.y as i32 + self.step.1) as usize;
            Some(ret)
        }
    }
}

type Line = (Point, Point);

fn line_is_horiz_or_vert(line: &Line) -> bool {
    line.0.x == line.1.x || line.0.y == line.1.y
}

fn parse_point(point_spec: &str) -> Result<Point, String> {
    let ints = scan!(point_spec, ',', usize, usize);
    match ints {
        (Some(x), Some(y)) => Ok(Point::new(x, y)),
        _ => Err(("Unable to parse point.").to_string()),
    }
}

fn parse_line(line_spec: &str) -> Result<Line, String> {
    let mut point_iter = line_spec.split(" -> ");
    let mut line = Line::default();

    line.0 = point_iter
        .next()
        .and_then(|s| parse_point(s).ok())
        .ok_or("Unable to parse line.")?;
    line.1 = point_iter
        .next()
        .and_then(|s| parse_point(s).ok())
        .ok_or("Unable to parse line.")?;

    Ok(line)
}

fn parse_input(input: &str) -> Result<(Vec<Line>, usize), String> {
    let lines = input
        .lines()
        .map(|line| parse_line(line))
        .collect::<Result<Vec<Line>, String>>()?;

    let max = lines
        .iter()
        .map(|l| {
            let values = [l.0.x, l.0.y, l.1.x, l.1.y];
            values.iter().max().copied().unwrap()
        })
        .max()
        .ok_or("Input is empty.")?;

    Ok((lines, max as usize + 1))
}

#[derive(Debug, Default)]
struct Grid {
    data: Vec<u32>,
    size: usize,
}

impl Grid {
    fn new(size: usize) -> Self {
        Self {
            data: vec![0; size * size],
            size,
        }
    }

    fn mark_point(&mut self, point: Point) {
        let offset = point.y * self.size + point.x;
        self.data[offset] += 1
    }

    fn mark_line(&mut self, line: &Line) {
        for p in PointIter::new(line.0, line.1) {
            self.mark_point(p);
        }
    }

    fn num_intersections(&self) -> usize {
        self.data.iter().filter(|&&c| c > 1).count()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.size {
            for j in 0..self.size {
                let offset = i * self.size + j;
                write!(f, "{}", self.data[offset])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Usage: ./day3 <input data path>".to_string());
    }

    let path = &args[1];
    let file_contents =
        fs::read_to_string(path).map_err(|e| format!("Error reading input data: {}.", e))?;

    let (lines, grid_size) = parse_input(&file_contents)?;

    let mut grid = Grid::new(grid_size);
    let horizontal_and_vertical_lines = lines
        .iter()
        .copied()
        .filter(line_is_horiz_or_vert)
        .collect::<Vec<Line>>();

    for line in &horizontal_and_vertical_lines {
        grid.mark_line(line);
    }

    println!("Part one: {}", grid.num_intersections());

    let mut grid = Grid::new(grid_size);
    for line in &lines {
        grid.mark_line(line);
    }

    println!("Part two: {}", grid.num_intersections());

    Ok(())
}
