use colored::Colorize;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "i", long = "input")]
    input_file_path: std::path::PathBuf,
}

fn parse_input_part1(lines: &Vec<Result<String, std::io::Error>>) -> HashMap<String, i32> {
    let mut data = HashMap::from([
        ("horizontal".to_string(), 0 as i32),
        ("vertical".to_string(), 0 as i32),
    ]);
    for line in lines.into_iter() {
        match line {
            Ok(content) => {
                let mut ele = content.split_whitespace();
                match ele.next() {
                    Some(s) => {
                        let distance = ele.next().unwrap().parse::<i32>().unwrap();
                        if s == "forward" {
                            data.entry("horizontal".to_string())
                                .and_modify(|val| *val += distance);
                        } else if s == "up" {
                            data.entry("vertical".to_string())
                                .and_modify(|val| *val -= distance);
                        } else if s == "down" {
                            data.entry("vertical".to_string())
                                .and_modify(|val| *val += distance);
                        } else {
                            println!("Data invalid, which is {}", s);
                        }
                    }
                    None => continue,
                }
            }
            Err(e) => {
                println!("Problem reading file line with error {:?}", e);
                break;
            }
        }
    }
    return data;
}
fn parse_input_part2(lines: &Vec<Result<String, std::io::Error>>) -> HashMap<String, i32> {
    let mut data = HashMap::from([
        ("horizontal".to_string(), 0 as i32),
        ("vertical".to_string(), 0 as i32),
    ]);
    let mut aim: i32 = 0;
    for line in lines.into_iter() {
        match line {
            Ok(content) => {
                let mut ele = content.split_whitespace();
                match ele.next() {
                    Some(s) => {
                        let distance = ele.next().unwrap().parse::<i32>().unwrap();
                        if s == "forward" {
                            data.entry("horizontal".to_string())
                                .and_modify(|val| *val += distance);
                            data.entry("vertical".to_string())
                                .and_modify(|val| *val += distance * aim);
                        } else if s == "up" {
                            aim -= distance;
                        } else if s == "down" {
                            aim += distance;
                        } else {
                            println!("Data invalid, which is {}", s);
                        }
                    }
                    None => continue,
                }
            }
            Err(e) => {
                println!("Problem reading file line with error {:?}", e);
                break;
            }
        }
    }
    return data;
}

fn calculate_result(movement: HashMap<String, i32>) -> i32 {
    return movement.get(&"horizontal".to_string()).unwrap()
        * movement.get(&"vertical".to_string()).unwrap();
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::from_args();
    let input_path = args.input_file_path;
    let mut full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    full_path.push(input_path);

    match full_path.canonicalize() {
        Ok(canonicalized_path) => full_path = canonicalized_path,
        Err(e) => panic!("Problem canonicalizing path with error {:?}.", e),
    }
    println!("input file path is {}", full_path.display());
    let file = File::open(full_path).expect("Something went wrong when reading file.");
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().collect();

    let part1_result = calculate_result(parse_input_part1(&lines));
    let part2_result = calculate_result(parse_input_part2(&lines));

    println!(
        "Part1: Multiply of horizontal and vertical movements are: {}",
        part1_result.to_string().red().bold()
    );
    println!(
        "Part2: Multiply of horizontal and vertical movements are: {}",
        part2_result.to_string().blue().bold()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    use crate::{calculate_result, parse_input_part1, parse_input_part2};

    #[test]
    fn part1_with_test_input() {
        let path = PathBuf::from("../resources/test_input.txt");
        let file = File::open(path).expect("Some thing went wrong while reading input file");
        let reader = BufReader::new(file);
        let lines: Vec<_> = reader.lines().collect();

        assert_eq!(150, calculate_result(parse_input_part1(&lines)));
    }

    #[test]
    fn part2_with_test_input() {
        let path = PathBuf::from("../resources/test_input.txt");
        let file = File::open(path).expect("Some thing went wrong while reading input file");
        let reader = BufReader::new(file);
        let lines: Vec<_> = reader.lines().collect();
        assert_eq!(900, calculate_result(parse_input_part2(&lines)));
    }
}
