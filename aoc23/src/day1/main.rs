use std::{env, fs};

use regex::Regex;

fn main() {
    let file_path = "src/day1/day1.txt";

    println!("{:?}", env::current_dir());
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let chunks: Vec<&str> = contents.split("\n").collect();

    let mut sum = 0;

    for chunk in chunks {
        println!("chunk: {}", chunk);
        let first = get_first_digit(chunk);
        let last = get_last_digit(chunk);
        println!("first: {}", first);
        println!("last: {}", last);

        let concat: u32 = vec![first, last].iter().collect::<String>().parse().unwrap();

        sum += concat;
    }

    println!("sum: {}", sum)
}

fn get_first_digit(calibration_line: &str)-> char {
    let chars = calibration_line.chars();
    for (index,c) in chars.enumerate() {
        if c.is_digit(10) {
            return c;
        }
        match is_digit_text(&(calibration_line.clone()[index..])) {
            Some(digit) => return digit,
            None => continue,
        }
    }
    char::default()
}

fn get_last_digit(calibration_line: &str)-> char {
    let chars = calibration_line.chars().rev();
    for (index,c) in chars.enumerate() {
        if c.is_digit(10) {
            return c;
        }
        match is_digit_text(&(reverse_str::reverse(calibration_line.clone())[index..])) {
            Some(digit) => return digit,
            None => continue,
        }
    }
    char::default()
}

fn is_digit_text(chunk: &str) -> Option<char> {
    println!("checking digit text chunk: {}", chunk);
    let mut re = Regex::new(r"^one*").unwrap();
    let mut re_reverse = Regex::new(r"^eno.*").unwrap();
    if re.captures(chunk).is_some() || re_reverse.captures(chunk).is_some() {
        println!("found one");
        return Some(char::from_digit(1, 10).unwrap());
    } 
    re = Regex::new(r"^two*").unwrap();
    re_reverse = Regex::new(r"^owt.*").unwrap();
    if re.captures(chunk).is_some() || re_reverse.captures(chunk).is_some() {
        return Some(char::from_digit(2, 10).unwrap());
    } 
    re = Regex::new(r"^three*").unwrap();
    re_reverse = Regex::new(r"^eerht.*").unwrap();
    if re.captures(chunk).is_some() || re_reverse.captures(chunk).is_some() {
        return Some(char::from_digit(3, 10).unwrap());
    } 
    re = Regex::new(r"^four*").unwrap();
    re_reverse = Regex::new(r"^ruof.*").unwrap();
    if re.captures(chunk).is_some() || re_reverse.captures(chunk).is_some() {
        return Some(char::from_digit(4, 10).unwrap());
    } 
    re = Regex::new(r"^five*").unwrap();
    re_reverse = Regex::new(r"^evif.*").unwrap();
    if re.captures(chunk).is_some() || re_reverse.captures(chunk).is_some() {
        return Some(char::from_digit(5, 10).unwrap());
    } 
    re = Regex::new(r"^six*").unwrap();
    re_reverse = Regex::new(r"^xis.*").unwrap();
    if re.captures(chunk).is_some() || re_reverse.captures(chunk).is_some(){
        return Some(char::from_digit(6, 10).unwrap());
    } 
    re = Regex::new(r"^seven*").unwrap();
    re_reverse = Regex::new(r"^neves.*").unwrap();
    if re.captures(chunk).is_some() || re_reverse.captures(chunk).is_some(){
        return Some(char::from_digit(7, 10).unwrap());
    } 
    re = Regex::new(r"^eight*").unwrap();
    re_reverse = Regex::new(r"^thgie.*").unwrap();
    if re.captures(chunk).is_some() || re_reverse.captures(chunk).is_some(){
        return Some(char::from_digit(8, 10).unwrap());
    } 
    re = Regex::new(r"^nine*").unwrap();
    re_reverse = Regex::new(r"^enin.*").unwrap();
    if re.captures(chunk).is_some() || re_reverse.captures(chunk).is_some(){
        println!("found nine");
        return Some(char::from_digit(9, 10).unwrap());
    } 
    None
}
