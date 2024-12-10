use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use colored::Colorize;
use structopt::*;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "i", long = "input_file_path")]
    input_file_path: std::path::PathBuf,
}

fn calculate_result_part1(data: &Vec<String>) -> i32 {
    let mut counter: Vec<i32>;
    {
        let ele_len = data.into_iter().next().unwrap().len();
        counter = vec![0; ele_len];
    }
    for line in data.into_iter() {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                counter[i] += 1;
            }
        }
    }

    let data_num = data.len();
    let mut gamma_result = String::new();
    let mut epsilon_result = String::new();

    for count in counter.into_iter() {
        if count > data_num as i32 / 2 {
            gamma_result.push('1');
            epsilon_result.push('0');
        } else {
            gamma_result.push('0');
            epsilon_result.push('1');
        }
    }
    let gamma = isize::from_str_radix(&gamma_result, 2).unwrap() as u32;
    let epsilon = isize::from_str_radix(&epsilon_result, 2).unwrap() as u32;
    (gamma * epsilon) as i32
}

fn calculate_result_part2(data: &Vec<String>) -> i32 {
    let calculate_1_count = |data_vec: &mut Vec<String>, i: usize| {
        let mut count: usize = 0;
        for line in data_vec.into_iter() {
            if line.as_bytes()[i] as char == '1' {
                count += 1;
            }
        }
        count
    };

    let data_len = data.first().unwrap().len();
    let mut gamma_filt = data.clone();
    let mut epsilon_filt = data.clone();

    for i in 0..data_len {
        let gamma_count = calculate_1_count(&mut gamma_filt, i);
        let epsilon_count = calculate_1_count(&mut epsilon_filt, i);

        if (gamma_count * 2 >= gamma_filt.len()) && (gamma_filt.len() > 1) {
            gamma_filt.retain(|ele| ele.as_bytes()[i] as char == '1');
        } else if (gamma_count * 2 < gamma_filt.len()) && (gamma_filt.len() > 1) {
            gamma_filt.retain(|ele| ele.as_bytes()[i] as char == '0');
        }

        if (epsilon_count * 2 >= epsilon_filt.len()) && (epsilon_filt.len() > 1) {
            epsilon_filt.retain(|ele| ele.as_bytes()[i] as char == '0');
        } else if (epsilon_count * 2 < epsilon_filt.len()) && (epsilon_filt.len() > 1) {
            epsilon_filt.retain(|ele| ele.as_bytes()[i] as char == '1');
        }
    }

    let gamma = isize::from_str_radix(&gamma_filt[0], 2).unwrap() as u32;
    let epsilon = isize::from_str_radix(&epsilon_filt[0], 2).unwrap() as u32;
    (gamma * epsilon) as i32
}

fn main() {
    let start = std::time::Instant::now();
    let arg = Cli::from_args();
    let input_file_path = arg.input_file_path;
    let mut full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    full_path.push(input_file_path);

    match full_path.canonicalize() {
        Ok(canonicalized_path) => full_path = canonicalized_path,
        Err(e) => panic!("Problem with canonicalize full path, error is {:?}", e),
    }

    let file = File::open(full_path).expect("Problem with open file.");
    let reader = BufReader::new(file);
    let data: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

    let result_part1 = calculate_result_part1(&data);
    let result_part2 = calculate_result_part2(&data);

    println!("The part 1 result is {}", result_part1.to_string().red());
    println!("The part 1 result is {}", result_part2.to_string().blue());
    let elapsed = start.elapsed();
    println!("Time consumption: {:?}", elapsed);
}

#[cfg(test)]
mod test {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    use crate::{calculate_result_part1, calculate_result_part2};

    #[test]
    fn test_part1_with_test_input() {
        let full_path = PathBuf::from("../resources/test_input.txt");
        let file = File::open(full_path).expect("Problem with open file.");
        let reader = BufReader::new(file);
        let data: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();
        assert_eq!(198, calculate_result_part1(&data));
    }

    #[test]
    fn test_part2_with_test_input() {
        let full_path = PathBuf::from("../resources/test_input.txt");
        let file = File::open(full_path).expect("Problem with open file.");
        let reader = BufReader::new(file);
        let data: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();
        assert_eq!(230, calculate_result_part2(&data));
    }
}
