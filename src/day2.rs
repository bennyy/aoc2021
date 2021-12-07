use crate::file_util;

#[derive(Debug)]
struct Action<'a> {
    direction: &'a str,
    steps: i32,
}

fn parse_action(input_str: &str) -> Action {
    let split: Vec<&str> = input_str.split(' ').collect();
    Action {
        direction: split.get(0).unwrap_or(&""),
        steps: split.get(1).unwrap_or(&"").parse::<i32>().unwrap_or(0),
    }
}

fn puzzle1(file_path: String) -> i32 {
    let input_str = file_util::file_to_string(file_path);

    let mut distance = 0;
    let mut depth = 0;

    for input in input_str.lines() {
        let action = parse_action(input);
        if action.direction == "forward" {
            distance += action.steps;
        } else if action.direction == "up" {
            depth -= action.steps;
        } else if action.direction == "down" {
            depth += action.steps;
        }
    }

    distance * depth
}

fn puzzle2(file_path: String) -> i32 {
    let input_str = file_util::file_to_string(file_path);

    let mut distance = 0;
    let mut depth = 0;
    let mut aim = 0;

    for input in input_str.lines() {
        let action = parse_action(input);
        if action.direction == "forward" {
            distance += action.steps;
            depth += aim * action.steps;
        } else if action.direction == "up" {
            aim -= action.steps;
        } else if action.direction == "down" {
            aim += action.steps;
        }
    }

    distance * depth
}

pub fn run() {
    let result = puzzle1("inputs/day2.txt".to_string());
    println!("D2P1: {}", result);
    let result = puzzle2("inputs/day2.txt".to_string());
    println!("D2P2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(150, puzzle1("inputs/day2_test.txt".to_string()));
        assert_eq!(1868935, puzzle1("inputs/day2.txt".to_string()));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(900, puzzle2("inputs/day2_test.txt".to_string()));
        assert_eq!(1965970888, puzzle2("inputs/day2.txt".to_string()));
    }
}
