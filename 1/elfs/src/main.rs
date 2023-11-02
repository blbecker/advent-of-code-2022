use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod elf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Parsing input file: {}", filename);
    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    let mut this_elf = elf::Elf::new();
    let mut elves = Vec::<elf::Elf>::new();
    let mut num_elves = 0;
    for (.., line) in reader.lines().enumerate() {
        lazy_static! {
            static ref CALORIE_RE: Regex = Regex::new(r"^\d+$").unwrap();
            static ref WHITESPACE_RE: Regex = Regex::new(r"^[[:space:]]*$").unwrap();
        }

        let line = line.unwrap();
        if CALORIE_RE.is_match(&line) {
            let calories = line.parse::<u64>();
            this_elf.add_food(elf::FoodItem::new(calories.unwrap()));
        } else if WHITESPACE_RE.is_match(&line) {
            // Whitepsace line is next elf
            elves.push(this_elf);
            this_elf = elf::Elf::new();
            num_elves += 1;
        }
    }

    let mut max_score = 0;
    let mut second_most = 0;
    let mut third_most = 0;
    let mut this_score = 0;
    for (.., elf) in elves.into_iter().enumerate() {
        this_score = elf.total_calories();

        if this_score > max_score {
            third_most = second_most;
            second_most = max_score;
            max_score = this_score;
        } else if this_score > second_most {
            third_most = second_most;
            second_most = this_score;
        } else if this_score > third_most {
            third_most = this_score;
        }

        this_score = 0;
    }
    println!("Processed {} elves.", num_elves);
    println!("Most Calories: {}", max_score);
    println!("Second most: {}", second_most);
    println!("Third most: {}", third_most);
    println!("Top 3 sum: {}", max_score + second_most + third_most);
}
