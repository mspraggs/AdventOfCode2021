use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;

trait Path: Clone {
    fn can_visit_node(&self, node: &str) -> bool;
    fn visit_node(&mut self, node: &str);
    fn current_node(&self) -> Option<String>;
}

#[derive(Debug, Default, Clone)]
struct FastPath {
    current_node: Option<String>,
    node_counter: HashMap<String, usize>,
}

impl FastPath {
    fn new(init_node: &str) -> Self {
        let mut node_counter = HashMap::new();
        node_counter.insert(init_node.to_string(), 1);
        Self {
            current_node: Some(init_node.to_string()),
            node_counter,
        }
    }
}

impl Path for FastPath {
    fn can_visit_node(&self, node: &str) -> bool {
        if node == "start" {
            return false;
        }
        let count = self.node_counter.get(node).copied().unwrap_or_default();
        count == 0 || node.to_ascii_uppercase() == node
    }

    fn visit_node(&mut self, node: &str) {
        let counter = self.node_counter.entry(node.to_string()).or_default();
        *counter += 1;

        self.current_node = Some(node.to_string());
    }

    fn current_node(&self) -> Option<String> {
        self.current_node.clone()
    }
}

#[derive(Debug, Default, Clone)]
struct ScenicPath {
    current_node: Option<String>,
    node_counter: HashMap<String, usize>,
    small_node_quota_reached: bool,
}

impl ScenicPath {
    fn new(init_node: &str) -> Self {
        let mut node_counter = HashMap::new();
        node_counter.insert(init_node.to_string(), 1);
        Self {
            current_node: Some(init_node.to_string()),
            node_counter,
            small_node_quota_reached: false,
        }
    }
}

impl Path for ScenicPath {
    fn can_visit_node(&self, node: &str) -> bool {
        if node == "start" {
            return false;
        }
        let count = self.node_counter.get(node).copied().unwrap_or_default();
        count == 0 || (count < 2 && !self.small_node_quota_reached) || node.to_ascii_uppercase() == node
    }

    fn visit_node(&mut self, node: &str) {
        let counter = self.node_counter.entry(node.to_string()).or_default();
        *counter += 1;

        if *counter == 2 && node.to_ascii_lowercase() == node {
            self.small_node_quota_reached = true;
        }

        self.current_node = Some(node.to_string());
    }

    fn current_node(&self) -> Option<String> {
        self.current_node.clone()
    }
}

fn parse_input(data: &str) -> Result<HashMap<String, Vec<String>>, String> {
    let mut ret: HashMap<_, Vec<_>> = HashMap::new();

    for line in data.lines() {
        let mut split_iter = line.split("-");
        let first = split_iter.next().ok_or("Unable to parse input.")?;
        let second = split_iter.next().ok_or("Unable to parse input.")?;
        if second != "start" {
            let entry = ret.entry(first.to_string()).or_default();
            entry.push(second.to_string());
        }
        if first != "start" {
            let entry = ret.entry(second.to_string()).or_default();
            entry.push(first.to_string());
        }
    }

    Ok(ret)
}

fn find_all_paths(
    map: &HashMap<String, Vec<String>>,
    init_path: impl Path,
) -> Result<Vec<impl Path>, String> {
    let mut queue = VecDeque::new();
    queue.push_back(init_path);
    let mut paths = Vec::new();

    while let Some(path) = queue.pop_front() {
        let node = path.current_node().ok_or("Path is empty")?;
        if node == "end" {
            paths.push(path);
            continue;
        }

        if let Some(neighbours) = map.get(&node) {
            for neighbour in neighbours {
                if path.can_visit_node(neighbour) {
                    let mut new_path = path.clone();
                    new_path.visit_node(&neighbour);
                    queue.push_back(new_path);
                }
            }
        }
    }

    Ok(paths)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format!("Usage: {} <input data path>", args[0]));
    }

    let path = &args[1];
    let file_contents =
        fs::read_to_string(path).map_err(|e| format!("Error reading input data: {}.", e))?;
    let map = parse_input(&file_contents)?;

    let paths = find_all_paths(&map, FastPath::new("start"))?;
    println!("Part one: {}", paths.len());

    let paths = find_all_paths(&map, ScenicPath::new("start"))?;
    println!("Part two: {}", paths.len());

    Ok(())
}
