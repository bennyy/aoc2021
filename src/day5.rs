use crate::file_util;
use std::cmp;

#[derive(Debug)]
struct Coordinate {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

fn puzzle1(file_path: String, diagonal: bool) -> u32 {
    let width = 999;
    let height = 999;
    let mut ocean_floor = vec![0; width * height];

    let input_str = file_util::file_to_string(file_path.clone());
    let input_lines: Vec<&str> = input_str.lines().collect();
    let mut coordinates: Vec<Coordinate> = Vec::new();

    for line in input_lines {
        let coords: Vec<i32> = str::replace(line, " -> ", ",")
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        if coords.len() == 4 {
            coordinates.push(Coordinate {
                x1: *coords.get(0).unwrap(),
                y1: *coords.get(1).unwrap(),
                x2: *coords.get(2).unwrap(),
                y2: *coords.get(3).unwrap(),
            })
        } else {
            panic!("Strange input data! Data: {}", line);
        }
    }

    for c in coordinates {
        if c.x1 == c.x2 {
            let min_y = cmp::min(c.y1, c.y2);
            let max_y = cmp::max(c.y1, c.y2);

            for n in min_y..max_y + 1 {
                let i = c.x1 + width as i32 * n;
                let b = ocean_floor.get_mut(i as usize).unwrap();
                *b += 1;
            }
        } else if c.y1 == c.y2 {
            let min_x = cmp::min(c.x1, c.x2);
            let max_x = cmp::max(c.x1, c.x2);

            for n in min_x..max_x + 1 {
                let i = n + height as i32 * (c.y1);
                let b = ocean_floor.get_mut(i as usize).unwrap();
                *b += 1;
            }
        } else {
            // Diagnoal coordinate.
            if diagonal {
                let ydiff = c.y2 - c.y1;
                let xdiff = c.x2 - c.x1;

                let mut x = 0;
                let mut y = 0;
                let steps = ((c.x1 - c.x2) as i32).abs();
                for i in 0..steps + 1 {
                    if ydiff > 0 && xdiff > 0 {
                        x = c.x1 + i;
                        y = c.y1 + i;
                    } else if ydiff < 0 && xdiff > 0 {
                        x = c.x1 + i;
                        y = c.y1 - i;
                    } else if ydiff < 0 && xdiff < 0 {
                        x = c.x1 - i;
                        y = c.y1 - i;
                    } else if ydiff > 0 && xdiff < 0 {
                        x = c.x1 - i;
                        y = c.y1 + i;
                    }
                    let i = x + height as i32 * y;
                    let b = ocean_floor.get_mut(i as usize).unwrap();
                    *b += 1;
                }
            }
        }
    }
    let res = ocean_floor.iter().filter(|&n| *n > 1).count();
    return res as u32;
}

fn puzzle2(file_path: String) -> u32 {
    return puzzle1(file_path, true);
}

pub fn run() {
    let result = puzzle1("inputs/day5.txt".to_string(), false);
    println!("D5P1: {}", result);
    let result = puzzle2("inputs/day5.txt".to_string());
    println!("D5P2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(5, puzzle1("inputs/day5_test.txt".to_string(), false));
        assert_eq!(5632, puzzle1("inputs/day5.txt".to_string(), false));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(12, puzzle2("inputs/day5_test.txt".to_string()));
        assert_eq!(22213, puzzle2("inputs/day5.txt".to_string()));
    }
}
