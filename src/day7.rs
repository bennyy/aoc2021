use crate::file_util;

static INPUT_FILE: &str = "inputs/day7.txt";

fn mean(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort_unstable();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn calc_fuel_cost(input: &[i32], diff: i32, expensive: bool) -> i32 {
    let mut ret_fuel: i32 = 0;
    for i in input {
        if expensive {
            let fuel = *i - diff;
            let fuel_to_add: i32 = (1..fuel.abs() + 1).sum();
            ret_fuel += fuel_to_add;
        } else {
            ret_fuel += (*i - diff).abs();
        }
    }

    ret_fuel
}

fn puzzle1(file_path: String) -> i32 {
    let input_str = file_util::file_to_string(file_path);
    let mut input_lines: Vec<i32> = input_str
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let median = median(&mut input_lines);
    calc_fuel_cost(&input_lines, median, false)
}

fn puzzle2(file_path: String) -> i32 {
    let input_str = file_util::file_to_string(file_path);
    let input_lines: Vec<i32> = input_str
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mean = mean(&input_lines);
    let fuel_min = calc_fuel_cost(&input_lines, mean.floor() as i32, true);
    let fuel_max = calc_fuel_cost(&input_lines, mean.ceil() as i32, true);

    if fuel_min > fuel_max {
        return fuel_max;
    }
    fuel_min
}

pub fn run() {
    let result = puzzle1(INPUT_FILE.to_string());
    println!("D7P1: {}", result);
    let result = puzzle2(INPUT_FILE.to_string());
    println!("D7P2: {}", result);
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_FILE: &str = "inputs/day7_test.txt";
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(37, puzzle1(TEST_INPUT_FILE.to_string()));
        assert_eq!(348664, puzzle1(INPUT_FILE.to_string()));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(168, puzzle2(TEST_INPUT_FILE.to_string()));
        assert_eq!(100220525, puzzle2(INPUT_FILE.to_string()));
    }
}
