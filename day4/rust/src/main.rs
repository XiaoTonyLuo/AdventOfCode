use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use colored::Colorize;
use itertools::Itertools;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "i", long = "input_file_path")]
    input_file_path: PathBuf,
}

#[derive(Clone)]
struct Data {
    steps: Vec<i32>,
    boards: Vec<[[i32; 5]; 5]>,
}

fn read_data(file_path: PathBuf) -> Data {
    let mut data = Data {
        steps: Vec::new(),
        boards: Vec::new(),
    };
    let file = File::open(file_path).expect("Problem with open file.");
    let mut reader = BufReader::new(file);

    let mut input_line: String = String::new();
    match reader.read_line(&mut input_line) {
        Ok(_) => {
            data.steps = input_line
                .split(',')
                .map(|s| s.trim())
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
        }
        Err(e) => panic!("Problem reading file with error:{}", e),
    }

    let line_iter = reader.lines().map(|l| l.unwrap());
    let mut counter = 0;
    let board_size = 5;
    for board_line in line_iter {
        if board_line.is_empty() {
            continue;
        }
        if counter >= board_size {
            counter = 0;
        }
        if counter == 0 {
            data.boards.push([[0; 5]; 5]);
        }
        let parsed: Vec<i32> = board_line
            .split_whitespace()
            .map(|s| s.trim())
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let line_array = data.boards.last_mut().unwrap()[counter].as_mut();
        line_array.iter_mut().set_from(parsed.iter().cloned());
        counter += 1;
    }
    data
}

fn process_each_move_part1(data: &Data) -> i32 {
    let mut local_data = data.clone();
    let mut score_board: Vec<[[i32; 5]; 2]> = vec![[[0; 5]; 2]; local_data.boards.len()];

    for step in local_data.steps {
        for board_index in 0..local_data.boards.len() {
            let board = local_data.boards.as_slice()[board_index];
            // search the index of this step on this board
            let loc: Vec<(usize, usize)> = board
                .iter()
                .enumerate()
                .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, &val)| (x, y, val)))
                .filter(|&(_, _, val)| val == step)
                .map(|(x, y, _)| (x, y))
                .collect();

            if loc.len() == 1 {
                // update row and column counter
                local_data.boards.as_mut_slice()[board_index][loc.as_slice()[0].0]
                    [loc.as_slice()[0].1] = -1;

                score_board.as_mut_slice()[board_index][0][loc.as_slice()[0].0] += 1;
                score_board.as_mut_slice()[board_index][1][loc.as_slice()[0].1] += 1;
                if score_board.as_mut_slice()[board_index][0][loc.as_slice()[0].0] == 5
                    || score_board.as_mut_slice()[board_index][1][loc.as_slice()[0].1] == 5
                {
                    local_data.boards.as_mut_slice()[board_index]
                        .iter_mut()
                        .for_each(|line| {
                            line.iter_mut()
                                .for_each(|ele| *ele = if *ele == -1 { 0 } else { *ele })
                        });
                    return local_data.boards.as_slice()[board_index]
                        .iter()
                        .map(|line| line.iter().sum::<i32>())
                        .sum::<i32>()
                        * step;
                }
            }
        }
    }
    0
}

fn process_each_move_part2(data: &Data) -> i32 {
    let mut local_data = data.clone();
    let mut score_board: Vec<[[i32; 5]; 2]> = vec![[[0; 5]; 2]; local_data.boards.len()];
    let mut unfinished_count = local_data.boards.len();
    for step in local_data.steps {
        for board_index in 0..local_data.boards.len() {
            let board = local_data.boards.as_slice()[board_index];
            // search the index of this step on this board
            let loc: Vec<(usize, usize)> = board
                .iter()
                .enumerate()
                .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, &val)| (x, y, val)))
                .filter(|&(_, _, val)| val == step)
                .map(|(x, y, _)| (x, y))
                .collect();

            if loc.len() == 1 {
                // update row and column counter
                local_data.boards.as_mut_slice()[board_index][loc.as_slice()[0].0]
                    [loc.as_slice()[0].1] = -1;

                score_board.as_mut_slice()[board_index][0][loc.as_slice()[0].0] += 1;
                score_board.as_mut_slice()[board_index][1][loc.as_slice()[0].1] += 1;
                if score_board.as_mut_slice()[board_index][0][loc.as_slice()[0].0] == 5
                    || score_board.as_mut_slice()[board_index][1][loc.as_slice()[0].1] == 5
                {
                    if unfinished_count == 1 {
                        local_data.boards.as_mut_slice()[board_index]
                            .iter_mut()
                            .for_each(|line| {
                                line.iter_mut()
                                    .for_each(|ele| *ele = if *ele == -1 { 0 } else { *ele })
                            });
                        return local_data.boards.as_slice()[board_index]
                            .iter()
                            .map(|line| line.iter().sum::<i32>())
                            .sum::<i32>()
                            * step;
                    }

                    local_data.boards.as_mut_slice()[board_index] = [[-1; 5]; 5];
                    unfinished_count -= 1;
                }
            }
        }
    }
    0
}
fn main() {
    let input_path = Cli::from_args().input_file_path;
    let mut full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    full_path.push(input_path);

    match full_path.canonicalize() {
        Ok(canonicalized) => full_path = canonicalized,
        Err(e) => panic!("Problem with canonical full path with error: {}.", e),
    }

    let data = read_data(full_path);
    let result_part1 = process_each_move_part1(&data);
    println!(
        "The result of part 1 is: {}",
        result_part1.to_string().red().bold()
    );

    let result_part2 = process_each_move_part2(&data);
    println!(
        "The result of part 2 is: {}",
        result_part2.to_string().blue().bold()
    );
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{process_each_move_part1, read_data};

    #[test]
    fn part1_test() {
        let mut full_path = PathBuf::from("../resources/test_input.txt");

        match full_path.canonicalize() {
            Ok(canonicalized) => full_path = canonicalized,
            Err(e) => panic!("Problem with canonical full path with error: {}.", e),
        }

        let data = read_data(full_path);
        assert_eq!(4512, process_each_move_part1(&data));
    }
}
