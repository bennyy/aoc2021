use crate::file_util;
use std::collections::HashSet;

static INPUT_FILE: &str = "inputs/day8.txt";

fn puzzle1(file_path: String) -> i32 {
    let input_str = file_util::file_to_string(file_path);

    let input_str2: Vec<&str> = input_str.split('\n').collect();
    let mut res = 0;
    for istr in input_str2 {
        let splitted_input = istr.split(" | ");
        let vec: Vec<&str> = splitted_input.collect();

        let signal_patterns = vec.first().unwrap();
        let output = vec.last().unwrap();

        let vec: Vec<&str> = output.split(' ').collect();

        for v in vec {
            if v.len() == 2 || v.len() == 3 || v.len() == 4 || v.len() == 7 {
                res += 1
            }
        }
    }

    res
}

#[derive(Debug)]
struct SSD {
    a: char,
    b: char,
    c: char,
    d: char,
    e: char,
    f: char,
    g: char
}

impl SSD {
    fn get_number(&mut self, string: String) -> &str {
        let mut input_chars: Vec<char> = string.chars().collect();
        input_chars.sort_unstable();

        let mut zero_str = vec![self.a, self.b, self.c, self.e, self.f, self.g];
        zero_str.sort_unstable();

        let mut one_str = vec![self.c, self.f];
        one_str.sort_unstable();

        let mut two_str = vec![self.a, self.c, self.d, self.e, self.g];
        two_str.sort_unstable();

        let mut three_str = vec![self.a, self.c, self.d, self.f, self.g];
        three_str.sort_unstable();

        let mut four_str = vec![self.b, self.c, self.d, self.f];
        four_str.sort_unstable();

        let mut five_str = vec![self.a, self.b, self.d, self.f, self.g];
        five_str.sort_unstable();

        let mut six_str = vec![self.a, self.b, self.d, self.e, self.f, self.g];
        six_str.sort_unstable();

        let mut seven_str = vec![self.a, self.c, self.f];
        seven_str.sort_unstable();

        let mut eight_str = vec![self.a, self.b, self.c, self.d, self.e, self.f, self.g];
        eight_str.sort_unstable();

        let mut nine_str = vec![self.a, self.b, self.c, self.d, self.f, self.g];
        nine_str.sort_unstable();

        if input_chars == one_str {
             "1"
        } else if input_chars == two_str {
             "2"
        } else if input_chars == three_str {
             "3"
        } else if input_chars == four_str {
             "4"
        } else if input_chars == five_str {
             "5"
        } else if input_chars == six_str {
             "6"
        } else if input_chars == seven_str {
            "7"
        } else if input_chars == eight_str {
            "8"
        } else if input_chars == nine_str {
            "9"
        } else if input_chars == zero_str {
            "0"
        } else {
            "-1"
        }
    }
}

fn puzzle2(file_path: String) -> i32 {
    let input_str = file_util::file_to_string(file_path);
    let input_str: Vec<&str> = input_str.split('\n').collect();

    let mut res = 0;

    for line in input_str {
        let splitted_input : Vec<&str> = line.split(" | ").collect();

        let signal_patterns = splitted_input.first().unwrap();
        let output_line = splitted_input.last().unwrap();
        let outputs: Vec<&str> = output_line.split(' ').collect();

        //  aaaa   
        // b    c 
        // b    c 
        //  dddd  
        // e    f  
        // e    f 
        //  gggg  

        let mut vec: Vec<&str> = signal_patterns.split(' ').collect();
        //vec.sort_by(|a, b| a.len().cmp(&b.len()));
        vec.sort_by_key(|a| a.len());

        let c_f: Vec<_> = vec.get(0).unwrap().chars().collect(); // 1
        let a_c_f: HashSet<_> = vec.get(1).unwrap().chars().collect(); // 7
        let b_c_d_f: HashSet<_> = vec.get(2).unwrap().chars().collect(); // 4
        let a_b_c_d_e_f_g: HashSet<_> = vec.last().unwrap().chars().collect(); // 8

        let a: Vec<_> = a_c_f.iter().filter(|k| !c_f.contains(k)).collect();
        let e_g: Vec<_> = a_b_c_d_e_f_g.iter().filter(|k| !b_c_d_f.contains(k)).filter(|k| !a.contains(k)).collect();

        // Find 0, 6, 9 and subtract with the 8. To find C, D and E
        let mut c_d_e: Vec<char> = Vec::new();
        for v in vec.clone() {
            let chars: Vec<char> = v.chars().collect();
            let diff: Vec<_> = a_b_c_d_e_f_g.clone().into_iter().filter(|item| !chars.contains(item)).collect();

            if diff.len() == 1 {
                c_d_e.push(*diff.last().unwrap());
            }
        }

        let c_d: Vec<_> = c_d_e.iter().filter(|k| !e_g.contains(k)).collect();
        let e: Vec<_> = c_d_e.iter().filter(|k| !c_d.contains(k)).collect();
        let c: Vec<_> = c_d.iter().filter(|k| c_f.contains(k)).collect();
        let g: Vec<_> = e_g.iter().filter(|k| !e.contains(k)).collect();
        let d: Vec<_> = c_d.iter().filter(|k| !c.contains(k)).collect();
        let f: Vec<_> = c_f.iter().filter(|k| !c.contains(&k)).collect();
        let b: Vec<_> = b_c_d_f.iter().filter(|k| !c_d.contains(k)).filter(|k| !f.contains(k)).collect();

        let mut ssd = SSD {
            a: **a.get(0).unwrap(),
            b: **b.first().unwrap(),
            c: ***c.first().unwrap(),
            d: ***d.first().unwrap(),
            e: **e.first().unwrap(),
            f: **f.first().unwrap(),
            g: ***g.first().unwrap()
        };

        let mut output_val_str = "".to_string();
        for output in outputs {
            let digit = ssd.get_number(output.to_string());
            output_val_str.push_str(digit);
        }

        res += output_val_str.parse::<i32>().unwrap();

    }

    res
}

pub fn run() {
    let result = puzzle1(INPUT_FILE.to_string());
    println!("D8P1: {}", result);
    let result = puzzle2(INPUT_FILE.to_string());
    println!("D8P2: {}", result);
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_FILE: &str = "inputs/day8_test.txt";
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(26, puzzle1(TEST_INPUT_FILE.to_string()));
        assert_eq!(521, puzzle1(INPUT_FILE.to_string()));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(61229, puzzle2(TEST_INPUT_FILE.to_string()));
        assert_eq!(1016804, puzzle2(INPUT_FILE.to_string()));
    }
}
