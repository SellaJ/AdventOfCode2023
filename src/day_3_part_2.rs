use std::fs::*;
use std::io::{BufRead, BufReader, Result};

const DIRECTION_VECTOR: [Position; 8] = [
    Position { x: 0, y: 1 },
    Position { x: 0, y: -1 },
    Position { x: 1, y: 0 },
    Position { x: -1, y: 0 },
    Position { x: 1, y: -1 },
    Position { x: 1, y: 1 },
    Position { x: -1, y: 1 },
    Position { x: -1, y: -1 },
];

fn main() {
    let lines = read_file_line_by_line(
        "C:/Programming projects/Rust/Advent_of_code/AdventOfCode2023/src/input.txt",
    )
    .unwrap();
    let power = find_power(lines.iter().map(|line| line.as_bytes()).collect());
    println!("{power}")
}

#[derive(PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn add(&self, pos: Position) -> Position {
        return Position {
            x: self.x + pos.x,
            y: self.y + pos.y,
        };
    }
}

fn find_power(matrix: Vec<&[u8]>) -> i32 {
    let mut power = 0;

    let mut numbers: Vec<(Vec<Position>, i32)> = Vec::new();
    let mut power_symbols: Vec<Position> = Vec::new();

    for (position, line) in matrix.iter().enumerate() {
        let mut line_index = 0;
        while line_index < line.len() {
            match line[line_index] {
                b'*' => {
                    power_symbols.push(Position {
                        x: line_index as i32,
                        y: position as i32,
                    });
                }
                b'1'..=b'9' => {
                    let start_number_index = line_index;
                    let mut end_number_index = line_index;
                    line_index += 1;
                    while line_index < line.len() && line[line_index].is_ascii_digit() {
                        end_number_index = line_index;
                        line_index += 1;
                    }
                    line_index -= 1;
                    let number =
                        (std::str::from_utf8(&line[start_number_index..=end_number_index]))
                            .unwrap()
                            .parse::<i32>()
                            .unwrap();

                    let points = vec![
                        Position {
                            x: start_number_index as i32,
                            y: position as i32,
                        },
                        Position {
                            x: end_number_index as i32,
                            y: position as i32,
                        },
                    ];

                    numbers.push((points, number));
                }
                _ => {}
            }

            line_index += 1;
        }
    }

    for power_symbol_position in power_symbols {
        let mut current_power = 1;
        let mut numbers_found = 0;
        
        let mut seen_positions: Vec<&Position> = Vec::new();
        for direction in DIRECTION_VECTOR {
            let current_pos = power_symbol_position.add(direction);
            for num in &numbers {
                if num.0.contains(&current_pos) && !seen_positions.contains(&&current_pos) {
                    current_power *= num.1;
                    numbers_found = numbers_found + 1;
                    seen_positions.push(&num.0[0]);
                    seen_positions.push(&num.0[1]);
                    break;
                }
            }    
        }

        if numbers_found == 2 {
            power += current_power;
        }
    }

    return power;
}

fn read_file_line_by_line(path: &str) -> Result<Vec<String>> {
    let mut file_lines: Vec<String> = Vec::new();
    let file = File::open(path)?;
    let buf_read = BufReader::new(file);
    for line in buf_read.lines() {
        file_lines.push(line?);
    }
    return Ok(file_lines);
}
