use std::{env, fs};


struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn new(red: u32, green: u32, blue: u32) -> Game {
        Game {
            red,
            green,
            blue,
        }
    }
}
fn main() {
    let file_path = "src/day2/day2.txt";

    println!("{:?}", env::current_dir());
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let chunks: Vec<&str> = contents.split("\n").collect();

    let mut sum = 0;
    for (game,chunk) in chunks.iter().enumerate() {
        if is_game_possible(chunk) {
            println!("game {} is possible", game);
            sum += game;
        }    
    }
    println!("sum: {}", sum)
}

fn is_game_possible(chunk: &str) -> bool {
    println!("checking chunk: {}", chunk);
    true
}

fn process_chunk(chunk: &str) ->  {
    println!("processing chunk: {}", chunk);
    true
}

