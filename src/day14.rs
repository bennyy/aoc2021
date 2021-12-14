use crate::file_util;
use std::collections::HashMap;

static INPUT_FILE: &str = "inputs/day14.txt";

fn puzzle(file_path: String, steps: u32) -> u64 {
    let input_str = file_util::file_to_string(file_path);
    let input_str: Vec<&str> = input_str.split("\n\n").collect();

    let polymer_template: Vec<_> = input_str.first().unwrap().chars().collect();

    let pair_insertion: HashMap<_, _> = input_str
        .clone()
        .last()
        .unwrap()
        .split('\n')
        .map(|x| {
            let mut split = x.split(" -> ");
            let tuple = (
                split.next().unwrap().to_string(),
                split.next().unwrap().chars().next().unwrap(),
            );
            tuple
        })
        .into_iter()
        .collect();

    let mut pairs: HashMap<_, _> = polymer_template
        .iter()
        .zip(polymer_template.iter().skip(1))
        .map(|x| {
            let mut polymer = String::from("");
            polymer.push(*x.0);
            polymer.push(*x.1);

            (polymer, 1)
        })
        .into_iter()
        .collect();

    for _ in 0..steps {
        let clone_pairs = pairs.clone();
        pairs.clear();
        for (k, n) in clone_pairs.iter() {
            let element = pair_insertion.get(&k.to_string()).unwrap();
            let chars: Vec<_> = k.chars().collect();

            let mut first_element = String::from("");
            first_element.push(*chars.first().unwrap());
            first_element.push(*element);

            let mut second_element = String::from("");
            second_element.push(*element);
            second_element.push(*chars.last().unwrap());
            pairs.entry(first_element.clone()).or_insert(0);
            pairs.entry(second_element.clone()).or_insert(0);

            *pairs.get_mut(&first_element).unwrap() += n;
            *pairs.get_mut(&second_element).unwrap() += n;
        }
    }
    // Count all first characters! (the second character one is another first caracter)
    let mut characters: HashMap<char, u64> = HashMap::new();
    for (k, n) in pairs.iter() {
        let chars: Vec<_> = k.chars().collect();
        let char1 = chars.first().unwrap();
        characters.entry(*char1).or_insert(0);
        *characters.get_mut(char1).unwrap() += n;
    }
    // Except the last character..
    *characters
        .get_mut(polymer_template.last().unwrap())
        .unwrap() += 1;

    let max = characters
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap();

    let min = characters
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap();

    let res = characters.get(max).unwrap() - characters.get(min).unwrap();
    res as u64
}

pub fn run() {
    let result = puzzle(INPUT_FILE.to_string(), 10);
    println!("D14P1: {}", result);
    let result = puzzle(INPUT_FILE.to_string(), 40);
    println!("D14P2: {}", result);
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_FILE: &str = "inputs/day14_test.txt";
    use super::*;

    #[test]
    fn puzzle_1() {
        assert_eq!(1588, puzzle(TEST_INPUT_FILE.to_string(), 10));
        assert_eq!(2590, puzzle(INPUT_FILE.to_string(), 10));
    }

    #[test]
    fn puzzle_2() {
        assert_eq!(2188189693529, puzzle(TEST_INPUT_FILE.to_string(), 40));
        assert_eq!(2875665202438, puzzle(INPUT_FILE.to_string(), 40));
    }
}
