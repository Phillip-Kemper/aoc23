use std::{env, fs};

fn main() {
    let file_path = "day1/day1.txt";

    println!("{:?}", env::current_dir());
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let chunks: Vec<&str> = contents.split("\n\n").collect();
}