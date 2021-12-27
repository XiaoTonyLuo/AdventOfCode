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

fn parse_input(reader: BufReader<File>) -> HashMap<String, i32> {
    let mut data = HashMap::from([
        ("horizontal".to_string(), 0 as i32),
        ("vertical".to_string(), 0 as i32),
    ]);
    for line in reader.lines() {
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

    let result = calculate_result(parse_input(reader));
    println!(
        "Multiply of horizontal and vertical movements are: {}",
        result.to_string().red().bold()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, fs::File, io::BufReader};

    use crate::{calculate_result, parse_input};

    #[test]
    fn with_test_input() {
        let path = PathBuf::from("../resources/test_input.txt");
        let file = File::open(path).expect("Some thing went wrong while reading input file");
        let reader = BufReader::new(file);

        assert_eq!(150, calculate_result(parse_input(reader)));
    }
}
