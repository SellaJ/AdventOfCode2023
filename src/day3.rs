use std::fs::*;
use std::io::{Result, BufReader, BufRead};


fn main() {
    let lines = read_file_line_by_line("input.txt").unwrap();
    for line in lines {
        println!("{line}");
    }
}

fn sum_valid_numbers(matrix: Vec<String>) -> i32 {
    let mut sum = 0;
    
     for (position, line) in matrix.iter().enumerate() {
        for line_index in 0..line.len() {
            
        } 
     }
    
    return sum;
}

fn read_file_line_by_line(path: &str) -> Result<Vec<String>>{ 
    let mut file_lines: Vec<String> = Vec::new();
    let file = File::open(path)?;
    let mut bufRead = BufReader::new(file);
    for line in bufRead.lines() {
        file_lines.push(line?);
    }
    return Ok(file_lines);
}
