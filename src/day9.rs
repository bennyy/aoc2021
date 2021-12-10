use crate::file_util;

static INPUT_FILE: &str = "inputs/day9.txt";

fn get_tubes(lava_tubes: &[u32], width: usize) -> Vec<(usize, u32)> {
    let mut result: Vec<(usize, u32)> = Vec::new();

    for (index, current_lava_tube) in lava_tubes.iter().enumerate() {
        let top = index.checked_sub(width);
        let mut left = index.checked_sub(1);
        let mut bottom = Some(index + width);
        let mut right = Some(index + 1);

        if right.is_some() && index / width != right.unwrap() / width {
            right = None
        }

        if left.is_some() && index / width != left.unwrap() / width {
            left = None
        }

        if bottom.is_some() && bottom.unwrap() >= lava_tubes.len() {
            bottom = None;
        }

        let mut left_tube = 99;
        if let Some(l) = left {
            left_tube = *lava_tubes.get(l).unwrap();
        }

        let mut right_tube = 99;
        if let Some(l) = right {
            right_tube = *lava_tubes.get(l).unwrap();
        }

        let mut top_tube = 99;
        if let Some(l) = top {
            top_tube = *lava_tubes.get(l).unwrap();
        }

        let mut bottom_tube = 99;
        if let Some(l) = bottom {
            bottom_tube = *lava_tubes.get(l).unwrap();
        }

        if left_tube > *current_lava_tube
            && right_tube > *current_lava_tube
            && top_tube > *current_lava_tube
            && bottom_tube > *current_lava_tube
        {
            result.push((index, current_lava_tube + 1))
        }
    }

    result
}

fn puzzle1(file_path: String) -> u32 {
    let input_str = file_util::file_to_string(file_path);
    let input_str: Vec<&str> = input_str.split('\n').collect();

    let width = input_str.get(0).unwrap().chars().count();

    let mut lava_tubes: Vec<u32> = Vec::new();
    for line in input_str {
        lava_tubes.append(&mut line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }

    let mut res = 0;
    let result = get_tubes(&lava_tubes, width);
    result.iter().for_each(|(_index, tube)| res += tube);
    res
}

fn fill(data: &mut Vec<u32>, basin_index: usize, width: usize) -> u32 {
    let fill_number = 1_u32;
    let mut queue: Vec<usize> = Vec::new();
    let mut basin_size = 0_u32;

    queue.push(basin_index);
    while !queue.is_empty() {
        let index = queue.pop().unwrap();

        let top = index.checked_sub(width);
        let bottom = index + width;
        let left = index.checked_sub(1);
        let right = index + 1;

        if let Some(top) = top {
            let top_data = data.get_mut(top).unwrap();
            if *top_data != fill_number && *top_data < 9 {
                *top_data = fill_number;
                queue.push(top);
                basin_size += 1;
            }
        }

        if bottom < data.len() {
            let bottom_data = data.get_mut(bottom).unwrap();
            if *bottom_data != fill_number && *bottom_data < 9 {
                *bottom_data = fill_number;
                queue.push(bottom);
                basin_size += 1;
            }
        }

        if let Some(left) = left {
            if index / width == left / width {
                // Check it's on the same row
                let left_data = data.get_mut(left).unwrap();
                if *left_data != fill_number && *left_data < 9 {
                    *left_data = fill_number;
                    queue.push(left);
                    basin_size += 1;
                }
            }
        }

        if right < data.len() && index / width == right / width {
            // Check it's on the same row
            let right_data = data.get_mut(right).unwrap();
            if *right_data != fill_number && *right_data < 9 {
                *right_data = fill_number;
                queue.push(right);
                basin_size += 1;
            }
        }
    }

    basin_size
}

fn puzzle2(file_path: String) -> u32 {
    let input_str = file_util::file_to_string(file_path);
    let input_str: Vec<&str> = input_str.split('\n').collect();
    let width = input_str.get(0).unwrap().chars().count();

    let mut lava_tubes: Vec<u32> = Vec::new();
    for line in input_str {
        lava_tubes.append(&mut line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }

    // Get all the low points
    let tubes = get_tubes(&lava_tubes, width);

    // Convert it all non 9-digits to zero.
    for tube in lava_tubes.iter_mut() {
        if *tube != 9 {
            *tube = 0;
        }
    }

    let mut basin_sizes: Vec<u32> = Vec::new();
    for (index, _tube) in tubes {
        // 9 == Wall
        if *lava_tubes.get(index).unwrap() != 9 {
            let basin_size = fill(&mut lava_tubes, index, width);
            basin_sizes.push(basin_size);
        }
    }

    basin_sizes.sort_unstable();
    let mut result = 1;
    basin_sizes.iter().rev().take(3).for_each(|x| result *= *x);
    result
}

pub fn run() {
    let result = puzzle1(INPUT_FILE.to_string());
    println!("D9P1: {}", result);
    let result = puzzle2(INPUT_FILE.to_string());
    println!("D9P2: {}", result);
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_FILE: &str = "inputs/day9_test.txt";
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(15, puzzle1(TEST_INPUT_FILE.to_string()));
        assert_eq!(566, puzzle1(INPUT_FILE.to_string()));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(1134, puzzle2(TEST_INPUT_FILE.to_string()));
        assert_eq!(891684, puzzle2(INPUT_FILE.to_string()));
    }
}
