use std::{path::Path, fs};

fn main() {
    let inputs = read_inputs("input.txt");

    solution_1(&inputs.1);
}

fn solution_1(output_values: &Vec<String>) {
    let num = output_values.iter().fold(0, |upper_acc, o| {
        upper_acc + o.split(' ').fold(0, |acc, e| {
            match e.len() {
                2 | 4 | 7 | 3 => {
                    acc + 1
                },
                _ => acc
            }
        })
    });

    println!("The number of unique digits is {}", &num);
}

fn read_inputs<T: AsRef<Path>>(path: T) -> (Vec<String>, Vec<String>) {
    let file_content = fs::read_to_string(path).expect("Input file not found");

    let mut input_vec: Vec<String> = vec![];
    let mut output_vec: Vec<String> = vec![];

    file_content.split('\n').for_each(|e| {
        let input = e.split_at(59);

        let output = input.1.replace('|', "");

        input_vec.push(input.0.trim().to_string());
        output_vec.push(output.trim().to_string());
    });

    (input_vec, output_vec)
}