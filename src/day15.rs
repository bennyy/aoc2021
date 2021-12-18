use crate::file_util;

static INPUT_FILE: &str = "inputs/day15.txt";

fn puzzle1(file_path: String) -> u64 {
    let input_str = file_util::file_to_string(file_path);
    let input_str: Vec<&str> = input_str.split('\n').collect();

    0
}

fn puzzle2(file_path: String) -> u64 {
    0
}

pub fn run() {
    let result = puzzle1(INPUT_FILE.to_string());
    println!("D15P1: {}", result);
    let result = puzzle2(INPUT_FILE.to_string());
    println!("D15P2: {}", result);
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_FILE: &str = "inputs/day15_test.txt";
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(40, puzzle1(TEST_INPUT_FILE.to_string()));
        assert_eq!(0, puzzle1(INPUT_FILE.to_string()));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(0, puzzle2(TEST_INPUT_FILE.to_string()));
        assert_eq!(0, puzzle2(INPUT_FILE.to_string()));
    }
}
