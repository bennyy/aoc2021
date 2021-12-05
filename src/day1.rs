use crate::file_util;
use std::collections::VecDeque;

fn puzzle1(file_path: String) -> i32 {
    let input_str = file_util::file_to_string(file_path);
    let input_vec = file_util::convert_string_of_ints_to_list(input_str);

    let mut no_increased = 0;
    let mut previous_depth = 0;

    // Day 1, Puzzle 1
    for depth in &input_vec {
        // Check if it increased and it's not the first measurement
        if *depth > previous_depth && previous_depth != 0 {
            no_increased += 1;
        }

        previous_depth = *depth
    }

    return no_increased;
}

fn puzzle2(file_path: String) -> i32 {
    let input_str = file_util::file_to_string(file_path);
    let input_vec = file_util::convert_string_of_ints_to_list(input_str);

    // Day 1, Puzzle 2
    let mut previous_depth = 0;
    let mut no_increased = 0;
    let mut sliding_window = VecDeque::new();
    for depth in &input_vec {
        sliding_window.push_back(*depth);

        // Fill until we have 3 elements
        if sliding_window.len() < 3 {
            continue;
        }

        // Remove the oldest one
        if sliding_window.len() > 3 {
            sliding_window.pop_front();
        }

        // Sum the data
        let sum: i32 = sliding_window.iter().sum();

        // Check if it increased and it's not the first measurement
        if sum > previous_depth && previous_depth != 0 {
            no_increased += 1;
        }

        previous_depth = sum
    }
    return no_increased;
}

pub fn run() {
    let result = puzzle1("inputs/day1.txt".to_string());
    println!("D1P1: {}", result);
    let result = puzzle2("inputs/day1.txt".to_string());
    println!("D1P2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(7, puzzle1("inputs/day1_p2_small.txt".to_string()));
        assert_eq!(1316, puzzle1("inputs/day1.txt".to_string()));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(5, puzzle2("inputs/day1_p2_small.txt".to_string()));
        assert_eq!(1344, puzzle2("inputs/day1.txt".to_string()));
    }
}
