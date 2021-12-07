use crate::file_util;

#[derive(Debug)]
struct BingoDigit {
    number: u32,
    marked: bool,
}

fn parse_drawn_number(file_path: String) -> Vec<u32> {
    let input_str = file_util::file_to_string(file_path);
    let input_lines: Vec<&str> = input_str.lines().collect();

    let numbers_to_draw: Vec<u32> = input_lines
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    numbers_to_draw
}

fn parse_bingo_boards(file_path: String) -> Vec<Vec<BingoDigit>> {
    let input_str = file_util::file_to_string(file_path);
    let input_lines: Vec<&str> = input_str.lines().collect();

    let mut bingo_boards: Vec<Vec<BingoDigit>> = Vec::new();

    // Skip the first ones..
    let bingo_numbers = &input_lines[1..];
    let mut iter = bingo_numbers.iter();
    for _ in (0..bingo_numbers.len()).step_by(6) {
        let mut tmp = " ".to_string();
        tmp.push_str(iter.next().unwrap());
        tmp.push(' ');
        tmp.push_str(iter.next().unwrap());
        tmp.push(' ');
        tmp.push_str(iter.next().unwrap());
        tmp.push(' ');
        tmp.push_str(iter.next().unwrap());
        tmp.push(' ');
        tmp.push_str(iter.next().unwrap());
        tmp.push(' ');
        tmp.push_str(iter.next().unwrap());
        tmp.push(' ');

        let tmp: Vec<BingoDigit> = tmp
            .split_whitespace()
            .map(|x| BingoDigit {
                number: x.parse::<u32>().unwrap(),
                marked: false,
            })
            .collect();
        bingo_boards.push(tmp);
    }

    bingo_boards
}

fn dutt(number: u32, bingo_board: &mut Vec<BingoDigit>) {
    for item in bingo_board.iter_mut() {
        if item.number == number {
            item.marked = true;
        }
    }
}

fn check_bingo(bingo_board: &[BingoDigit]) -> bool {
    if bingo_board.len() != 25 {
        println!("{:?}", bingo_board);
        assert_eq!(true, false);
    }

    // Check vertical
    for n in 0..5 {
        let row = &bingo_board[n * 5..n * 5 + 5];
        let c = row.iter().filter(|&n| n.marked).count();
        if c == 5 {
            return true;
        }
    }

    // check horizontal
    for n in 0..5 {
        let mut bingo = true;
        for m in 0..5 {
            let col = &bingo_board[n + m * 5];
            if !col.marked {
                bingo = false;
                break;
            }
        }

        if bingo {
            return true;
        }
    }
    false
}

fn calculate_score(bingo_board: &[BingoDigit]) -> u32 {
    let mut sum = 0;
    for b in bingo_board {
        if !b.marked {
            sum += b.number;
        }
    }
    sum
}

fn puzzle1(file_path: String) -> u32 {
    let numbers_to_draw = parse_drawn_number(file_path.clone());
    let mut bingo_boards = parse_bingo_boards(file_path);

    for number in numbers_to_draw {
        for bingo_board in bingo_boards.iter_mut() {
            dutt(number, bingo_board);
            let bingo = check_bingo(bingo_board);
            if bingo {
                let score = calculate_score(bingo_board);
                return score * number;
            }
        }
    }

    0
}

fn puzzle2(file_path: String) -> u32 {
    let numbers_to_draw = parse_drawn_number(file_path.clone());
    let mut bingo_boards = parse_bingo_boards(file_path);

    let mut last_score = 0;
    let mut bingo_done = Vec::new();

    for number in numbers_to_draw {
        for (i, bingo_board) in bingo_boards.iter_mut().enumerate() {
            if bingo_done.contains(&i) {
                continue;
            }

            dutt(number, bingo_board);
            let bingo = check_bingo(bingo_board);
            if bingo {
                let score = calculate_score(bingo_board);
                last_score = score * number;

                bingo_done.push(i);
            }
        }
    }

    last_score
}

pub fn run() {
    let result = puzzle1("inputs/day4.txt".to_string());
    println!("D4P1: {}", result);
    let result = puzzle2("inputs/day4.txt".to_string());
    println!("D4P2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(4512, puzzle1("inputs/day4_test.txt".to_string()));
        assert_eq!(39984, puzzle1("inputs/day4.txt".to_string()));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(1924, puzzle2("inputs/day4_test.txt".to_string()));
        assert_eq!(8468, puzzle2("inputs/day4.txt".to_string()));
    }
}
