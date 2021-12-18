use crate::file_util;
use std::cmp;
use std::collections::HashMap;

static INPUT_FILE: &str = "inputs/day15.txt";

fn heuristic(start: (i32, i32), goal: (i32, i32)) -> i32 {
    let d = 1;
    let d2 = 1;
    let dx = (start.0 - goal.0).abs();
    let dy = (start.1 - goal.1).abs();
    return d * (dx + dy) + (d2 - 2 * d) * cmp::min(dx, dy);
}

fn puzzle1(file_path: String) -> u64 {
    let input_str = file_util::file_to_string(file_path);
    let input_str: Vec<&str> = input_str.split('\n').collect();

    let width = input_str.get(0).unwrap().chars().count();

    let mut risk_level_map: Vec<u32> = Vec::new();
    input_str.iter().for_each(|line| {
        risk_level_map.append(&mut line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    });

    let x = ((risk_level_map.len() - 1) % width) as i32;
    let y = ((risk_level_map.len() - 1) / width) as i32;

    let start_pos = (0, 0);
    let end_pos = (x, y);

    let mut open: Vec<usize> = Vec::new();
    let mut came_from: HashMap<usize, usize> = HashMap::new();
    let mut g_score: HashMap<usize, usize> = HashMap::new();
    let mut f_score: HashMap<usize, i32> = HashMap::new();
    g_score.insert(0, 0);
    f_score.insert(0, heuristic(start_pos, end_pos));

    open.push(0);
    while !open.is_empty() {
        let min = open
            .iter_mut()
            .min_by(|a, b| f_score.get(a).unwrap().cmp(&f_score.get(b).unwrap()))
            .unwrap();

        let i = *min;
        let x = (i % width) as i32;
        let y = (i / width) as i32;
        open.retain(|&x| i != x);
        if end_pos == (x, y) {
            let mut total_path: Vec<usize> = Vec::new();
            let mut curr = i;
            total_path.push(curr);
            while came_from.contains_key(&curr) {
                curr = *came_from.get(&curr).unwrap();
                total_path.push(curr);
            }

            let mut ret_val = 0;
            for p in total_path.iter() {
                if *p != 0 {
                    ret_val += risk_level_map.get(*p).unwrap();
                }
            }
            return ret_val as u64;
        }
        let directions = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut neighbors: Vec<usize> = Vec::new();
        for v in directions {
            let x = (i % width) as i32 + v.0;
            let y = (i / width) as i32 + v.1;

            if x >= width as i32
                || x < 0
                || y < 0
                || y >= risk_level_map.len() as i32 / width as i32
            {
                continue;
            }
            let i = x + width as i32 * y;
            neighbors.push(i as usize);
        }

        for neighbor in neighbors.iter_mut() {
            let tentative_gscore = *g_score.entry(i).or_insert(usize::MAX) as u32
                + risk_level_map.get(*neighbor).unwrap();
            let neighbor_gscore = *g_score.entry(*neighbor).or_insert(usize::MAX);

            if tentative_gscore < neighbor_gscore as u32 {
                came_from.entry(*neighbor).or_insert(usize::MAX);
                *came_from.get_mut(neighbor).unwrap() = i;
                let nx = (*neighbor % width) as i32;
                let ny = (*neighbor / width) as i32;

                *g_score.get_mut(neighbor).unwrap() = tentative_gscore as usize;

                f_score.entry(*neighbor).or_insert(i32::MAX);

                *f_score.get_mut(neighbor).unwrap() =
                    tentative_gscore as i32 + heuristic((nx, ny), end_pos) as i32;
                if !open.contains(neighbor) {
                    open.push(*neighbor);
                }
            }
        }
    }
    0
}

fn puzzle2(_file_path: String) -> u64 {
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
        assert_eq!(429, puzzle1(INPUT_FILE.to_string()));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(0, puzzle2(TEST_INPUT_FILE.to_string()));
        //assert_eq!(0, puzzle2(INPUT_FILE.to_string()));
    }
}
