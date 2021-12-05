use crate::file_util;

#[derive(Debug)]
struct Power {
    bits: Vec<i32>,
}

impl Power {
    fn new() -> Power {
        Power { bits: Vec::new() }
    }

    fn push(&mut self, bit: i32) {
        self.bits.push(bit);
    }
}

fn puzzle1(file_path: String) -> i32 {
    let input_str = file_util::file_to_string(file_path);
    let input_lines: Vec<&str> = input_str.lines().collect();

    let mut power_list = Vec::new();
    // check the width of the bits
    let first_line = input_lines.get(0).unwrap();
    for _ in 0..first_line.len() {
        power_list.push(Power::new());
    }

    for input in input_lines {
        let intval = i32::from_str_radix(input, 2).unwrap();
        let bit_length = input.len();
        
        // Todo: Fix reverse..
        for n in (0..bit_length).rev() {
            let mask = 1 << n;
            let bit = (intval & mask) >> n;
            power_list.get_mut(n).unwrap().push(bit);
        }
    }

    let mut gamma_rate_str = "".to_string();
    let mut epsilon_rate_str = "".to_string();

    // Todo: Fix reverse..
    for power in power_list.iter().rev() {
        let no_of_zero_bits = power.bits.iter().filter(|&n| *n == 0).count();
        let no_of_one_bits = power.bits.iter().filter(|&n| *n == 1).count();

        if no_of_zero_bits > no_of_one_bits {
            gamma_rate_str.push_str("0");
            epsilon_rate_str.push_str("1");
        } else {
            gamma_rate_str.push_str("1");
            epsilon_rate_str.push_str("0");
        }
    }
    let gamma_rate = i32::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_rate = i32::from_str_radix(&epsilon_rate_str, 2).unwrap();
    return gamma_rate * epsilon_rate;
}

fn get_rating_index(power_vec: &Vec<Power>, size: i32, most_common_value: bool) -> usize {
    // At the start, include all indexes.
    let mut indexes: Vec<i32> = (0..size).collect();

    for power in power_vec.iter().rev() {

        // Get all items in the list based on the index-list. 
        let mut power_vec_tmp = Vec::new();
        for i in &indexes {
            let bit = power.bits.get(*i as usize).unwrap();
            power_vec_tmp.push(*bit);
        }

        // Calcuate amount of zeroes (0) and ones (1).
        let no_of_zero_bits = power_vec_tmp.iter().filter(|&n| *n == 0).count();
        let no_of_one_bits = power_vec_tmp.iter().filter(|&n| *n == 1).count();

        // If most common value, take the that with highest count.
        let which_bit;
        if no_of_zero_bits > no_of_one_bits {
            which_bit = if most_common_value { 0 } else { 1 };
        } else {
            which_bit = if most_common_value { 1 } else { 0 };
        }

        // Update the index with the new index who matches the digit.
        let mut temp_indexes = Vec::new();
        for i in &indexes
        {
            let b = power.bits.get(*i as usize).unwrap();
            if *b == which_bit {
                temp_indexes.push(*i);
            }
        }

        // Replace the index-vector with the new one.
        indexes.clear();
        indexes = temp_indexes.to_vec();

        if indexes.len() == 1 {
            // If one element left. We're done!
            return *indexes.first().unwrap() as usize;
        }
    }

    return 0;
}

fn puzzle2(file_path: String) -> i32 {
    let input_str = file_util::file_to_string(file_path);
    let input_lines: Vec<&str> = input_str.lines().collect();

    let mut power_list = Vec::new();
    // check the width of the bits
    let first_line = input_lines.get(0).unwrap();
    for _ in 0..first_line.len() {
        power_list.push(Power::new());
    }

    for input in &input_lines {
        let intval = i32::from_str_radix(input, 2).unwrap();
        let bit_length = input.len();
        
        // Todo: Fix reverse..
        for n in (0..bit_length).rev() {
            let mask = 1 << n;
            let bit = (intval & mask) >> n;
            power_list.get_mut(n).unwrap().push(bit);
        }
    }

    let size = input_lines.len() as i32;

    // Oxygen generator rating
    let i = get_rating_index(&power_list, size, true);
    let res = *input_lines.get(i).unwrap();
    let oxygen_generator_rating = i32::from_str_radix(res, 2).unwrap();
    
    // CO2 scrubber rating
    let i = get_rating_index(&power_list, size, false);
    let res = *input_lines.get(i).unwrap();
    let co2_scrubber_rating = i32::from_str_radix(res, 2).unwrap();

    return oxygen_generator_rating * co2_scrubber_rating;
}

pub fn run() {
    let result = puzzle1("inputs/day3.txt".to_string());
    println!("D3P1: {}", result);
    let result = puzzle2("inputs/day3.txt".to_string());
    println!("D3P2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(198, puzzle1("inputs/day3_test.txt".to_string()));
        assert_eq!(2724524, puzzle1("inputs/day3.txt".to_string()));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(230, puzzle2("inputs/day3_test.txt".to_string()));
        assert_eq!(2775870, puzzle2("inputs/day3.txt".to_string()));
    }
}
