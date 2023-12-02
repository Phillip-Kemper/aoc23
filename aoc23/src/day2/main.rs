use std::{env, fs};

use regex::Regex;

#[derive(Clone, Debug)]
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
    let game_config = Game::new(12,13,14);

    let chunks: Vec<&str> = contents.split("\n").collect();

    let mut sum = 0;
    for (game,chunk) in chunks.iter().enumerate() {
        if are_all_games_possible(chunk, game_config.clone()) {
            println!("game {} is possible", game+1);
            sum += game+1;
        } else {
            println!("game {} is not possible", game+1);
        }    
    }
    println!("sum: {}", sum)
}

fn are_all_games_possible(chunk: &str, game_config: Game) -> bool {
    println!("checking chunk: {}", chunk);
    let current_game_vec = process_chunk(chunk);
    // println!("current_game_vec: {:?}", current_game_vec);
    for game in current_game_vec {
        if !(compare_game_with_game_config(game, game_config.clone())) {
            return false;
        } 
    }
    true

}

fn process_chunk(chunk: &str) -> Vec<Game>  {
    let _remove_game_prefix: String= Regex::new(r"^Game [0-9]+:").unwrap().replace(chunk, "").into();
    // println!("remove_game_prefix: {}", _remove_game_prefix);

    let sub_games = _remove_game_prefix.split(";").collect::<Vec<&str>>();
    // println!("sub_games: {:?}", sub_games.len());
    let mut sub_games_vec: Vec<Game> = Vec::new();
    for current_game in sub_games {
        let sub_game : Game = subgame_from_chunk(current_game);
        sub_games_vec.push(sub_game);
    }

    sub_games_vec    
}

fn compare_game_with_game_config(game: Game, game_config: Game) -> bool {
    println!("game: {:?}", game);
    println!("game_config: {:?}", game_config);
    if game.red <= game_config.red && game.green <= game_config.green && game.blue <= game_config.blue {
        println!("true");
        true
    } else {
        println!("false");
        false
    }

}

fn subgame_from_chunk(game_str: &str) -> Game {
    println!("game_str: {}", game_str);
    let mut game = Game::new(0,0,0);
    let dices = game_str.trim().split(",").collect::<Vec<&str>>();

    for dice in dices {
        println!("dice: {}", dice);
        let regex_red = Regex::new(r"[0-9]+ red").unwrap();
        let regex_blue = Regex::new(r"[0-9]+ blue").unwrap();
        let regex_green = Regex::new(r"[0-9]+ green").unwrap();

        let digit: u32 = dice.chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap();

        if regex_red.is_match(dice) {
            println!("red: {}", digit);
            game.red = digit;
        } else if regex_blue.is_match(dice) {
            println!("blue: {}", digit);
            game.blue = digit;
        } else if regex_green.is_match(dice) {
            println!("green: {}", digit);
            game.green = digit;
        }

    }
    game
}

