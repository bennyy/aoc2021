use crate::file_util;

static INPUT_FILE: &str = "inputs/day6.txt";

fn puzzle1(file_path: String) -> u32 {
    let input_str = file_util::file_to_string(file_path.clone());
    let input_lines: Vec<&str> = input_str.lines().collect();
    return 0;
}

fn puzzle2(file_path: String) -> u32 {
    return 0;
}

pub fn run() {
    let result = puzzle1(INPUT_FILE.to_string());
    println!("D6P1: {}", result);
    let result = puzzle2(INPUT_FILE.to_string());
    println!("D6P2: {}", result);
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_FILE: &str = "inputs/day6_test.txt";
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(1, puzzle1(TEST_INPUT_FILE.to_string()));
        assert_eq!(1, puzzle1(INPUT_FILE.to_string()));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(1, puzzle2(TEST_INPUT_FILE.to_string()));
        assert_eq!(1, puzzle2(INPUT_FILE.to_string()));
    }
}
