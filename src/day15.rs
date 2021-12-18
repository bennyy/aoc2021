use crate::file_util;
use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;



use std::collections::BinaryHeap;

static INPUT_FILE: &str = "inputs/day15.txt";

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pos: usize,
    prio: i32,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .prio
            .cmp(&self.prio)
            .then_with(|| self.prio.cmp(&other.prio))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calc_risk_level(risk_level_map: &[u32], total_path: &[usize]) -> u64 {
    let mut ret_val = 0;
    for i in total_path.iter() {
        // Skip the first one!
        if *i != 0 {
            ret_val += risk_level_map.get(*i).unwrap();
        }
    }
    ret_val as u64
}

fn heuristic(start: (i32, i32), goal: (i32, i32)) -> i32 {
    let d = 1;
    let d2 = 1;
    let dx = (start.0 - goal.0).abs();
    let dy = (start.1 - goal.1).abs();
    d * (dx + dy) + (d2 - 2 * d) * cmp::min(dx, dy)
}

fn a_star(risk_level_map: &[u32], width: usize) -> Vec<usize> {
    let end_x = ((risk_level_map.len() - 1) % width) as i32;
    let end_y = ((risk_level_map.len() - 1) / width) as i32;

    let start_pos = (0, 0);
    let end_pos = (end_x, end_y);
    let end_i = risk_level_map.len() - 1;

    let mut open = BinaryHeap::new();
    let mut came_from: HashMap<usize, usize> = HashMap::new();
    let mut g_score: HashMap<usize, i32> = HashMap::new();
    let mut f_score: HashMap<usize, i32> = HashMap::new();
    g_score.insert(0, 0);
    f_score.insert(0, heuristic(start_pos, end_pos));

    let start_node = Node {
        pos: 0,
        prio: heuristic(start_pos, end_pos),
    };
    open.push(start_node);

    while !open.is_empty() {
        let min = open.pop().unwrap();
        let i = min.pos;

        // Check if i is the goal i.
        if i == end_i {
            let mut total_path: Vec<usize> = Vec::new();
            let mut curr = i;
            total_path.push(curr);
            while came_from.contains_key(&curr) {
                curr = *came_from.get(&curr).unwrap();
                total_path.push(curr);
            }
            return total_path;
        }

        // Find all neighbours.
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
                // Outside the limit..
                continue;
            }
            let i = x + width as i32 * y;
            neighbors.push(i as usize);
        }

        for neighbor in neighbors.iter_mut() {
            let current_gscore = *g_score.entry(i).or_insert(i32::MAX);
            let tentative_gscore = current_gscore + *risk_level_map.get(*neighbor).unwrap() as i32;
            let neighbor_gscore = *g_score.entry(*neighbor).or_insert(i32::MAX);

            if tentative_gscore < neighbor_gscore {
                came_from.entry(*neighbor).or_insert(usize::MAX);
                f_score.entry(*neighbor).or_insert(i32::MAX);

                let nx = (*neighbor % width) as i32;
                let ny = (*neighbor / width) as i32;

                *came_from.get_mut(neighbor).unwrap() = i;
                *g_score.get_mut(neighbor).unwrap() = tentative_gscore;

                let f = tentative_gscore + heuristic((nx, ny), end_pos);
                *f_score.get_mut(neighbor).unwrap() = f;

                open.push(Node {
                    pos: *neighbor,
                    prio: f,
                });
            }
        }
    }
    Vec::new()
}

fn puzzle1(file_path: String) -> u64 {
    let input_str = file_util::file_to_string(file_path);
    let input_str: Vec<&str> = input_str.split('\n').collect();

    let width = input_str.get(0).unwrap().chars().count();

    let mut risk_level_map: Vec<u32> = Vec::new();
    input_str.iter().for_each(|line| {
        risk_level_map.append(&mut line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    });

    let total_path = a_star(&risk_level_map, width);
    calc_risk_level(&risk_level_map, &total_path)
}

fn puzzle2(file_path: String) -> u64 {
    let input_str = file_util::file_to_string(file_path);
    let input_str: Vec<&str> = input_str.split('\n').collect();

    let width = input_str.get(0).unwrap().chars().count() * 5;
    let mut risk_level_map: Vec<u32> = Vec::new();

    for line in input_str {
        for n in 0..5 {
            let mut tmp_vec: Vec<u32> = line
                .chars()
                .map(|x| {
                    let digit = x.to_digit(10).unwrap() + n;

                    if digit > 9 {
                        return digit % 9;
                    }

                    digit
                })
                .collect();

            risk_level_map.append(&mut tmp_vec);
        }
    }

    let mut tmp_risk_level: Vec<u32> = Vec::new();
    for n in 1..5 {
        risk_level_map.iter().for_each(|x| {
            let mut digit = x + n;

            if digit > 9 {
                let asdf = digit % 9;
                digit = asdf;
            }

            tmp_risk_level.push(digit);
        });
    }
    risk_level_map.append(&mut tmp_risk_level);
    let total_path = a_star(&risk_level_map, width);
    calc_risk_level(&risk_level_map, &total_path)
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
        assert_eq!(315, puzzle2(TEST_INPUT_FILE.to_string()));
        assert_eq!(2844, puzzle2(INPUT_FILE.to_string()));
    }
}
