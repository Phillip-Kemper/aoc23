use core::num;
use std::{env, fs, cmp};
use regex::Regex;


#[derive(Clone, Debug)]
struct NumberMatch {
    number: u32,
    row: usize,
    start: usize,
    end: usize,
}

fn main() {
    let file_path = "src/day3/day3.txt";

    println!("{:?}", env::current_dir());
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let chunks: Vec<&str> = contents.split("\n").collect();

    let mut sum = 0;


    for (row, chunk) in chunks.clone().iter().enumerate() {
        let row_number_matches = get_number_matches(chunk, row);

        let valid_number_matches = get_valid_number_matches(row, row_number_matches,chunks.clone());

        println!("valid matches {:?}", valid_number_matches);
        for mat in valid_number_matches {
            sum += mat.number;
        }
    }

    println!("sum: {}", sum);
}


fn get_number_matches(chunk: &str, row: usize)-> Vec<NumberMatch>{
    let number_regex = Regex::new(r"[0-9]+").unwrap();
    let mut number_matches: Vec<NumberMatch> = vec![];

    for mat in number_regex.find_iter(chunk) {
        let num: u32 = mat.as_str().parse().unwrap();

        let number_match = NumberMatch {
            number: num,
            row,
            start: mat.start(),
            end: mat.end(),
        };
        number_matches.push(number_match);
    }
    number_matches
}

fn get_valid_number_matches(row: usize, number_matches: Vec<NumberMatch>, chunks: Vec<&str>) -> Vec<NumberMatch> {
    let mut valid_number_matches: Vec<NumberMatch> = vec![];

    for number_match in number_matches {
        if is_match_valid(row, number_match.start, number_match.end, chunks.clone()) {
            valid_number_matches.push(number_match);
        }
    }
    valid_number_matches
}

fn is_match_valid(row: usize, start_index: usize, end_index: usize, chunks: Vec<&str>) -> bool {
    if row != 0 {
        let previous_row = chunks[row-1].get(subtract_one_if_not_zero(start_index)..cmp::min(end_index+1, 140)).unwrap();
        // println!("previous_row: {}", previous_row);
        for c in previous_row.chars() {
            if !(c as u8 == 46)
            {
                return true;
            }
        }
    }

    if start_index != 0 {
        if !(chunks[row].chars().nth(start_index-1).unwrap() as u8 == 46) {
            return true;
        }
    }
    // println!("end_index: {}", end_index);
    // println!("test: {}", chunks[row].chars().nth(end_index).unwrap());
    println!("end_index: {}", end_index);
    println!("chunks: {}", chunks[row]);
    if end_index != 140 {
        if !(chunks[row].chars().nth(end_index).unwrap() as u8 == 46) {
            return true;
        }
    }

    if row != chunks.len()-1 {
        let next_row = chunks[row+1].get(subtract_one_if_not_zero(start_index)..cmp::min(end_index+1, 140)).unwrap();
        // println!("next_row: {}", next_row);
        for c in next_row.chars() {
            if !(c as u8 == 46)
            {
                return true;
            }
        }
    }
    false
}

fn subtract_one_if_not_zero(x: usize) -> usize {
    if x > 0 {
        x - 1
    } else {
        x
    }
}
