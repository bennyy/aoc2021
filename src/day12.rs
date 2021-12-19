use crate::file_util;
use std::collections::HashMap;

static INPUT_FILE: &str = "inputs/day12.txt";

#[derive(Debug)]
struct Node<'a> {
    name: String,
    nodes: Vec<&'a Node<'a>>,
}

fn visit(
    korv: &HashMap<String, Vec<String>>,
    current: &str,
    path: &mut Vec<String>,
    paths: &mut Vec<Vec<String>>,
    visited: &mut Vec<String>,
) {
    // Check if we're a small cave..
    if current.chars().next().unwrap().is_lowercase() {
        visited.push(current.to_string()); // Yes, never come back here..
    }

    path.push(current.to_string());

    if current == "end" {
        paths.push(path.to_vec());
    } else {
        let neighbours = korv.get(current).unwrap();
        for ne in neighbours {
            if ne == "start" {
                continue;
            }

            if !visited.contains(ne) {
                visit(korv, ne, path, paths, visited);
            }
        }
    }

    path.pop();
    visited.retain(|x| x != current);
}

fn puzzle1(file_path: String) -> u64 {
    let input_str = file_util::file_to_string(file_path);
    let input_str: Vec<&str> = input_str.split('\n').collect();

    let mut nodes: HashMap<String, Vec<String>> = HashMap::new();

    for line in input_str {
        let split: Vec<_> = line.split('-').collect();
        let from = split.get(0).unwrap();
        let to = split.get(1).unwrap();

        nodes.entry(from.to_string()).or_insert_with(Vec::new);
        nodes.entry(to.to_string()).or_insert_with(Vec::new);

        nodes
            .get_mut(&from.to_string())
            .unwrap()
            .push(to.to_string());
        nodes
            .get_mut(&to.to_string())
            .unwrap()
            .push(from.to_string());
    }

    let mut visited = Vec::new();
    let mut paths: Vec<Vec<String>> = Vec::new();
    let mut path: Vec<String> = Vec::new();

    visit(
        &nodes,
        &"start".to_string(),
        &mut path,
        &mut paths,
        &mut visited,
    );

    paths.len() as u64
}

fn puzzle2(_file_path: String) -> u64 {
    0
}

pub fn run() {
    let result = puzzle1(INPUT_FILE.to_string());
    println!("D12P1: {}", result);
    let result = puzzle2(INPUT_FILE.to_string());
    println!("D12P2: {}", result);
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_FILE_1: &str = "inputs/day12_test_1.txt";
    static TEST_INPUT_FILE_2: &str = "inputs/day12_test_2.txt";
    static TEST_INPUT_FILE_3: &str = "inputs/day12_test_3.txt";
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(10, puzzle1(TEST_INPUT_FILE_1.to_string()));
        assert_eq!(19, puzzle1(TEST_INPUT_FILE_2.to_string()));
        assert_eq!(226, puzzle1(TEST_INPUT_FILE_3.to_string()));
        assert_eq!(4691, puzzle1(INPUT_FILE.to_string()));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(0, puzzle2(TEST_INPUT_FILE_1.to_string()));
        assert_eq!(0, puzzle2(INPUT_FILE.to_string()));
    }
}
