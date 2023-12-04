use core::num;
use std::{env, fs};
use regex::Regex;

#[derive(Clone, Debug)]
struct CardRow {
    winning_cards: Vec<u32>,
    own_cards: Vec<u32>,
}

fn main() {
    let file_path = "src/day4/day4.txt";

    println!("{:?}", env::current_dir());
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let chunks: Vec<&str> = contents.split("\n").collect();

    let mut sum = 0;


    for (_row, chunk) in chunks.clone().iter().enumerate() {
        let processed_row = process_row(chunk);
        sum += get_score(processed_row);
    }

    println!("sum: {}", sum);

}


fn remove_game_prefix(chunk: &str) -> String {
    let remove_card_prefix: String = Regex::new(r"^Card\s*[0-9]+:").unwrap().replace(chunk, "").into();
    remove_card_prefix
}

fn process_row(chunk: &str) -> CardRow {
    let mut processed_row = CardRow {
        winning_cards: vec![],
        own_cards: vec![],
    };
    let remove_game_prefix = remove_game_prefix(chunk);
    let split_row: Vec<&str> = remove_game_prefix.split("|").collect();
    for (index,num_set) in split_row.into_iter().enumerate() {
        // println!("num: {}", num_set);
        let split_nums: Vec<&str> = num_set.split(" ").collect();
        for num in split_nums {
            if num.ne("") {
                let parsed_num: u32 = num.parse().unwrap();
                if index == 0 {
                    processed_row.winning_cards.push(parsed_num);
                } else {
                    processed_row.own_cards.push(parsed_num);
                }
            }
        }
    }
    println!("processed_row: {:?}", processed_row);
    processed_row
}

fn get_score(card_row: CardRow)-> u32 {
    let mut power = 0;

    for winning_card in card_row.winning_cards {
        for own_card in card_row.own_cards.clone() {
            if own_card == winning_card {
                power += 1;
            }
        }
    }

    if power >= 1 {
        u32::pow(2, power-1)
    } else {
        0
    }
}