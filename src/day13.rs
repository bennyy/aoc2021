use crate::file_util;
use std::collections::HashSet;

static INPUT_FILE: &str = "inputs/day13.txt";

fn puzzle(file_path: String, is_puzzle_2: bool) -> u64 {
    let input_str = file_util::file_to_string(file_path);
    let input_str: Vec<&str> = input_str.split("\n\n").collect();

    let mut coords: Vec<_> = input_str
        .first()
        .unwrap()
        .split('\n')
        .map(|x| {
            let mut split = x.split(',');
            
            (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();

    let folds: Vec<_> = input_str
        .clone()
        .last()
        .unwrap()
        .split('\n')
        .map(|x| {
            let mut split = x.split('=');
            
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();
    let mut width = coords.iter().max().unwrap().0;
    let mut height = coords.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

    for fold in &folds {
        // Remove the text..
        let axis = str::replace(&*fold.0, "fold along ", "");
        let split_coord = fold.1;

        if axis == "x" {
            
            for point in coords.iter_mut().filter(|x| x.0 > split_coord) {
                let digit = point.0 % split_coord;
                if digit == 0 {
                    point.0 = 0;
                } else {
                    point.0 = split_coord - digit;
                }
            }
            width -= split_coord;
        } else if axis == "y" {
            
            for point in coords.iter_mut().filter(|x| x.1 > split_coord) {
                let digit = point.1 % split_coord;
                if digit == 0 {
                    point.1 = 0;
                } else {
                    point.1 = split_coord - digit;
                }
            }
            height -= split_coord;
        }

        if !is_puzzle_2 {
            break;
        }
    }

    // Collect the uniques.
    let coords: HashSet<_> = coords.into_iter().collect();
    let number_of_dots = coords.len() as u64;
    if !is_puzzle_2 {
        return number_of_dots;
    }
    // Convert it to a 1D / 2D-array.
    let mut zero_vec = vec![" "; (width * height) as usize];
    for point in coords.into_iter() {
        let x = point.0;
        let y = point.1;
        let i = x + width * y;
        *zero_vec.get_mut(i as usize).unwrap() = "#";
    }

    for chunk in zero_vec.chunks(width as usize) {
        let joined = chunk.join("");
        println!("{}", joined);
    }
    number_of_dots
}

pub fn run() {
    let result = puzzle(INPUT_FILE.to_string(), false);
    println!("D13P1: {}", result);
    let result = puzzle(INPUT_FILE.to_string(), true);
    println!("D13P2: {}", result);
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_FILE: &str = "inputs/day13_test.txt";
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(17, puzzle(TEST_INPUT_FILE.to_string(), false));
        assert_eq!(602, puzzle(INPUT_FILE.to_string(), false));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(16, puzzle(TEST_INPUT_FILE.to_string(), true));
        assert_eq!(92, puzzle(INPUT_FILE.to_string(), true));
    }
}
