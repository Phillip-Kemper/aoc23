use std::{env, fs};
use regex::Regex;


#[derive(Clone, Debug)]
struct NumberMatch {
    number: u32,
    row: usize,
    start: usize,
    end: usize,
}

fn main() {
    let file_path = "src/day4/day4.txt";

    println!("{:?}", env::current_dir());
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let chunks: Vec<&str> = contents.split("\n").collect();

    let mut sum = 0;


    for (row, chunk) in chunks.clone().iter().enumerate() {
    }
}