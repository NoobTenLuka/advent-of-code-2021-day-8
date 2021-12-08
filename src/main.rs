use itertools::Itertools;
use std::{collections::HashMap, fs, path::Path};

const ALL_CHARS: &str = "abcdefg";

fn main() {
    let inputs = read_inputs("input.txt");

    solution_1(&inputs);
    solution_2(&inputs);
}

fn solution_1(all_inputs_and_outputs: &[(String, String)]) {
    let num = all_inputs_and_outputs.iter().fold(0, |upper_acc, o| {
        upper_acc
            + o.1.split(' ').fold(0, |acc, e| match e.len() {
                2 | 4 | 7 | 3 => acc + 1,
                _ => acc,
            })
    });

    println!("The number of unique digits is {}", &num);
}

fn solution_2(all_inputs_and_outputs: &[(String, String)]) {
    let sum_of_all: usize = all_inputs_and_outputs
        .iter()
        .map(|(inputs, outputs)| {
            let mut wire_map: HashMap<char, char> = HashMap::new();
            let split_inputs: Vec<&str> = inputs.split(' ').collect();

            let number_one = split_inputs
                .iter()
                .find(|s| s.len() == 2)
                .expect("No number one");
            let number_seven = split_inputs
                .iter()
                .find(|s| s.len() == 3)
                .expect("No number seven");
            let number_four = split_inputs
                .iter()
                .find(|s| s.len() == 4)
                .expect("No number four");

            let upper = number_seven
                .chars()
                .find(|c| !number_one.contains(*c))
                .unwrap();

            wire_map.insert(upper, 'a');

            let number_nine = split_inputs
                .iter()
                .find(|s| s.len() == 6 && number_four.chars().all(|c| s.contains(c)))
                .unwrap();

            let lower_left = ALL_CHARS
                .chars()
                .find(|c| !number_nine.contains(*c))
                .unwrap();

            wire_map.insert(lower_left, 'e');

            let lower = number_nine
                .chars()
                .find(|c| !number_seven.contains(*c) && !number_four.contains(*c))
                .unwrap();

            wire_map.insert(lower, 'g');

            let num_four_unique_chars: Vec<char> = number_four
                .chars()
                .filter(|c| !number_one.contains(*c))
                .collect();

            let all_but_left: Vec<char> = number_nine
                .chars()
                .filter(|c| !num_four_unique_chars.contains(c))
                .collect();

            let number_zero = split_inputs
                .iter()
                .find(|s| {
                    s.len() == 6 && all_but_left.iter().all(|c| s.contains(*c)) && *s != number_nine
                })
                .unwrap();

            let middle = ALL_CHARS
                .chars()
                .find(|c| !number_zero.contains(*c))
                .unwrap();

            wire_map.insert(middle, 'd');

            let upper_left = num_four_unique_chars
                .iter()
                .find(|c| **c != middle)
                .unwrap();

            wire_map.insert(*upper_left, 'b');

            let number_two = split_inputs
                .iter()
                .find(|s| {
                    [upper, middle, lower, lower_left]
                        .into_iter()
                        .all(|c| s.contains(c))
                        && s.len() == 5
                })
                .unwrap();

            let upper_right = number_two
                .chars()
                .find(|c| !(*c == upper || *c == middle || *c == lower || *c == lower_left))
                .unwrap();

            wire_map.insert(upper_right, 'c');

            let lower_right = number_one.chars().find(|c| *c != upper_right).unwrap();

            wire_map.insert(lower_right, 'f');

            let split_outputs: Vec<&str> = outputs.split(' ').collect();

            let number_str: String = split_outputs
                .iter()
                .map(|o| {
                    let correct_wires: String = o
                        .chars()
                        .map(|c| wire_map.get(&c).unwrap())
                        .sorted()
                        .collect();

                    match correct_wires.as_str() {
                        "abcefg" => '0',
                        "cf" => '1',
                        "acdeg" => '2',
                        "acdfg" => '3',
                        "bcdf" => '4',
                        "abdfg" => '5',
                        "abdefg" => '6',
                        "acf" => '7',
                        "abcdefg" => '8',
                        "abcdfg" => '9',
                        _ => '0',
                    }
                })
                .collect();

            number_str.parse::<usize>().unwrap()
        })
        .sum();

    println!("The sum of all numbers is {}", &sum_of_all);
}

fn read_inputs<T: AsRef<Path>>(path: T) -> Vec<(String, String)> {
    let file_content = fs::read_to_string(path).expect("Input file not found");

    file_content
        .split('\n')
        .map(|e| {
            let input = e.split_at(59);

            let output = input.1.replace('|', "");

            (input.0.trim().to_string(), output.trim().to_string())
        })
        .collect()
}
