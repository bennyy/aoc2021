use crate::file_util;

static INPUT_FILE: &str = "inputs/day11.txt";

fn plopp(octos: &mut [u32], queue: &mut Vec<usize>, index: Option<usize>) {
    if let Some(index) = index {
        let data = octos.get_mut(index).unwrap();
        // Don't touch the zeroes.
        if *data != 0 {
            *data += 1;

            // If 10, Flash!!
            if *data == 10 {
                *data = 0;

                // Push this index to the queue, it's neighbours need to be updated next.
                queue.push(index);
            }
        }
    }
}

fn tick(octos: &mut [u32], width: usize) -> u64 {
    let mut queue: Vec<usize> = Vec::new();

    // Step all
    octos.iter_mut().for_each(|o| *o += 1);

    // Push all flashes to the queue
    octos
        .iter_mut()
        .enumerate()
        .filter(|(_, x)| **x > 9)
        .for_each(|(index, octo)| {
            queue.push(index);
            *octo = 0;
        });

    while !queue.is_empty() {
        let index = queue.pop().unwrap();

        // Top
        let top = index.checked_sub(width);
        let mut top_left = None;
        let mut top_right = None;
        if let Some(top) = top {
            top_left = top.checked_sub(1);
            top_right = top.checked_add(1);

            if let Some(tr) = top_right {
                if tr / width != top / width {
                    top_right = None;
                }
            }

            if let Some(tl) = top_left {
                if tl / width != top / width {
                    top_left = None;
                }
            }
        }

        // Bottom
        let mut bottom = None;
        let mut bottom_left = None;
        let mut bottom_right = None;

        if index + width < octos.len() {
            bottom = Some(index + width);
        }

        if let Some(bottom) = bottom {
            // Check so it's not on the last row.
            if (bottom - 1) / width == bottom / width {
                // Check if Bottom left is on the same line.
                bottom_left = Some(bottom - 1);
            }

            if (bottom + 1) / width == bottom / width {
                // Check if Bottom Right is on the same line.
                bottom_right = Some(bottom + 1);
            }
        }

        // Left
        let mut left = index.checked_sub(1);
        if let Some(l) = left {
            // Check if left is on the same row
            if index / width != l / width {
                left = None;
            }
        }

        // Right
        let mut right = None;
        if (index + 1) < octos.len() {
            // Check if right is on the same row
            if index / width == (index + 1) / width {
                right = Some(index + 1);
            }
        }

        plopp(octos, &mut queue, top);
        plopp(octos, &mut queue, top_left);
        plopp(octos, &mut queue, top_right);
        plopp(octos, &mut queue, bottom);
        plopp(octos, &mut queue, bottom_left);
        plopp(octos, &mut queue, bottom_right);
        plopp(octos, &mut queue, left);
        plopp(octos, &mut queue, right);
    }

    let zeroes_count = octos.iter().filter(|&n| *n == 0).count() as u64;
    zeroes_count
}

fn puzzle1(file_path: String) -> u64 {
    let input_str = file_util::file_to_string(file_path);
    let input_str: Vec<&str> = input_str.split('\n').collect();
    let width = input_str.get(0).unwrap().chars().count();

    let mut octos: Vec<u32> = Vec::new();
    input_str.iter().for_each(|line| {
        octos.append(&mut line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    });

    let mut tot_flashes = 0;
    let steps = 100;

    for _ in 0..steps {
        let flashes = tick(&mut octos, width);
        tot_flashes += flashes;
    }

    tot_flashes
}

fn puzzle2(file_path: String) -> u64 {
    let input_str = file_util::file_to_string(file_path);
    let input_str: Vec<&str> = input_str.split('\n').collect();
    let width = input_str.get(0).unwrap().chars().count();

    let mut octos: Vec<u32> = Vec::new();
    input_str.iter().for_each(|line| {
        octos.append(&mut line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    });

    let mut all_flashing = false;
    let mut step = 0;

    while !all_flashing {
        step += 1;
        let flashes = tick(&mut octos, width);
        if flashes == octos.len() as u64 {
            all_flashing = true;
        }
    }

    step
}

pub fn run() {
    let result = puzzle1(INPUT_FILE.to_string());
    println!("D11P1: {}", result);
    let result = puzzle2(INPUT_FILE.to_string());
    println!("D11P2: {}", result);
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_FILE: &str = "inputs/day11_test.txt";
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(1656, puzzle1(TEST_INPUT_FILE.to_string()));
        assert_eq!(1694, puzzle1(INPUT_FILE.to_string()));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(195, puzzle2(TEST_INPUT_FILE.to_string()));
        assert_eq!(346, puzzle2(INPUT_FILE.to_string()));
    }
}
