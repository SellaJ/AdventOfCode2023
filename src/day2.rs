use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Game {
    blue_cubes: i32,
    red_cubes: i32,
    green_cubes: i32,
}

#[derive(Debug)]
struct GameBatch {
    batch_number: i32,
    batch_games: Vec<Game>,
}

impl GameBatch {
    fn new() -> Self {
        GameBatch {
            batch_number: 0,
            batch_games: Vec::new(),
        }
    }
}

fn main() {
    let mut power = 0;
    if let Ok(lines) = read_lines("C:/Programming projects/Rust/advent-of-code-day-2/target/debug/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(raw_game_batch) = line {
                let game = parse_games(&raw_game_batch);
                power +=  find_power_of_min_required_cubes(&game);    
            }
        }
    }

    println!("{}", power);
}

fn parse_games(raw_game: &str) -> GameBatch {
    let mut result: GameBatch = GameBatch::new();

    let raw_game_data: Vec<&str> = raw_game.split(":").collect();

    let game_number: Vec<&str> = raw_game_data[0].split(' ').collect();

    result.batch_number = game_number[1].parse::<i32>().unwrap();

    let raw_games = raw_game_data[1].split(';');

    for raw_game in raw_games {
        let mut game = Game {
            blue_cubes: 0,
            green_cubes: 0,
            red_cubes: 0,
        };
        let results = raw_game.split(',');
        for game_result in results {
            let raw_game_result: Vec<&str> = game_result.split(' ').collect();
            let number_of_cubes = raw_game_result[1].parse::<i32>().unwrap();
            match raw_game_result[2] {
                "blue" => game.blue_cubes += number_of_cubes,
                "red" => game.red_cubes += number_of_cubes,
                "green" => game.green_cubes += number_of_cubes,
                _ => {}
            }
        }
        result.batch_games.push(game);
    }

    result
}

fn validate_game_batch(
    game_batch: &GameBatch,
    max_red: i32,
    max_blue: i32,
    max_green: i32,
) -> bool {
    for game in game_batch.batch_games.iter() {
        if game.green_cubes > max_green {
            return false;
        }
        if game.red_cubes > max_red {
            return false;
        }
        if game.blue_cubes > max_blue {
            return false;
        }
    }
    return true;
}

fn find_power_of_min_required_cubes(game_batch: &GameBatch) -> i32 {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for game in game_batch.batch_games.iter() {
        if game.blue_cubes > max_blue {
            max_blue = game.blue_cubes;
        }
        if game.red_cubes > max_red {
            max_red = game.red_cubes;
        }
        if game.green_cubes > max_green {
            max_green = game.green_cubes;
        }
    }

    max_red * max_green * max_blue
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
