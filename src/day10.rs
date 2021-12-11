use crate::file_util;

static INPUT_FILE: &str = "inputs/day10.txt";

fn get_inverse_char(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => c,
    }
}

fn syntax_check(file_path: String) -> (u64, u64) {
    let input_str = file_util::file_to_string(file_path);
    let input_str: Vec<&str> = input_str.split('\n').collect();

    let mut score_syntax_error = 0; // Puzzle 1
    let mut scores_completion = Vec::new(); // Puzzle 2
    for line in input_str {
        let mut stack = Vec::new();
        let mut bad_syntax = false;
        let input_chars: Vec<_> = line.chars().collect();

        for c in input_chars {
            if c == ')' || c == ']' || c == '}' || c == '>' {
                // Something closing now.
                let last_closing_char = stack.pop().unwrap(); // Pop last element

                if !((last_closing_char == '(' && c == ')')
                    || (last_closing_char == '[' && c == ']')
                    || (last_closing_char == '{' && c == '}')
                    || (last_closing_char == '<' && c == '>'))
                {
                    // Syntax error!
                    match c {
                        ')' => score_syntax_error += 3,
                        ']' => score_syntax_error += 57,
                        '}' => score_syntax_error += 1197,
                        '>' => score_syntax_error += 25137,
                        _ => panic!("Unknown char!"),
                    };
                    bad_syntax = true;
                    break; // Exit the parsning!
                }
            } else {
                stack.push(c); // Continue!
            }
        }

        if bad_syntax {
            continue;
        }

        // Reverse the stack and get the counter part chars
        stack = stack.iter().rev().map(|x| get_inverse_char(*x)).collect();
        let mut score = 0_u64;
        stack.iter().for_each(|c| {
            score *= 5;
            match *c {
                ')' => score += 1,
                ']' => score += 2,
                '}' => score += 3,
                '>' => score += 4,
                _ => {}
            };
        });
        scores_completion.push(score);
    }

    scores_completion.sort_unstable();
    let middle = (scores_completion.len() - 1) / 2;
    let average_score = *scores_completion.get(middle as usize).unwrap();

    (score_syntax_error, average_score)
}

fn puzzle1(file_path: String) -> u64 {
    let (score_syntax_error, _) = syntax_check(file_path);
    score_syntax_error
}

fn puzzle2(file_path: String) -> u64 {
    let (_, completion_score) = syntax_check(file_path);
    completion_score
}

pub fn run() {
    let result = puzzle1(INPUT_FILE.to_string());
    println!("D10P1: {}", result);
    let result = puzzle2(INPUT_FILE.to_string());
    println!("D10P2: {}", result);
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_FILE: &str = "inputs/day10_test.txt";
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(26397, puzzle1(TEST_INPUT_FILE.to_string()));
        assert_eq!(392043, puzzle1(INPUT_FILE.to_string()));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(288957, puzzle2(TEST_INPUT_FILE.to_string()));
        assert_eq!(1605968119, puzzle2(INPUT_FILE.to_string()));
    }
}
