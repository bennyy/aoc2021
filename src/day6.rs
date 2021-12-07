use crate::file_util;

static INPUT_FILE: &str = "inputs/day6.txt";

fn puzzle(file_path: String, days: u32) -> u64 {
    let input_str = file_util::file_to_string(file_path);
    let input_lines: Vec<usize> = input_str
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    // Convert the data to a "map" where the key is the timer and the value is how many fishes
    let mut lanternfishes = vec![0; 9];
    for fish in input_lines {
        *lanternfishes.get_mut(fish).unwrap() += 1;
    }

    for _ in 0..days {
        // Split the array in to two parts. Old fishes and the new fishes.
        let (old_fishes, new_fishes) = lanternfishes.split_at_mut(7);

        // Save all data before it shifted away and replaced and edited.
        let fishes_to_spawn = *old_fishes.get(0).unwrap();
        let fishes_to_merge = *new_fishes.get(0).unwrap();

        // Shift the list one step to the left
        old_fishes.rotate_left(1);
        new_fishes.rotate_left(1);

        // Add the new fishes from 7 timer-fishes into 6 timer-fishes position.
        *old_fishes.get_mut(6).unwrap() += fishes_to_merge;
        // All 0 timer-fishes, spawn fishes in timer 8.
        *new_fishes.get_mut(1).unwrap() = fishes_to_spawn;
    }
    return lanternfishes.iter().sum();
}

pub fn run() {
    let result = puzzle(INPUT_FILE.to_string(), 80);
    println!("D6P1: {}", result);
    let result = puzzle(INPUT_FILE.to_string(), 256);
    println!("D6P2: {}", result);
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_FILE: &str = "inputs/day6_test.txt";
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(5934, puzzle(TEST_INPUT_FILE.to_string(), 80));
        assert_eq!(374927, puzzle(INPUT_FILE.to_string(), 80));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(26984457539, puzzle(TEST_INPUT_FILE.to_string(), 256));
        assert_eq!(1687617803407, puzzle(INPUT_FILE.to_string(), 256));
    }
}
