use colored::*;
use std::{
    fs::File,
    io::{BufRead, BufReader}, path::PathBuf,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "i", long = "input")]
    input_file_path: std::path::PathBuf,
}

fn calculate(reader: BufReader<File>) -> i32 {
    let mut data: Vec<u32> = Vec::new();
    let mut counter = 0;
    for line in reader.lines() {
        data.push(line.unwrap().parse::<u32>().unwrap());
        let len = data.len();
        if len > 1 {
            if data[len - 1] > data[len - 2] {
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
    println!("input file path is: {}", env_path.display());
    let file = File::open(env_path).expect("Some thing went wrong while reading input file");
    let reader = BufReader::new(file);

    let counter = calculate(reader);

    println!(
        "The measurement incresed {} times!",
        counter.to_string().red().bold()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use std::fs::File;
    use std::path::PathBuf;

    use crate::calculate;

    #[test]
    fn with_test_input() {
        let path = PathBuf::from("../resources/test_input.txt");
        let file = File::open(path).expect("Some thing went wrong while reading input file");
        let reader = BufReader::new(file);

        assert_eq!(calculate(reader), 7);
    }
}
