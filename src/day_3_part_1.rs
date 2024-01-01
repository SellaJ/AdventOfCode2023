use std::fs::*;
use std::io::{BufRead, BufReader, Result};

const start_direction_vector: [(i32, i32); 5] = [(0, 1), (0, -1), (-1, 1), (-1, -1), (-1, 0)];
const end_direction_vector: [(i32, i32); 5] = [(0, 1), (0, -1), (1, 1), (1, -1), (1, 0)];
const middle_direction_vector: [(i32, i32); 2] = [(0, 1), (0, -1)];

fn main() {
    let lines = read_file_line_by_line(
        "C:/Programming projects/Rust/Advent_of_code/AdventOfCode2023/src/input.txt",
    )
    .unwrap();
    let sum = sum_valid_numbers(lines.iter().map(|line| line.as_bytes()).collect());
    println!("{sum}")
}

fn sum_valid_numbers(matrix: Vec<&[u8]>) -> i32 {
    let mut sum = 0;

    for (position, line) in matrix.iter().enumerate() {
        let mut number_start_index = 0;
        let mut number_end_index = 0;

        let mut number = String::new();
        let mut line_index = 0;

        while line_index < line.len() {
            let is_digit = line[line_index].is_ascii_digit();
            let mut near_symbol = false;
            if is_digit {
                number_start_index = line_index;
                number_end_index = line_index;

                line_index += 1;
                while line_index < line.len() && line[line_index].is_ascii_digit() {
                    number_end_index = line_index;
                    line_index += 1;
                }
                if number_start_index != number_end_index {
                    for digit_index in number_start_index + 1..number_end_index {
                        near_symbol = check_digit(
                            digit_index,
                            position,
                            line,
                            &matrix,
                            &middle_direction_vector,
                        );

                        if near_symbol {
                            break;
                        }
                    }
                }

                if !near_symbol {
                    near_symbol = check_digit(
                        number_start_index,
                        position,
                        line,
                        &matrix,
                        &start_direction_vector,
                    );

                    if !near_symbol {
                        near_symbol = check_digit(
                            number_end_index,
                            position,
                            line,
                            &matrix,
                            &end_direction_vector,
                        );
                    }
                }
            }
            if near_symbol {
                number +=
                    std::str::from_utf8(&line[number_start_index..number_end_index + 1]).unwrap();
                sum += number.parse::<i32>().unwrap();
            }
            number = String::new();

            line_index += 1;
        }
    }

    return sum;
}

fn check_digit(
    digit_index: usize,
    position: usize,
    line: &&[u8],
    matrix: &Vec<&[u8]>,
    direction_vector: &[(i32, i32)],
) -> bool {
    for direction in direction_vector {
        let point = (
            digit_index as i32 + direction.0,
            position as i32 + direction.1,
        );
        if point.0 >= 0
            && (point.0 as usize) < line.len()
            && point.1 >= 0
            && (point.1 as usize) < matrix.len()
        {
            let value = matrix[point.1 as usize][point.0 as usize];
            if value != b'.' && !value.is_ascii_digit() {
                return true;
            }
        }
    }

    return false;
}

fn read_file_line_by_line(path: &str) -> Result<Vec<String>> {
    let mut file_lines: Vec<String> = Vec::new();
    let file = File::open(path)?;
    let mut bufRead = BufReader::new(file);
    for line in bufRead.lines() {
        file_lines.push(line?);
    }
    return Ok(file_lines);
}
