use colored::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use structopt::{StructOpt};

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "i", long = "input")]
    input_file_path: std::path::PathBuf,
}

fn calculate_part_1(lines: &Vec<Result<String, std::io::Error>>) -> i32 {
    let mut data: Vec<u32> = Vec::new();
    let mut counter = 0;
    for line in lines.into_iter() {
        data.push(line.as_ref().unwrap().parse::<u32>().unwrap());
        let len = data.len();
        if len > 1 {
            if data[len - 1] > data[len - 2] {
                counter += 1;
            }
        }
    }

    return counter;
}

fn calculate_part_2(lines: &Vec<Result<String, std::io::Error>>) -> i32 {
    let mut data: Vec<u32> = Vec::new();
    let mut counter = 0;
    for line in lines.into_iter() {
        data.push(line.as_ref().unwrap().parse::<u32>().unwrap());
        let len = data.len();
        if len > 3 {
            let current_window = data[len-1]+data[len-2]+data[len-3];
            let prev_window = data[len-2]+data[len-3]+data[len-4];
            if current_window > prev_window {
                counter += 1;
            }
        }
    }

    return counter;
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::from_args();
    let input_path = args.input_file_path;
    let mut env_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    env_path.push(input_path.clone());
    println!(
        "input file path is: {}",
        env_path.canonicalize().unwrap().display()
    );
    let file = File::open(env_path).expect("Some thing went wrong while reading input file");
    let reader = BufReader::new(file.try_clone().unwrap());
    let lines: Vec<_> = reader.lines().collect();

    let counter_part1 = calculate_part_1(&lines);
    let counter_part2 = calculate_part_2(&lines);
    println!(
        "Part1: The measurement incresed {} times!",
        counter_part1.to_string().red().bold()
    );

    println!(
        "Part2: The measurement incresed {} times!",
        counter_part2.to_string().blue().bold()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use std::path::PathBuf;

    use crate::{calculate_part_1, calculate_part_2};

    #[test]
    fn part1_with_test_input() {
        let path = PathBuf::from("../resources/test_input.txt");
        let file = File::open(path).expect("Some thing went wrong while reading input file");
        let reader = BufReader::new(file);
        let lines: Vec<_> = reader.lines().collect();

        assert_eq!(calculate_part_1(&lines), 7);
    }
    
    #[test]
    fn part2_with_test_input() {
        let path = PathBuf::from("../resources/test_input.txt");
        let file = File::open(path).expect("Some thing went wrong while reading input file");
        let reader = BufReader::new(file);
        let lines: Vec<_> = reader.lines().collect();

        assert_eq!(calculate_part_2(&lines), 5);
    }
}
